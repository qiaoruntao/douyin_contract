# douyin_contract

[中文](README.md) | English

The core of this project is a pipeline:

```
Douyin's shipped frontend JS  --(tools/extract_proto_from_webpack_chunk.py)-->  .proto + mapping.json
```

Consumers (e.g. `douyin_live_fetcher`) depend on the generated proto (via the
Rust crate this repo also builds) so they can focus on fetching/decoding
logic instead of schema reverse-engineering and upkeep.

## Why this exists

Douyin's live-room wss protocol (chat, gifts, fansclub, rankings, red
packets, ...) is undocumented. The schema isn't published anywhere, but it
*is* fully present in the frontend JS that live.douyin.com ships to every
browser — as a protobuf.js-compiled "static module" (each message type's
`decode()` function hand-encodes its own field-number → field-name → type
table). That's enough to mechanically reconstruct a real `.proto`, and
because it comes straight from the production bundle, it stays accurate as
Douyin evolves the protocol — just re-run the script against a fresh chunk.

## The pipeline

`tools/extract_proto_from_webpack_chunk.py` takes one input (the JS chunk)
and produces two outputs (a `.proto` and a `mapping.json`):

1. **Locate the namespace/type tree.** Walks the source with a
   string-aware brace matcher (so braces inside string/template literals
   don't desync depth) to recover the `Webcast.Im.*` / `Webcast.Data.*`
   containment tree — `Im` holds every message type that's actually
   dispatched over the wire; `Data` holds shared structs (`User`, `Image`,
   etc.) referenced from `Im` types.
2. **Extract each type's own fields.** Each message's `decode()` function
   builds an inline field table, e.g.
   `a={1:["nickname",e.string,0],2:["avatar_thumb",r.webcast.data.Image.decode,1],...}`
   (field number → [JS field name, decoder function reference, repeated
   flag]). The table is located via a brace-depth-aware scan, not a single
   regex — a regex that assumes one level of bracket nesting silently
   truncates on large tables (e.g. `User`'s real ~70-field table has entries
   the regex's nesting assumption didn't cover). Nested child types' own
   field tables are masked out first, otherwise a type with large nested
   children can end up emitted with a *child's* fields instead of its own
   (found and fixed during this project's first real test run — see
   `User` vs `User.PayGrade` in git history if curious).
3. **Extract `map<K,V>` fields separately.** protobuf.js generates map
   fields via bespoke inline decode code *outside* the generic field table
   (a map entry is wire-encoded as a 2-field embedded message: field 1 =
   key, field 2 = value, decoded in a hand-written loop) — so they're
   otherwise invisible to step 2 entirely, not just mistyped. Detected via a
   separate pattern match on that inline-loop shape. Two textual shapes
   exist for the *same* underlying pattern: a single map field on a type
   becomes `if(N===_){...}`; two or more become a
   `switch(_){case N:...;break;case M:...;break;}` instead — both are
   matched (`PublicAreaCommon` and `MemberMessage` each have multiple map
   fields and use the switch form, which the first version of this script
   didn't recognize at all).
4. **Infer proto types.** Scalar types come from the decoder reference
   (`e.string`→`string`, `e.int64String`/`e.uint64String`→`int64`/`uint64`
   — protobuf.js renders 64-bit values as JS strings client-side since JS
   numbers can't hold them exactly, but the wire format is plain
   int64/uint64, unaffected). Message-type fields resolve from the
   decoder's dotted path (`r.webcast.data.Image.decode` → `Webcast.Data.Image`).
5. **Emit `mapping.json`.** Wire `Message.method` string → fully qualified
   proto type, e.g. `"WebcastChatMessage": "Webcast.Im.ChatMessage"`.
   Convention is `method == "Webcast" + TypeName` for the vast majority of
   types in `Webcast.Im`; a small hardcoded override table
   (`KNOWN_METHOD_OVERRIDES` in the script, 5 entries) covers the known
   exceptions — e.g. `LinkMicMethod` has no "Webcast" prefix at all, and
   `WebcastRoomNotifyMessage` maps to type `NotifyMessage`, not
   `RoomNotifyMessage`. These can't be derived from the JS chunk itself
   (it never embeds the literal wire method string, only type names) —
   sourced once from `Remember-the-past/douyin_proto`'s method-mapping doc
   and hardcoded.

### Usage

```
python3 tools/extract_proto_from_webpack_chunk.py <path-to-live-schema-im.js> <output.proto> [mapping.json]
```

Finding the input: the CDN host and hash serving this chunk rotate over
time (seen so far: `lf-webcast-platform.bytetos.com` and
`lf-douyin-pc-web.douyinstatic.com`, hash changes with each frontend
deploy) — open live.douyin.com, find the `live-schema-im.<hash>.js` chunk in
the loaded network requests, and download that URL directly. Don't hardcode
a URL; always re-discover it.

### Known limitations

- **Map key/value types are inferred from runtime decoder calls, not
  declared types** — works for every case observed so far (all int32 keys,
  scalar or message values), but an unusual key type would need a small
  extension to `MAP_FIELD_RE`/`find_map_fields`.
- **`mapping.json`'s override list is a fixed table**, not derived from the
  JS. If Douyin introduces a new method name that doesn't follow the
  `"Webcast" + TypeName` convention, it needs to be added by hand (the
  symptom would be a `Message.method` string with no entry in
  `mapping.json`).
- Output is necessarily a snapshot of whatever JS chunk was fed in. Re-run
  against a fresh chunk periodically; the script itself has no notion of
  "current" beyond its input file.

## Layout

- `tools/extract_proto_from_webpack_chunk.py` — the generator (see above).
- `examples/verify_mongo_roundtrip.rs` — the verification harness (see
  above). Dev-only dependencies (`prost-reflect`, `mongodb`, `tokio`, etc.)
  live under `[dev-dependencies]` in `Cargo.toml`, not pulled in by
  consumers of this crate.
- `assets/douyin.proto` — compiled at build time via `prost-build` + `protox`
  (see `build.rs`) into this crate's Rust types. Two parts: a small
  hand-written envelope section (`PushFrame`, `HeadersList`, `Response`,
  `Message` — the outer wss frame, *not* covered by `live-schema-im.js`,
  which only describes `Message.payload` contents) followed by the fully
  generated `Webcast.Im.*` / `Webcast.Data.*` schema.
- `mapping.json` (repo root) — generator output, paired with `assets/douyin.proto`.

The prior, years-old hand-curated proto2 schema (rust-protobuf-generated,
flat `Im`-prefixed message names) and its method-name mapping have been
removed now that the generated schema replaces them — still available in
git history if ever needed.

## Toolchain

`prost` + `prost-build` + `protox`, proto3, matching `douyin_live_fetcher`'s
existing setup so both projects share one protobuf runtime.
