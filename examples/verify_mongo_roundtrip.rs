//! Verifies the generated `assets/douyin.proto` against real captured wss
//! messages: for every document in douyin_live_fetcher's local
//! `douyin_live.messages` collection, decode `payload` using the message
//! type `mapping.json` says corresponds to that document's `method`.
//!
//! The actual signal for "does our proto recognize every field present in
//! real traffic" is `DynamicMessage::unknown_fields()` -- prost-reflect
//! explicitly preserves (and would re-emit, if asked) any field number in
//! the wire data that the message descriptor doesn't declare. Any
//! non-empty `unknown_fields()` result, at any nesting depth, is a
//! concrete proto gap: a field number real Douyin traffic uses that
//! `assets/douyin.proto` doesn't have a definition for.
//!
//! (An earlier version of this script compared re-encoded byte length
//! against the original instead, on the theory that a missing field
//! declaration would silently drop data and shrink the output. That's
//! wrong: unknown fields are preserved, not dropped, so length never
//! changes because of them. The actual, large, very consistent length
//! diffs that approach found come from proto3's encode-side convention of
//! omitting fields that are at their default value (e.g. an explicit
//! `false`/`0` on the wire decodes correctly but isn't re-emitted) --
//! semantically lossless, not a proto gap. `unknown_fields()` sidesteps
//! that confound entirely.)
//!
//! Run: `cargo run --example verify_mongo_roundtrip` (reads
//! `MONGO_URI`/`MONGO_DB`/`MONGO_COLLECTION` env vars, defaulting to the
//! local douyin_live_fetcher dev setup).

use std::collections::BTreeMap;

use mongodb::bson::{doc, Binary};
use mongodb::{Client, Collection};
use prost_reflect::{DescriptorPool, DynamicMessage, Value};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct StoredMessage {
    method: String,
    msg_id: i64,
    payload: Binary,
}

#[derive(Default)]
struct MethodStats {
    total: usize,
    unmapped_method: usize,
    type_not_found: usize,
    decode_errors: usize,
    clean: usize,
    /// (msg_id, field path like "ranks[3].user.fans_club", unknown field numbers at that path)
    unknown_field_hits: Vec<(i64, String, Vec<u32>)>,
}

/// Recursively walk a decoded message, collecting any unknown field numbers
/// found at this level or in any nested message/repeated-message/map-message
/// field, with a dotted path for reporting.
fn collect_unknown_fields(msg: &DynamicMessage, path: &str, out: &mut Vec<(String, Vec<u32>)>) {
    let nums: Vec<u32> = msg.unknown_fields().map(|f| f.number()).collect();
    if !nums.is_empty() {
        out.push((path.to_string(), nums));
    }
    for (field_desc, value) in msg.fields() {
        let name = field_desc.name();
        match value {
            Value::Message(nested) => {
                collect_unknown_fields(nested, &format!("{path}.{name}"), out);
            }
            Value::List(items) => {
                for (i, item) in items.iter().enumerate() {
                    if let Value::Message(nested) = item {
                        collect_unknown_fields(nested, &format!("{path}.{name}[{i}]"), out);
                    }
                }
            }
            Value::Map(map) => {
                for (k, v) in map.iter() {
                    if let Value::Message(nested) = v {
                        collect_unknown_fields(nested, &format!("{path}.{name}[{k:?}]"), out);
                    }
                }
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mongo_uri =
        std::env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://127.0.0.1:27017".to_string());
    let db_name = std::env::var("MONGO_DB").unwrap_or_else(|_| "douyin_live".to_string());
    let coll_name = std::env::var("MONGO_COLLECTION").unwrap_or_else(|_| "messages".to_string());

    let mapping: BTreeMap<String, String> =
        serde_json::from_str(&std::fs::read_to_string("mapping.json")?)?;

    let file_descriptors = protox::compile(["assets/douyin.proto"], ["assets"])?;
    let pool = DescriptorPool::from_file_descriptor_set(file_descriptors)?;

    let client = Client::with_uri_str(&mongo_uri).await?;
    let coll: Collection<StoredMessage> = client.database(&db_name).collection(&coll_name);
    let mut cursor = coll.find(doc! {}).await?;

    let mut stats: BTreeMap<String, MethodStats> = BTreeMap::new();
    let mut total_docs = 0usize;

    use futures_util::TryStreamExt;
    while let Some(msg) = cursor.try_next().await? {
        total_docs += 1;
        let entry = stats.entry(msg.method.clone()).or_default();
        entry.total += 1;

        let Some(type_path) = mapping.get(&msg.method) else {
            entry.unmapped_method += 1;
            continue;
        };
        let full_name = format!("douyin.{type_path}");
        let Some(descriptor) = pool.get_message_by_name(&full_name) else {
            entry.type_not_found += 1;
            continue;
        };

        let decoded = match DynamicMessage::decode(descriptor, msg.payload.bytes.as_slice()) {
            Ok(d) => d,
            Err(_) => {
                entry.decode_errors += 1;
                continue;
            }
        };

        let mut hits = Vec::new();
        collect_unknown_fields(&decoded, type_path, &mut hits);
        if hits.is_empty() {
            entry.clean += 1;
        } else {
            for (path, nums) in hits {
                entry.unknown_field_hits.push((msg.msg_id, path, nums));
            }
        }
    }

    println!("=== douyin_contract proto verification ===");
    println!("mongo: {mongo_uri} db={db_name} collection={coll_name}");
    println!("total documents scanned: {total_docs}");
    println!();

    let mut any_real_issues = false;
    for (method, s) in &stats {
        println!(
            "{method}: total={} clean={}{}{}{}",
            s.total,
            s.clean,
            opt_field("unmapped_method", s.unmapped_method),
            opt_field("type_not_found", s.type_not_found),
            opt_field("decode_errors", s.decode_errors),
        );
        if s.unmapped_method > 0 || s.type_not_found > 0 || s.decode_errors > 0 {
            any_real_issues = true;
        }
        if !s.unknown_field_hits.is_empty() {
            any_real_issues = true;
            // Aggregate by (path, field number) so one undeclared field
            // doesn't print one line per message -- show count + one example.
            let mut by_path_field: BTreeMap<(String, u32), (usize, i64)> = BTreeMap::new();
            for (msg_id, path, nums) in &s.unknown_field_hits {
                for &n in nums {
                    let key = (path.clone(), n);
                    let e = by_path_field.entry(key).or_insert((0, *msg_id));
                    e.0 += 1;
                }
            }
            println!("  *** UNDECLARED FIELDS (present in real traffic, missing from assets/douyin.proto) ***");
            for ((path, field_num), (count, example_msg_id)) in &by_path_field {
                println!(
                    "    {path}: field #{field_num} undeclared -- seen in {count} message(s), e.g. msg_id={example_msg_id}"
                );
            }
        }
    }

    println!();
    if any_real_issues {
        println!(
            "RESULT: issues found -- see above. unmapped_method/type_not_found/decode_errors are hard \
             failures (a method string or type couldn't be resolved/decoded at all). UNDECLARED FIELDS \
             entries are real, specific proto gaps: that exact field number is used in production but has \
             no definition in assets/douyin.proto for that message type -- add it by re-running the \
             extractor against the current JS chunk, or by hand if it's a one-off."
        );
    } else {
        println!("RESULT: clean -- every decoded message's field set (recursively) is fully declared in assets/douyin.proto.");
    }

    Ok(())
}

fn opt_field(label: &str, n: usize) -> String {
    if n > 0 {
        format!(" {label}={n}")
    } else {
        String::new()
    }
}
