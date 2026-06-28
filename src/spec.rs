//! OpenAPI spec parser.
//!
//! The committed spec (`spec/drata-openapi-v2.json`) is embedded via
//! `include_str!` so the parser and coverage test work regardless of the
//! process's working directory. The spec anchors two things:
//!
//! - **`--example` skeletons** generated from an operation's request schema
//!   (`example_for_operation`). This generalizes the hand-written vendors
//!   skeleton from Phase 1 to any of the 167 operations.
//! - **Coverage**: `operations()` enumerates every operation (method + path +
//!   operationId + tags), sourcing tags from the *operation-level* `tags`
//!   array, NOT the 33-entry top-level `tags` list (which omits
//!   `Audit Requests` and `Procurement Connection Mappings`).
//!
//! `serde_json::Value` is the wire currency here too: the spec is parsed once
//! into a `Value` and walked, rather than modeled as 434 typed structs.
//!
//! ### `--example` fidelity
//!
//! Skeletons are a *minimal-plus-examples* stub, not a deep materialization:
//! every `required` property is emitted, plus any property that carries its own
//! `example`/`default` (so the common optional fields are discoverable). `$ref`
//! and `allOf` are resolved (to pull in enum values and merged object shapes);
//! `oneOf`/`anyOf` take the first variant. Nested objects/arrays recurse to a
//! bounded depth (`MAX_DEPTH`) to stay cycle-safe and keep the skeleton legible.

use eyre::{Context, Result, eyre};
use serde_json::{Map, Value, json};
use std::sync::OnceLock;
use tracing::{debug, instrument, trace};

/// The committed OpenAPI spec, embedded at compile time so spec-driven features
/// (examples, coverage) never depend on the runtime working directory.
const SPEC_JSON: &str = include_str!("../spec/drata-openapi-v2.json");

/// Bound on `$ref`/`allOf`/nested-object recursion when building a skeleton.
/// Deep enough for Drata's real request bodies, shallow enough to stay legible
/// and cycle-safe.
const MAX_DEPTH: u32 = 6;

/// A single OpenAPI operation: one HTTP method on one path template.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Operation {
    /// Upper-cased HTTP method (`GET`, `POST`, `PUT`, `DELETE`).
    pub method: String,
    /// Path template as written in the spec, e.g. `/vendors/{id}`.
    pub path: String,
    /// The spec's `operationId` for this operation.
    pub operation_id: String,
    /// Operation-level tags (the authoritative tag set for coverage).
    pub tags: Vec<String>,
    /// True if the request body is `multipart/form-data` (the upload ops).
    pub multipart: bool,
}

/// Parse and cache the embedded spec as a `serde_json::Value`. Parsed once;
/// subsequent calls return the cached reference.
fn spec() -> Result<&'static Value> {
    static SPEC: OnceLock<Value> = OnceLock::new();
    if let Some(v) = SPEC.get() {
        return Ok(v);
    }
    let parsed: Value = serde_json::from_str(SPEC_JSON).context("Failed to parse embedded OpenAPI spec")?;
    // OnceLock::set fails only if another thread won the race; either way the
    // value is now present, so re-read it.
    let _ = SPEC.set(parsed);
    SPEC.get()
        .ok_or_else(|| eyre!("spec OnceLock unexpectedly empty after set"))
}

/// HTTP method keys we treat as operations. Drata's spec uses only these four,
/// but the filter keeps non-operation path keys (`parameters`, `summary`) out.
const HTTP_METHODS: &[&str] = &["get", "post", "put", "patch", "delete", "head", "options"];

/// Enumerate every operation in the spec: method + path + operationId + tags.
///
/// Tags come from each operation's own `tags` array (35 distinct values), not
/// the top-level `tags` list (33). This is the authoritative set the coverage
/// test asserts against.
#[instrument(skip_all)]
pub fn operations() -> Result<Vec<Operation>> {
    let spec = spec()?;
    let paths = spec
        .get("paths")
        .and_then(Value::as_object)
        .ok_or_else(|| eyre!("spec has no `paths` object"))?;

    let mut ops = Vec::new();
    for (path, item) in paths {
        let Some(methods) = item.as_object() else {
            continue;
        };
        for (method, op) in methods {
            if !HTTP_METHODS.contains(&method.to_lowercase().as_str()) {
                continue;
            }
            let operation_id = op.get("operationId").and_then(Value::as_str).unwrap_or("").to_string();
            let tags = op
                .get("tags")
                .and_then(Value::as_array)
                .map(|arr| arr.iter().filter_map(Value::as_str).map(String::from).collect())
                .unwrap_or_default();
            let multipart = op
                .get("requestBody")
                .and_then(|rb| rb.get("content"))
                .and_then(Value::as_object)
                .map(|c| c.contains_key("multipart/form-data"))
                .unwrap_or(false);
            // Per-op iteration is the hot loop -> TRACE, not DEBUG.
            trace!(%method, %path, %operation_id, "enumerated operation");
            ops.push(Operation {
                method: method.to_uppercase(),
                path: path.clone(),
                operation_id,
                tags,
                multipart,
            });
        }
    }
    debug!(count = ops.len(), "enumerated operations");
    Ok(ops)
}

/// The distinct operation-level tag set (sorted, deduped). Sourced from the
/// operations themselves, so `Audit Requests` and
/// `Procurement Connection Mappings` are included.
#[instrument(skip_all)]
pub fn operation_tags() -> Result<Vec<String>> {
    let mut tags: Vec<String> = operations()?.into_iter().flat_map(|op| op.tags).collect();
    tags.sort();
    tags.dedup();
    debug!(count = tags.len(), "collected operation-level tags");
    Ok(tags)
}

/// Find an operation by HTTP method + path template (case-insensitive method).
#[instrument(skip_all, fields(%method, %path))]
pub fn find_by_method_path(method: &str, path: &str) -> Result<Option<Operation>> {
    let want = method.to_uppercase();
    let found = operations()?
        .into_iter()
        .find(|op| op.method == want && op.path == path);
    debug!(found = found.is_some(), "lookup by method+path");
    Ok(found)
}

/// Generate a `--example` request-body skeleton for an operation identified by
/// HTTP method + path template. Returns `None` when the operation has no
/// JSON request body (e.g. a GET or a multipart upload).
#[instrument(skip_all, fields(%method, %path))]
pub fn example_for_operation(method: &str, path: &str) -> Result<Option<String>> {
    let spec = spec()?;
    let schema = match request_schema(spec, method, path)? {
        Some(s) => s,
        None => {
            debug!("no JSON request schema for operation");
            return Ok(None);
        }
    };
    let skeleton = build_skeleton(spec, schema, 0);
    let pretty = serde_json::to_string_pretty(&skeleton).context("Failed to render example skeleton")?;
    debug!(bytes = pretty.len(), "generated example skeleton");
    Ok(Some(format!("{}\n", pretty)))
}

/// Resolve the `application/json` request-body schema for an operation, if any.
fn request_schema<'a>(spec: &'a Value, method: &str, path: &str) -> Result<Option<&'a Value>> {
    let want = method.to_lowercase();
    let op = spec
        .get("paths")
        .and_then(|p| p.get(path))
        .and_then(|item| item.get(&want));
    let Some(op) = op else {
        return Ok(None);
    };
    let schema = op
        .get("requestBody")
        .and_then(|rb| rb.get("content"))
        .and_then(|c| c.get("application/json"))
        .and_then(|j| j.get("schema"));
    Ok(schema)
}

/// Build a skeleton value from a schema node, resolving `$ref`/`allOf` and
/// recursing into objects/arrays up to `MAX_DEPTH`.
fn build_skeleton(spec: &Value, schema: &Value, depth: u32) -> Value {
    if depth > MAX_DEPTH {
        return Value::Null;
    }

    // A direct example/default short-circuits everything below it.
    if let Some(example) = schema.get("example") {
        return example.clone();
    }
    if let Some(default) = schema.get("default") {
        return default.clone();
    }

    // Resolve a `$ref` to its target schema and recurse.
    if let Some(target) = resolve_ref(spec, schema) {
        return build_skeleton(spec, target, depth + 1);
    }

    // `allOf`: merge member object schemas; a lone member just recurses (this is
    // how the spec wraps an enum: `allOf: [ { $ref: SomeEnum } ]`).
    if let Some(members) = schema.get("allOf").and_then(Value::as_array) {
        return build_allof(spec, members, depth);
    }

    // `oneOf`/`anyOf`: pick the first variant.
    for key in ["oneOf", "anyOf"] {
        if let Some(first) = schema.get(key).and_then(Value::as_array).and_then(|a| a.first()) {
            return build_skeleton(spec, first, depth + 1);
        }
    }

    // An inline enum: use the first allowed value.
    if let Some(first) = schema.get("enum").and_then(Value::as_array).and_then(|a| a.first()) {
        return first.clone();
    }

    match schema.get("type").and_then(Value::as_str) {
        Some("object") => build_object(spec, schema, depth),
        Some("array") => build_array(spec, schema, depth),
        Some("string") => json!("string"),
        Some("integer") | Some("number") => json!(0),
        Some("boolean") => json!(false),
        // No `type` but has `properties` -> still an object.
        _ if schema.get("properties").is_some() => build_object(spec, schema, depth),
        _ => Value::Null,
    }
}

/// Merge `allOf` members into one skeleton. Object members merge their fields;
/// a single non-object member (e.g. an enum `$ref`) just resolves through.
fn build_allof(spec: &Value, members: &[Value], depth: u32) -> Value {
    let mut merged = Map::new();
    let mut scalar: Option<Value> = None;
    for member in members {
        let built = build_skeleton(spec, member, depth + 1);
        match built {
            Value::Object(map) => merged.extend(map),
            other => scalar = Some(other),
        }
    }
    if merged.is_empty() {
        scalar.unwrap_or(Value::Null)
    } else {
        Value::Object(merged)
    }
}

/// Build an object skeleton: emit every `required` property, plus any property
/// that advertises its own `example`/`default` (so common optionals surface).
fn build_object(spec: &Value, schema: &Value, depth: u32) -> Value {
    let required: Vec<&str> = schema
        .get("required")
        .and_then(Value::as_array)
        .map(|arr| arr.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();

    let mut out = Map::new();
    if let Some(props) = schema.get("properties").and_then(Value::as_object) {
        for (name, prop) in props {
            let is_required = required.contains(&name.as_str());
            let has_hint = prop.get("example").is_some() || prop.get("default").is_some();
            if is_required || has_hint {
                out.insert(name.clone(), build_skeleton(spec, prop, depth + 1));
            }
        }
    }
    Value::Object(out)
}

/// Build a one-element array skeleton from the `items` schema.
fn build_array(spec: &Value, schema: &Value, depth: u32) -> Value {
    match schema.get("items") {
        Some(items) => json!([build_skeleton(spec, items, depth + 1)]),
        None => json!([]),
    }
}

/// If `schema` is a `{ "$ref": "#/components/schemas/Foo" }`, return the target.
fn resolve_ref<'a>(spec: &'a Value, schema: &Value) -> Option<&'a Value> {
    let reference = schema.get("$ref").and_then(Value::as_str)?;
    // Only local component refs appear in this spec.
    let pointer = reference.strip_prefix('#')?;
    spec.pointer(pointer)
}

#[cfg(test)]
mod tests;
