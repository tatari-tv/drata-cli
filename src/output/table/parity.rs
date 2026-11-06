//! Renderer parity with v0.1.5: every op the v0.1.5 tree curated must still
//! pick the table renderer (or the JSON fallback) that v0.1.5 picked.
//!
//! The expected renderer comes from `v015_pick_renderer`, a verbatim copy of
//! the v0.1.5 sniff order (`git show 165c6cf:src/output/table.rs`), applied to
//! a row built from each op's 2xx response schema in the committed spec. A new
//! sniff that claims an old shape fails here, not in a user's terminal.

#![allow(clippy::unwrap_used)]
use super::*;
use serde_json::Map;

/// The 83 ops the v0.1.5 tree reached with a curated command (the command
/// table rows that are not in the design doc's Appendix A).
const V015_CURATED: &[(&str, &str)] = &[
    ("DELETE", "/assets/{assetId}"),
    ("DELETE", "/risk-registers/{riskRegisterId}"),
    ("DELETE", "/vendors/{vendorId}"),
    (
        "DELETE",
        "/workspaces/{workspaceId}/evidence-library/{evidenceLibraryId}",
    ),
    ("GET", "/assets"),
    ("GET", "/assets/{assetId}"),
    ("GET", "/company"),
    ("GET", "/devices"),
    ("GET", "/devices/{deviceId}"),
    ("GET", "/devices/{deviceId}/apps"),
    ("GET", "/events"),
    ("GET", "/events/{eventId}"),
    ("GET", "/personnel"),
    ("GET", "/personnel/{personnelId}"),
    ("GET", "/personnel/{personnelId}/devices"),
    ("GET", "/policies"),
    ("GET", "/policies/{policyId}"),
    ("GET", "/policies/{policyId}/actions"),
    ("GET", "/policies/{policyId}/policy-versions"),
    ("GET", "/policies/{policyId}/policy-versions/{policyVersionId}"),
    ("GET", "/risk-registers"),
    ("GET", "/risk-registers/{riskRegisterId}"),
    ("GET", "/risk-registers/{riskRegisterId}/insights"),
    ("GET", "/risk-registers/{riskRegisterId}/risks"),
    ("GET", "/risk-registers/{riskRegisterId}/risks/{riskId}"),
    ("GET", "/roles"),
    ("GET", "/roles/{roleId}"),
    ("GET", "/roles/{roleId}/users"),
    ("GET", "/users"),
    ("GET", "/users/{userId}"),
    ("GET", "/vendors"),
    ("GET", "/vendors/{vendorId}"),
    ("GET", "/vendors/{vendorId}/questionnaires"),
    ("GET", "/vendors/{vendorId}/questionnaires/{questionnaireId}"),
    ("GET", "/vendors/{vendorId}/security-reviews"),
    ("GET", "/vendors/{vendorId}/security-reviews/{securityReviewId}"),
    ("GET", "/vendors/{vendorId}/security-reviews/{securityReviewId}/actions"),
    (
        "GET",
        "/vendors/{vendorId}/security-reviews/{securityReviewId}/security-questionnaires",
    ),
    ("GET", "/workspaces"),
    ("GET", "/workspaces/{workspaceId}/audits"),
    ("GET", "/workspaces/{workspaceId}/audits/{auditId}"),
    ("GET", "/workspaces/{workspaceId}/controls"),
    ("GET", "/workspaces/{workspaceId}/controls-requirement-comparison"),
    ("GET", "/workspaces/{workspaceId}/controls/{controlId}"),
    ("GET", "/workspaces/{workspaceId}/controls/{controlId}/requirements"),
    ("GET", "/workspaces/{workspaceId}/evidence-library"),
    ("GET", "/workspaces/{workspaceId}/evidence-library/{evidenceLibraryId}"),
    (
        "GET",
        "/workspaces/{workspaceId}/evidence-library/{evidenceLibraryId}/versions/{versionId}",
    ),
    ("GET", "/workspaces/{workspaceId}/frameworks"),
    ("GET", "/workspaces/{workspaceId}/frameworks/{frameworkId}/requirements"),
    ("GET", "/workspaces/{workspaceId}/monitoring-tests"),
    ("GET", "/workspaces/{workspaceId}/monitoring-tests/{testId}"),
    ("GET", "/workspaces/{workspaceId}/monitoring-tests/{testId}/exclusions"),
    ("GET", "/workspaces/{workspaceId}/monitoring-tests/{testId}/failures"),
    ("GET", "/workspaces/{workspaceId}/monitoring-tests/{testId}/passes"),
    ("POST", "/assets"),
    ("POST", "/devices/{deviceId}/documents"),
    ("POST", "/policies"),
    ("POST", "/risk-registers"),
    ("POST", "/risk-registers/{riskRegisterId}/risks"),
    ("POST", "/risk-registers/{riskRegisterId}/risks/{riskId}/documents"),
    ("POST", "/vendors"),
    ("POST", "/vendors/{vendorId}/documents"),
    ("POST", "/vendors/{vendorId}/questionnaires"),
    ("POST", "/vendors/{vendorId}/security-questionnaires"),
    ("POST", "/vendors/{vendorId}/security-reviews"),
    ("POST", "/vendors/{vendorId}/security-reviews/with-file"),
    (
        "POST",
        "/vendors/{vendorId}/security-reviews/{securityReviewId}/actions",
    ),
    (
        "POST",
        "/vendors/{vendorId}/security-reviews/{securityReviewId}/security-questionnaires",
    ),
    ("POST", "/workspaces/{workspaceId}/controls"),
    ("POST", "/workspaces/{workspaceId}/evidence-library"),
    ("POST", "/workspaces/{workspaceId}/frameworks"),
    ("PUT", "/assets/{assetId}"),
    ("PUT", "/personnel/{personnelId}"),
    ("PUT", "/policies/{policyId}"),
    ("PUT", "/risk-registers/{riskRegisterId}"),
    ("PUT", "/risk-registers/{riskRegisterId}/risks/{riskId}"),
    ("PUT", "/vendors/{vendorId}"),
    ("PUT", "/vendors/{vendorId}/security-reviews/{securityReviewId}"),
    ("PUT", "/workspaces/{workspaceId}/controls/{controlId}"),
    ("PUT", "/workspaces/{workspaceId}/evidence-library/{evidenceLibraryId}"),
    ("PUT", "/workspaces/{workspaceId}/frameworks/{frameworkId}"),
    ("PUT", "/workspaces/{workspaceId}/monitoring-tests/{testId}"),
];

/// The v0.1.5 sniff order, verbatim from 165c6cf. Do not edit to match today's
/// output: it is the reference the current dispatcher is held to.
fn v015_pick_renderer(sample: &Value) -> Option<RowRenderer> {
    let obj = sample.as_object()?;
    // Questionnaire rows carry `recipientEmail` + `isCompleted`.
    if obj.contains_key("recipientEmail") || obj.contains_key("isCompleted") {
        return Some(render_questionnaires);
    }
    // Vendor rows carry `category` + `risk` (plus `name`/`status`).
    if obj.contains_key("category") && obj.contains_key("risk") {
        return Some(render_vendors);
    }
    // Risk rows carry `treatmentPlan` + `riskId`.
    if obj.contains_key("treatmentPlan") || obj.contains_key("riskId") {
        return Some(render_risks);
    }
    // Control rows carry `code` + `question` (or just `code`).
    if obj.contains_key("code") && (obj.contains_key("question") || obj.contains_key("activity")) {
        return Some(render_controls);
    }
    // Device rows carry `serialNumber` or `isDeviceCompliant`.
    if obj.contains_key("serialNumber") || obj.contains_key("isDeviceCompliant") {
        return Some(render_devices);
    }
    // Personnel rows carry `employmentStatus`.
    if obj.contains_key("employmentStatus") {
        return Some(render_personnel);
    }
    // Policy rows carry `currentVersionId` or `scope`.
    if obj.contains_key("currentVersionId") || obj.contains_key("scope") {
        return Some(render_policies);
    }
    // Evidence library rows carry `evidenceTemplateCode`.
    if obj.contains_key("evidenceTemplateCode") || obj.contains_key("implementationGuidance") {
        return Some(render_evidence);
    }
    // Framework rows carry `numInScopeControls` or `shortName` (tag is unique to frameworks).
    if obj.contains_key("numInScopeControls") || (obj.contains_key("shortName") && obj.contains_key("slug")) {
        return Some(render_frameworks);
    }
    // Asset rows carry `assetType` + `assetProvider`.
    if obj.contains_key("assetType") || obj.contains_key("assetProvider") {
        return Some(render_assets);
    }
    // Workspace rows carry `primary` (bool) alongside `name`.
    if obj.contains_key("primary") && obj.contains_key("name") {
        return Some(render_workspaces);
    }
    // Risk register rows carry `owners` (array) alongside `workspaces` but no `primary`.
    if obj.contains_key("owners") && obj.contains_key("workspaces") {
        return Some(render_registers);
    }
    // User rows carry `firstName` + `lastName` (distinct from personnel `employmentStatus`).
    if obj.contains_key("firstName") && obj.contains_key("lastName") {
        return Some(render_users);
    }
    // Role rows carry `role` (the role name string) + `permissions`.
    if obj.contains_key("role") && obj.contains_key("permissions") {
        return Some(render_roles);
    }
    // Monitor/monitoring-test rows carry `checkResultStatus` or `checkStatus`.
    if obj.contains_key("checkResultStatus") || obj.contains_key("checkStatus") {
        return Some(render_monitors);
    }
    // Audit rows carry `auditType` or `frameworkType`.
    if obj.contains_key("auditType") || obj.contains_key("frameworkType") {
        return Some(render_audits);
    }
    // Event rows carry `requestDescription` or `testName` (both unique to events).
    if obj.contains_key("requestDescription") || obj.contains_key("testName") {
        return Some(render_events);
    }
    // Security review rows carry `reviewDeadlineAt` (unique to the security-review DTO).
    if obj.contains_key("reviewDeadlineAt") {
        return Some(render_security_reviews);
    }
    None
}

const SPEC: &str = include_str!("../../../spec/drata-openapi-v2.json");

/// Follow a local `$ref` (`#/components/schemas/Foo`) to its target.
fn resolve<'a>(spec: &'a Value, schema: &'a Value) -> &'a Value {
    match schema.get("$ref").and_then(Value::as_str) {
        Some(reference) => resolve(spec, spec.pointer(reference.trim_start_matches('#')).unwrap()),
        None => schema,
    }
}

/// Which keys a sample row carries. The sniffs read keys, not values, and a
/// server may send a null field or omit it, so the test checks both rows.
#[derive(Debug, Clone, Copy)]
enum Keys {
    /// Every property, `null` where the spec has no example.
    Every,
    /// Only the properties the spec gives an example.
    Examples,
}

/// The properties of an object schema, `allOf` members merged, each set to its
/// `example` (or null, under `Keys::Every`).
fn properties(spec: &Value, schema: &Value, keys: Keys) -> Map<String, Value> {
    let schema = resolve(spec, schema);
    let mut out = Map::new();
    for member in schema.get("allOf").and_then(Value::as_array).into_iter().flatten() {
        out.extend(properties(spec, member, keys));
    }
    for (name, prop) in schema
        .get("properties")
        .and_then(Value::as_object)
        .into_iter()
        .flatten()
    {
        match (resolve(spec, prop).get("example"), keys) {
            (Some(example), _) => {
                out.insert(name.clone(), example.clone());
            }
            (None, Keys::Every) => {
                out.insert(name.clone(), Value::Null);
            }
            (None, Keys::Examples) => {}
        }
    }
    out
}

/// The row `render` sniffs for an op: the first `data` item of a list
/// envelope, else the object itself. `None` for an op with no JSON 2xx body.
fn sample_row(spec: &Value, method: &str, path: &str, keys: Keys) -> Option<Value> {
    let responses = spec.pointer(&format!(
        "/paths/{}/{}/responses",
        path.replace('~', "~0").replace('/', "~1"),
        method.to_lowercase()
    ))?;
    let schema = responses
        .as_object()?
        .iter()
        .filter(|(status, _)| status.starts_with('2'))
        .find_map(|(_, r)| r.pointer("/content/application~1json/schema"))?;
    let body = resolve(spec, schema);
    if let Some(data) = body.pointer("/properties/data").map(|d| resolve(spec, d))
        && data.get("type").and_then(Value::as_str) == Some("array")
    {
        return Some(Value::Object(properties(spec, data.get("items")?, keys)));
    }
    Some(Value::Object(properties(spec, body, keys)))
}

fn rendered(pick: fn(&Value) -> Option<RowRenderer>, row: &Value) -> Option<String> {
    pick(row).map(|renderer| renderer(std::slice::from_ref(row), DEFAULT_WIDTH))
}

#[test]
fn every_v015_curated_op_keeps_its_v015_renderer() {
    let spec: Value = serde_json::from_str(SPEC).unwrap();
    assert_eq!(V015_CURATED.len(), 83);
    let mut checked = 0;
    let mut drift = Vec::new();
    for ((method, path), keys) in V015_CURATED
        .iter()
        .flat_map(|op| [(op, Keys::Every), (op, Keys::Examples)])
    {
        let Some(row) = sample_row(&spec, method, path, keys) else {
            continue;
        };
        checked += 1;
        let want = rendered(v015_pick_renderer, &row);
        let got = rendered(pick_renderer, &row);
        if want != got {
            let header = |t: &Option<String>| {
                t.as_deref().map_or("JSON fallback".to_string(), |t| {
                    t.lines().next().unwrap_or("").to_string()
                })
            };
            drift.push(format!(
                "{method} {path} ({keys:?} keys)\n    v0.1.5: {}\n    now:    {}",
                header(&want),
                header(&got)
            ));
        }
    }
    // The four DELETEs answer 204 and render nothing; the other 79 ops are
    // sniffed twice, once per `Keys`.
    assert_eq!(checked, 2 * 79, "ops with a JSON 2xx body");
    assert!(drift.is_empty(), "renderer drift from v0.1.5:\n{}", drift.join("\n"));
}

#[test]
fn sample_row_unwraps_the_list_envelope() {
    let spec: Value = serde_json::from_str(SPEC).unwrap();
    let row = sample_row(&spec, "GET", "/assets", Keys::Every).unwrap();
    assert!(row.get("assetType").is_some() && row.get("data").is_none(), "{row}");
}

#[test]
fn example_keys_drop_the_properties_with_no_example() {
    let spec: Value = serde_json::from_str(SPEC).unwrap();
    let path = "/workspaces/{workspaceId}/monitoring-tests/{testId}/failures";
    let every = sample_row(&spec, "GET", path, Keys::Every).unwrap();
    let examples = sample_row(&spec, "GET", path, Keys::Examples).unwrap();
    assert!(every.get("cause").is_some_and(Value::is_null), "{every}");
    assert!(examples.get("cause").is_none(), "{examples}");
}

#[test]
fn sample_row_is_none_for_a_204() {
    let spec: Value = serde_json::from_str(SPEC).unwrap();
    assert!(sample_row(&spec, "DELETE", "/assets/{assetId}", Keys::Every).is_none());
}
