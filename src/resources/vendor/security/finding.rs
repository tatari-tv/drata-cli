//! Security-review notes and observations (both beta).
//!
//! The two collections are one shape: a free-text field (`note` or
//! `observation`), an optional `source`, and an optional related assessment.
//! One implementation, parameterized by `Kind`:
//!
//! | Verb | Method | Path |
//! |------|--------|------|
//! | `list` | GET | `/vendors/{v}/security-reviews/{r}/<notes\|observations>` |
//! | `get` | GET | `.../<collection>/{id}` |
//! | `create` | POST | `.../<collection>` |
//! | `update` | PUT | `.../<collection>/{id}` |
//! | `remove` | DELETE | `.../<collection>/{id}` |

use crate::cli::{FindingExpand, SecurityReviewNoteAction, SecurityReviewObservationAction};
use crate::client::DrataClient;
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::data::{DataSource, read_data};
use crate::expand::append_expand;
use crate::output::print_value;
use crate::query::{QueryPath, wire_name};
use crate::spec;
use eyre::{Result, bail};
use serde_json::{Value, json};
use tracing::{debug, instrument};

/// Which of the two collections a verb addresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Note,
    Observation,
}

impl Kind {
    /// The path segment after the security review.
    fn segment(self) -> &'static str {
        match self {
            Kind::Note => "notes",
            Kind::Observation => "observations",
        }
    }

    /// The body field holding the free text.
    fn field(self) -> &'static str {
        match self {
            Kind::Note => "note",
            Kind::Observation => "observation",
        }
    }

    /// The spec path template of the create op, for `--example`.
    fn create_spec_path(self) -> &'static str {
        match self {
            Kind::Note => "/vendors/{vendorId}/security-reviews/{securityReviewId}/notes",
            Kind::Observation => "/vendors/{vendorId}/security-reviews/{securityReviewId}/observations",
        }
    }

    /// The spec path template of the update op, for `--example`.
    fn update_spec_path(self) -> &'static str {
        match self {
            Kind::Note => "/vendors/{vendorId}/security-reviews/{securityReviewId}/notes/{securityReviewNoteId}",
            Kind::Observation => "/vendors/{vendorId}/security-reviews/{securityReviewId}/observations/{observationId}",
        }
    }
}

/// One verb with the review's collection path already built.
#[derive(Debug)]
pub struct Target<'a> {
    pub kind: Kind,
    pub collection: String,
    pub op: Op<'a>,
}

#[derive(Debug)]
pub enum Op<'a> {
    List {
        assessment_id: Option<u64>,
        expand: Vec<String>,
    },
    Get {
        id: u64,
        expand: Vec<String>,
    },
    Create {
        text: Option<&'a str>,
        source: Option<&'a str>,
        assessment_id: Option<u64>,
        data: Option<&'a str>,
    },
    Update {
        id: u64,
        text: Option<&'a str>,
        source: Option<&'a str>,
        clear_source: bool,
        data: Option<&'a str>,
    },
    Remove {
        id: u64,
    },
}

/// `<collection>` path of a review's notes or observations. Pure.
pub fn collection_path(vendor_id: &str, security_review_id: u64, kind: Kind) -> String {
    format!(
        "/vendors/{}/security-reviews/{}/{}",
        vendor_id,
        security_review_id,
        kind.segment()
    )
}

fn wire_names(expand: &[FindingExpand]) -> Vec<String> {
    expand.iter().map(wire_name).collect()
}

const MISSING_IDS: &str =
    "requires <vendor_id> <security_review_id> (and the item id for update); use --example to print the body skeleton";

/// Fold a clap note action into a `Target`. `create`/`update` ids are optional
/// in clap only so `--example` runs without them; here they are required.
pub fn note_target(action: &SecurityReviewNoteAction) -> Result<Target<'_>> {
    let kind = Kind::Note;
    Ok(match action {
        SecurityReviewNoteAction::List {
            vendor_id,
            security_review_id,
            assessment_id,
            expand,
        } => Target {
            kind,
            collection: collection_path(vendor_id, *security_review_id, kind),
            op: Op::List {
                assessment_id: *assessment_id,
                expand: wire_names(expand),
            },
        },
        SecurityReviewNoteAction::Get {
            vendor_id,
            security_review_id,
            note_id,
            expand,
        } => Target {
            kind,
            collection: collection_path(vendor_id, *security_review_id, kind),
            op: Op::Get {
                id: *note_id,
                expand: wire_names(expand),
            },
        },
        SecurityReviewNoteAction::Create {
            vendor_id,
            security_review_id,
            note,
            source,
            assessment_id,
            data,
            example: _,
        } => {
            let (Some(vendor_id), Some(security_review_id)) = (vendor_id, security_review_id) else {
                bail!("`vendor security-review note create` {MISSING_IDS}");
            };
            Target {
                kind,
                collection: collection_path(vendor_id, *security_review_id, kind),
                op: Op::Create {
                    text: note.as_deref(),
                    source: source.as_deref(),
                    assessment_id: *assessment_id,
                    data: data.as_deref(),
                },
            }
        }
        SecurityReviewNoteAction::Update {
            vendor_id,
            security_review_id,
            note_id,
            note,
            source,
            clear_source,
            data,
            example: _,
        } => {
            let (Some(vendor_id), Some(security_review_id), Some(note_id)) = (vendor_id, security_review_id, note_id)
            else {
                bail!("`vendor security-review note update` {MISSING_IDS}");
            };
            Target {
                kind,
                collection: collection_path(vendor_id, *security_review_id, kind),
                op: Op::Update {
                    id: *note_id,
                    text: note.as_deref(),
                    source: source.as_deref(),
                    clear_source: *clear_source,
                    data: data.as_deref(),
                },
            }
        }
        SecurityReviewNoteAction::Remove {
            vendor_id,
            security_review_id,
            note_id,
        } => Target {
            kind,
            collection: collection_path(vendor_id, *security_review_id, kind),
            op: Op::Remove { id: *note_id },
        },
    })
}

/// Fold a clap observation action into a `Target`.
pub fn observation_target(action: &SecurityReviewObservationAction) -> Result<Target<'_>> {
    let kind = Kind::Observation;
    Ok(match action {
        SecurityReviewObservationAction::List {
            vendor_id,
            security_review_id,
            assessment_id,
            expand,
        } => Target {
            kind,
            collection: collection_path(vendor_id, *security_review_id, kind),
            op: Op::List {
                assessment_id: *assessment_id,
                expand: wire_names(expand),
            },
        },
        SecurityReviewObservationAction::Get {
            vendor_id,
            security_review_id,
            observation_id,
            expand,
        } => Target {
            kind,
            collection: collection_path(vendor_id, *security_review_id, kind),
            op: Op::Get {
                id: *observation_id,
                expand: wire_names(expand),
            },
        },
        SecurityReviewObservationAction::Create {
            vendor_id,
            security_review_id,
            observation,
            source,
            assessment_id,
            data,
            example: _,
        } => {
            let (Some(vendor_id), Some(security_review_id)) = (vendor_id, security_review_id) else {
                bail!("`vendor security-review observation create` {MISSING_IDS}");
            };
            Target {
                kind,
                collection: collection_path(vendor_id, *security_review_id, kind),
                op: Op::Create {
                    text: observation.as_deref(),
                    source: source.as_deref(),
                    assessment_id: *assessment_id,
                    data: data.as_deref(),
                },
            }
        }
        SecurityReviewObservationAction::Update {
            vendor_id,
            security_review_id,
            observation_id,
            observation,
            source,
            clear_source,
            data,
            example: _,
        } => {
            let (Some(vendor_id), Some(security_review_id), Some(observation_id)) =
                (vendor_id, security_review_id, observation_id)
            else {
                bail!("`vendor security-review observation update` {MISSING_IDS}");
            };
            Target {
                kind,
                collection: collection_path(vendor_id, *security_review_id, kind),
                op: Op::Update {
                    id: *observation_id,
                    text: observation.as_deref(),
                    source: source.as_deref(),
                    clear_source: *clear_source,
                    data: data.as_deref(),
                },
            }
        }
        SecurityReviewObservationAction::Remove {
            vendor_id,
            security_review_id,
            observation_id,
        } => Target {
            kind,
            collection: collection_path(vendor_id, *security_review_id, kind),
            op: Op::Remove { id: *observation_id },
        },
    })
}

/// The `--example` skeleton when the action is a create/update `--example`.
pub fn example_if_requested(
    note: Option<&SecurityReviewNoteAction>,
    observation: Option<&SecurityReviewObservationAction>,
) -> Option<Result<String>> {
    let (kind, is_update) = match (note, observation) {
        (Some(SecurityReviewNoteAction::Create { example: true, .. }), _) => (Kind::Note, false),
        (Some(SecurityReviewNoteAction::Update { example: true, .. }), _) => (Kind::Note, true),
        (_, Some(SecurityReviewObservationAction::Create { example: true, .. })) => (Kind::Observation, false),
        (_, Some(SecurityReviewObservationAction::Update { example: true, .. })) => (Kind::Observation, true),
        _ => return None,
    };
    let (method, path) = if is_update {
        ("PUT", kind.update_spec_path())
    } else {
        ("POST", kind.create_spec_path())
    };
    Some(
        spec::example_for_operation(method, path)
            .and_then(|s| s.ok_or_else(|| eyre::eyre!("operation `{} {}` has no JSON request body", method, path))),
    )
}

/// `GET <collection>` path with the optional assessment filter and expand. Pure.
pub fn build_list_path(collection: &str, assessment_id: Option<u64>, expand: &[String]) -> String {
    let mut q = QueryPath::new(collection);
    q.push_opt("vendorAssessmentId", assessment_id);
    append_expand(&q.finish(), expand)
}

/// `GET|PUT|DELETE <collection>/{id}` path. Pure.
pub fn build_item_path(collection: &str, id: u64) -> String {
    format!("{}/{}", collection, id)
}

/// `POST <collection>` body from flags. The free text is required. Pure.
pub fn build_create_body(
    kind: Kind,
    text: Option<&str>,
    source: Option<&str>,
    assessment_id: Option<u64>,
) -> Result<Value> {
    let text = text.ok_or_else(|| eyre::eyre!("create requires --{} (or use --data/--example)", kind.field()))?;
    let mut body = json!({ kind.field(): text });
    if let Some(s) = source {
        body["source"] = json!(s);
    }
    if let Some(id) = assessment_id {
        body["vendorAssessmentId"] = json!(id);
    }
    Ok(body)
}

/// `PUT <collection>/{id}` body from flags. Omitted fields keep their value;
/// `--clear-source` sends `source: null`. An empty update is an error. Pure.
pub fn build_update_body(kind: Kind, text: Option<&str>, source: Option<&str>, clear_source: bool) -> Result<Value> {
    let mut body = json!({});
    if let Some(t) = text {
        body[kind.field()] = json!(t);
    }
    if let Some(s) = source {
        body["source"] = json!(s);
    } else if clear_source {
        body["source"] = Value::Null;
    }
    if body.as_object().is_some_and(|o| o.is_empty()) {
        bail!(
            "update requires --{}, --source or --clear-source (or use --data/--example)",
            kind.field()
        );
    }
    Ok(body)
}

/// Run one note or observation verb.
#[instrument(skip(client, config, confirm))]
pub async fn handle(client: &DrataClient, config: &Config, confirm: &ConfirmFn, target: Target<'_>) -> Result<()> {
    let Target { kind, collection, op } = target;
    debug!(?kind, collection, ?op, "security-review finding");
    match op {
        Op::List { assessment_id, expand } => {
            let items = client
                .get_all(&build_list_path(&collection, assessment_id, &expand))
                .await?;
            print_value(&json!({ "data": items }), &config.output_format);
        }
        Op::Get { id, expand } => {
            let path = append_expand(&build_item_path(&collection, id), &expand);
            let resp = client.get(&path).await?;
            print_value(&resp, &config.output_format);
        }
        Op::Create {
            text,
            source,
            assessment_id,
            data,
        } => {
            let body = match data {
                Some(raw) => read_data(raw, DataSource::Any)?,
                None => build_create_body(kind, text, source, assessment_id)?,
            };
            if !confirm("POST", &collection)? {
                bail!("aborted");
            }
            let result = client.post(&collection, body).await?;
            print_value(&result, &config.output_format);
        }
        Op::Update {
            id,
            text,
            source,
            clear_source,
            data,
        } => {
            let body = match data {
                Some(raw) => read_data(raw, DataSource::Any)?,
                None => build_update_body(kind, text, source, clear_source)?,
            };
            let path = build_item_path(&collection, id);
            if !confirm("PUT", &path)? {
                bail!("aborted");
            }
            let result = client.put(&path, body).await?;
            print_value(&result, &config.output_format);
        }
        Op::Remove { id } => {
            let path = build_item_path(&collection, id);
            if !confirm("DELETE", &path)? {
                bail!("aborted");
            }
            let result = client.delete(&path).await?;
            print_value(&result, &config.output_format);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
