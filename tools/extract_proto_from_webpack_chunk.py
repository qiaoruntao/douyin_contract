#!/usr/bin/env python3
"""Reverse-engineer a .proto from a protobufjs static-module webpack chunk.

Source pattern (per message type, minified but structurally regular):

    e.TypeName = function () {
        function e(e) { ... }                      // constructor
        return s(e, {field: default, ...}), e.decode = function (e, t) {
            ...
            var X = { 1: ["field_name", decoderRef, flags], 2: [...] };
            ...
        }, e;
    }();

Namespace pattern:

    e.nsname = function () {
        let e = {};
        return e.Type1 = ..., e.Type2 = ..., e;
    }();

This script walks the source with a string-aware brace matcher (so braces
inside string/template literals don't desync depth), builds a containment
tree of namespace/type spans, then regexes inside each isolated type span
for its decode-time field descriptor table and its default-value object
(from the `s(e, {...})` call) to recover field name/number/type/repeated.
"""
import re
import sys
import json

def find_matching_brace(s, open_idx):
    """open_idx points at a '{'. Return index of matching '}', string-aware."""
    assert s[open_idx] == '{'
    depth = 0
    i = open_idx
    n = len(s)
    in_str = None  # None, "'", '"', or '`'
    while i < n:
        c = s[i]
        if in_str:
            if c == '\\':
                i += 2
                continue
            if c == in_str:
                in_str = None
            i += 1
            continue
        if c in ("'", '"', '`'):
            in_str = c
            i += 1
            continue
        if c == '{':
            depth += 1
        elif c == '}':
            depth -= 1
            if depth == 0:
                return i
        i += 1
    raise ValueError(f"no matching brace from {open_idx}")

NS_OPEN_RE = re.compile(r'\.([A-Za-z_][A-Za-z0-9_]*)\s*=\s*function\s*\(\)\s*\{\s*let \w+\s*=\s*\{\}\s*;')
TYPE_OPEN_RE = re.compile(r'\.([A-Za-z_][A-Za-z0-9_]*)\s*=\s*function\s*\(\)\s*\{\s*(?:let \w+;\s*)?function\s+\w+\(\w+\)\s*\{')

def parse_span(s, start, end, path):
    """Find direct-child namespace/type defs within s[start:end], recurse."""
    nodes = []
    i = start
    while True:
        ns_m = NS_OPEN_RE.search(s, i, end)
        ty_m = TYPE_OPEN_RE.search(s, i, end)
        cands = [m for m in (ns_m, ty_m) if m]
        if not cands:
            break
        m = min(cands, key=lambda mm: mm.start())
        name = m.group(1)
        # find the '{' that starts the function body: it's the '{' right after "function()"
        body_open = s.index('{', m.end() - 1)
        # NS_OPEN_RE/TYPE_OPEN_RE already consumed up to/just past the first '{',
        # so back up: locate the actual function-body brace precisely.
        # Simplest: the function body opening brace is the one matched inside the regex
        # for TYPE (group ends right after "function e(e){" so we need the OUTER
        # function(){ brace, which is earlier). Recompute robustly:
        fn_kw = s.rfind('function()', m.start(), m.end())
        body_open = s.index('{', fn_kw)
        body_close = find_matching_brace(s, body_open)
        child_path = path + [name]
        is_type = m is ty_m
        if is_type:
            nodes.append({
                "kind": "type",
                "name": name,
                "path": child_path,
                "span": (body_open, body_close),
            })
            # types can have nested types too in some schemas (rare) -- recurse anyway
            nodes[-1]["children"] = parse_span(s, body_open, body_close, child_path)
        else:
            children = parse_span(s, body_open, body_close, child_path)
            nodes.append({
                "kind": "namespace",
                "name": name,
                "path": child_path,
                "span": (body_open, body_close),
                "children": children,
            })
        i = body_close + 1
    return nodes

FIELD_TABLE_RE = re.compile(r'(?<![A-Za-z0-9_])([A-Za-z_$][A-Za-z0-9_$]*)\s*=\s*\{((?:\d+\s*:\s*\[[^\[\]]*(?:\[[^\]]*\][^\[\]]*)*\]\s*,?\s*)+)\}')
ENTRY_RE = re.compile(r'(\d+)\s*:\s*\[\s*"([^"]+)"\s*,\s*([^,]+?)\s*,\s*(\d+)\s*\]')

DEFAULTS_CALL_RE = re.compile(r'\bs\(\w+\s*,\s*\{')

def extract_defaults(body):
    m = DEFAULTS_CALL_RE.search(body)
    if not m:
        return {}
    open_idx = body.index('{', m.start())
    close_idx = find_matching_brace(body, open_idx)
    obj_src = body[open_idx + 1:close_idx]
    defaults = {}
    # split on top-level commas (depth-aware, string-aware)
    depth = 0
    in_str = None
    cur = []
    parts = []
    i = 0
    while i < len(obj_src):
        c = obj_src[i]
        if in_str:
            if c == '\\':
                cur.append(c)
                cur.append(obj_src[i+1] if i+1 < len(obj_src) else '')
                i += 2
                continue
            cur.append(c)
            if c == in_str:
                in_str = None
            i += 1
            continue
        if c in ("'", '"'):
            in_str = c
            cur.append(c)
            i += 1
            continue
        if c in '{[(':
            depth += 1
        elif c in '}])':
            depth -= 1
        if c == ',' and depth == 0:
            parts.append(''.join(cur))
            cur = []
            i += 1
            continue
        cur.append(c)
        i += 1
    if cur:
        parts.append(''.join(cur))
    for part in parts:
        part = part.strip()
        if not part or ':' not in part:
            continue
        key, _, val = part.partition(':')
        defaults[key.strip().strip('"\'')] = val.strip()
    return defaults

TABLE_START_RE = re.compile(r'(?<![A-Za-z0-9_$])[A-Za-z_$][A-Za-z0-9_$]*\s*=\s*\{(?=\s*\d+\s*:\s*\[)')

def find_field_table(body):
    """Find the field-descriptor object literal inside a decode() function.

    Locates candidate object-literal starts with a brace-depth-aware matcher
    (handles arbitrarily large/nested tables, unlike a single big regex which
    breaks on huge tables -- e.g. User's 100+-field table -- where some entry's
    shape doesn't fit a rigid one-level-nesting assumption). Once the full
    balanced {...} text is isolated, entries are extracted with a loose
    findall that doesn't require matching the entire object, just individual
    `N:["name", decoderRef, flags]` entries wherever they appear inside it.
    """
    best = None
    for m in TABLE_START_RE.finditer(body):
        open_idx = body.index('{', m.start())
        try:
            close_idx = find_matching_brace(body, open_idx)
        except ValueError:
            continue
        obj_src = body[open_idx + 1:close_idx]
        entries = ENTRY_RE.findall(obj_src)
        if not entries:
            continue
        if best is None or len(entries) > len(best):
            best = entries
    if not best:
        return []
    entries = best
    out = []
    for num, fname, decoder, flags in entries:
        out.append({
            "number": int(num),
            "name": fname,
            "decoder": decoder.strip(),
            "flags": int(flags),
        })
    return out

# protobuf.js generates `map<K, V>` fields as bespoke inline decode code
# OUTSIDE the generic field-table dispatch (`d[_]`) that find_field_table
# reads -- so map fields are otherwise invisible to this script entirely
# (silently dropped, not just mistyped). Pattern, from a real example
# (Webcast.Data.User.FansClub.FansClubData.UserBadge's `icons` field):
#
#   if(1===_){u.icons===a.emptyObject&&(u.icons={});var f=e.uint32()+e.pos;
#   for(n=0,i=null;e.pos<f;){var p=e.uint32();switch(p>>>3){
#     case 1:n=e.int32();break;
#     case 2:i=r.webcast.data.Image.decode(e,e.uint32());break;
#     default:e.skipType(7&p)}}u.icons[n]=i}
#
# i.e. a map entry is wire-encoded as a 2-field embedded message (field 1 =
# key, field 2 = value) per the protobuf map spec -- protobuf.js just inlines
# that decode instead of routing through the generic table.
MAP_FIELD_RE = re.compile(
    r'(?:if\(\d+===\w+\)|case \d+:)\{?\w+\.(\w+)===\w+\.emptyObject&&\(\w+\.\1=\{\}\);'
    r'var \w+=\w+\.uint32\(\)\+\w+\.pos;'
    r'for\([^)]*\)\{var \w+=\w+\.uint32\(\);switch\(\w+>>>3\)\{'
    r'case 1:\w+=([^;]+);break;'
    r'case 2:\w+=([^;]+);break;'
)
# Two shapes protobuf.js emits for this special-cased dispatch: a single
# map field on a type becomes `if(N===_){...}`; two or more become a
# `switch(_){case N:...;break;case M:...;break;}` instead (no braces around
# each case body). Field-number capture mirrors both.
MAP_FIELD_NUM_RE = re.compile(
    r'(?:if\((\d+)===\w+\)\{|case (\d+):)(\w+)\.(\w+)===\w+\.emptyObject'
)

def find_map_fields(body):
    """Find map<K,V> fields, which find_field_table can't see at all (see
    MAP_FIELD_RE comment). Returns a list of dicts with number/name/key
    decoder/value decoder, in the same number-space as regular fields."""
    out = []
    for m in MAP_FIELD_RE.finditer(body):
        num_m = MAP_FIELD_NUM_RE.match(body, m.start())
        if not num_m:
            continue
        field_num = num_m.group(1) if num_m.group(1) is not None else num_m.group(2)
        out.append({
            "number": int(field_num),
            "name": num_m.group(4),
            # these are call expressions, e.g. "e.int32()" or
            # "r.webcast.data.Image.decode(e,e.uint32())" -- strip the
            # trailing call parens so infer_scalar_proto_type /
            # resolve_message_ref (which expect a bare reference, as used
            # in the generic field table) work unmodified.
            "key_decoder": re.sub(r'\(.*\)$', '', m.group(2).strip()),
            "value_decoder": re.sub(r'\(.*\)$', '', m.group(3).strip()),
        })
    return out

def infer_scalar_proto_type(decoder, default_val):
    decoder = decoder.strip()
    # primitive reader method refs, e.g. t.string / e.bool / a.int64 etc.
    tail = decoder.rsplit('.', 1)[-1]
    table = {
        "string": "string",
        "bool": "bool",
        "int32": "int32",
        "uint32": "uint32",
        "sint32": "sint32",
        "fixed32": "fixed32",
        "sfixed32": "sfixed32",
        "float": "float",
        "double": "double",
        "bytes": "bytes",
        "int64": "int64",
        "uint64": "uint64",
        "sint64": "sint64",
        "fixed64": "fixed64",
        "sfixed64": "sfixed64",
        # protobuf.js's JS-side convenience readers for 64-bit values that
        # can't be represented exactly as JS numbers -- they still decode
        # the same int64/uint64 varint wire format, just expose it as a
        # string client-side. Wire format is unaffected.
        "int64String": "int64",
        "uint64String": "uint64",
    }
    if tail in table:
        return table[tail]
    return None  # not a recognized primitive reader -> likely message/enum decode ref

def resolve_message_ref(decoder):
    """decoder like 'r.webcast.im.Common.decode' -> dotted type path 'webcast.im.Common'."""
    decoder = decoder.strip()
    if decoder.endswith('.decode'):
        dotted = decoder[:-len('.decode')]
        parts = dotted.split('.')
        # drop the leading root var (e.g. 'r')
        if len(parts) > 1:
            return '.'.join(parts[1:])
    return None

def to_proto_name(path):
    # path is list of namespace/type names from root, e.g. ['webcast','im','ChatMessage']
    return '.'.join(p[0].upper() + p[1:] if p else p for p in path)

def collect_types(nodes, out):
    for n in nodes:
        if n["kind"] == "type":
            out.append(n)
        collect_types(n.get("children", []), out)

def build_proto_decl_name(name):
    return name[0].upper() + name[1:] if name else name

def find_node(nodes, path):
    for n in nodes:
        if n["path"] == path:
            return n
        r = find_node(n.get("children", []), path)
        if r:
            return r
    return None

# A small number of wire `method` strings don't follow the "Webcast" + TypeName
# convention (e.g. method "LinkMicMethod" has no "Webcast" prefix at all, and
# method "WebcastRoomNotifyMessage" maps to type "NotifyMessage", not
# "RoomNotifyMessage"). Sourced from Remember-the-past/douyin_proto's
# method-mapping doc (which marks these as the known irregular cases) --
# can't be derived from this JS chunk alone since it doesn't embed the wire
# method string anywhere, only type names.
KNOWN_METHOD_OVERRIDES = {
    "WebcastRoomNotifyMessage": "NotifyMessage",
    "WebcastLinkMicBattleMethod": "LinkMicBattle",
    "LinkMicMethod": "LinkMicMethod",
    "WebcastLinkMicBattleFinishMethod": "LinkMicBattleFinish",
    "WebcastDecorationModifyMethod": "DecorationModifyMessage",
}

def build_method_mapping(im_node):
    """method string (as seen in Message.method on the wire) -> fully
    qualified proto type name, for every type nested directly under
    Webcast.Im (the namespace holding all dispatched push message types).

    Convention: method == "Webcast" + TypeName for the vast majority of
    types (verified against 8 real captured message types in
    douyin_live_fetcher's local Mongo -- see docs in that repo). The known
    exceptions are applied as overrides.
    """
    mapping = {}
    type_names = {c["name"] for c in im_node["children"] if c["kind"] == "type"}
    override_targets = set(KNOWN_METHOD_OVERRIDES.values())
    for name in sorted(type_names):
        if name in override_targets:
            continue  # only reachable via its override method string below
        method = "Webcast" + name
        mapping[method] = f"Webcast.Im.{name}"
    for method, type_name in KNOWN_METHOD_OVERRIDES.items():
        if type_name in type_names:
            mapping[method] = f"Webcast.Im.{type_name}"
    return dict(sorted(mapping.items()))

def main():
    src_path = sys.argv[1]
    out_path = sys.argv[2]
    mapping_out_path = sys.argv[3] if len(sys.argv) > 3 else None
    s = open(src_path, encoding='utf-8').read()

    # Locate the root: r.webcast=(()=>{ ... })()
    root_m = re.search(r'\br\.webcast\s*=\s*\(\s*\(\)\s*=>\s*\{', s)
    if not root_m:
        print("root 'r.webcast=' not found", file=sys.stderr)
        sys.exit(1)
    root_body_open = s.index('{', root_m.end() - 1)
    root_body_close = find_matching_brace(s, root_body_open)
    root_children = parse_span(s, root_body_open, root_body_close, ["webcast"])

    all_types = []
    collect_types(root_children, all_types)
    print(f"found {len(all_types)} message types", file=sys.stderr)

    # Build full tree structure (namespaces + types) for proto emission
    type_by_path = {}
    for t in all_types:
        span_start, span_end = t["span"]
        body = s[span_start:span_end + 1]
        # Mask out nested child type/namespace spans before searching for
        # this type's own field table -- otherwise, for a type with nested
        # children that also have decode() field tables, find_field_table's
        # "most entries" heuristic can pick a CHILD's table instead of the
        # parent's own (e.g. User wrongly inheriting nested PayGrade's fields).
        masked = list(body)
        for child in t.get("children", []):
            c_start, c_end = child["span"]
            # span offsets are relative to the full source `s`; translate to body-local.
            lo = max(0, c_start - span_start)
            hi = min(len(masked), c_end - span_start + 1)
            for i in range(lo, hi):
                masked[i] = ' '
        masked_body = ''.join(masked)
        fields = find_field_table(masked_body)
        map_fields = find_map_fields(masked_body)
        defaults = extract_defaults(masked_body)
        type_by_path['.'.join(t["path"])] = {
            "fields": fields,
            "map_fields": map_fields,
            "defaults": defaults,
        }
        t["fields"] = fields
        t["map_fields"] = map_fields
        t["defaults"] = defaults

    def emit_node(node, indent):
        pad = '  ' * indent
        lines = []
        if node["kind"] == "namespace":
            # protobuf has no concept of plain "namespace" -- represent as a wrapper message,
            # matching the reference repo's convention (Webcast.Im.* style).
            lines.append(f'{pad}message {build_proto_decl_name(node["name"])} {{')
            lines.append("")
            for child in node["children"]:
                lines.extend(emit_node(child, indent + 1))
            lines.append(f'{pad}}}')
            lines.append("")
        else:
            lines.append(f'{pad}message {build_proto_decl_name(node["name"])} {{')
            fields = node.get("fields", [])
            defaults = node.get("defaults", {})
            seen_numbers = set()
            for f in sorted(fields, key=lambda x: x["number"]):
                if f["number"] in seen_numbers:
                    continue
                seen_numbers.add(f["number"])
                repeated = bool(f["flags"] & 0b010)
                scalar = infer_scalar_proto_type(f["decoder"], defaults.get(f["name"]))
                if scalar:
                    ptype = scalar
                else:
                    ref = resolve_message_ref(f["decoder"])
                    if ref:
                        parts = ref.split('.')
                        ptype = '.'.join(build_proto_decl_name(p) for p in parts)
                    else:
                        ptype = f'/* UNKNOWN decoder={f["decoder"]} */ bytes'
                prefix = 'repeated ' if repeated else ''
                lines.append(f'{pad}  {prefix}{ptype} {f["name"]} = {f["number"]};')
            for mf in sorted(node.get("map_fields", []), key=lambda x: x["number"]):
                if mf["number"] in seen_numbers:
                    continue
                seen_numbers.add(mf["number"])
                key_type = infer_scalar_proto_type(mf["key_decoder"], None) or 'int32'
                val_scalar = infer_scalar_proto_type(mf["value_decoder"], None)
                if val_scalar:
                    val_type = val_scalar
                else:
                    ref = resolve_message_ref(mf["value_decoder"])
                    val_type = '.'.join(build_proto_decl_name(p) for p in ref.split('.')) if ref \
                        else f'/* UNKNOWN decoder={mf["value_decoder"]} */ bytes'
                lines.append(f'{pad}  map<{key_type}, {val_type}> {mf["name"]} = {mf["number"]};')
            for child in node["children"]:
                lines.extend(emit_node(child, indent + 1))
            lines.append(f'{pad}}}')
            lines.append("")
        return lines

    out_lines = ['syntax = "proto3";', '', 'package douyin;', '', 'message Webcast {', '']
    for child in root_children:
        out_lines.extend(emit_node(child, 1))
    out_lines.append('}')

    with open(out_path, 'w', encoding='utf-8') as f:
        f.write('\n'.join(out_lines))
    print(f"wrote {out_path}", file=sys.stderr)

    if mapping_out_path:
        im_node = find_node(root_children, ["webcast", "im"])
        if im_node is None:
            print("warning: 'webcast.im' namespace not found, skipping mapping.json", file=sys.stderr)
        else:
            mapping = build_method_mapping(im_node)
            with open(mapping_out_path, 'w', encoding='utf-8') as f:
                json.dump(mapping, f, indent=2, ensure_ascii=False, sort_keys=True)
                f.write('\n')
            print(f"wrote {mapping_out_path} ({len(mapping)} methods)", file=sys.stderr)

if __name__ == '__main__':
    main()
