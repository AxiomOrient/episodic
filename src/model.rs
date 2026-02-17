use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OmScope {
    Session,
    Thread,
    Resource,
}

impl OmScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Session => "session",
            Self::Thread => "thread",
            Self::Resource => "resource",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "session" => Some(Self::Session),
            "thread" => Some(Self::Thread),
            "resource" => Some(Self::Resource),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OmOriginType {
    Initial,
    Reflection,
}

impl OmOriginType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Initial => "initial",
            Self::Reflection => "reflection",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "initial" => Some(Self::Initial),
            "reflection" => Some(Self::Reflection),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OmRecord {
    pub id: String,
    pub scope: OmScope,
    pub scope_key: String,
    pub session_id: Option<String>,
    pub thread_id: Option<String>,
    pub resource_id: Option<String>,
    pub generation_count: u32,
    pub last_applied_outbox_event_id: Option<i64>,
    pub origin_type: OmOriginType,
    pub active_observations: String,
    pub observation_token_count: u32,
    pub pending_message_tokens: u32,
    pub last_observed_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_task: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggested_response: Option<String>,
    #[serde(default)]
    pub last_activated_message_ids: Vec<String>,
    #[serde(default)]
    pub observer_trigger_count_total: u32,
    #[serde(default)]
    pub reflector_trigger_count_total: u32,
    pub is_observing: bool,
    pub is_reflecting: bool,
    pub is_buffering_observation: bool,
    pub is_buffering_reflection: bool,
    pub last_buffered_at_tokens: u32,
    pub last_buffered_at_time: Option<DateTime<Utc>>,
    pub buffered_reflection: Option<String>,
    pub buffered_reflection_tokens: Option<u32>,
    pub buffered_reflection_input_tokens: Option<u32>,
    pub reflected_observation_line_count: Option<u32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OmObservationChunk {
    pub id: String,
    pub record_id: String,
    pub seq: u32,
    pub cycle_id: String,
    pub observations: String,
    pub token_count: u32,
    pub message_tokens: u32,
    pub message_ids: Vec<String>,
    pub last_observed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OmRecordInvariantViolation {
    MissingScopeIdentifier {
        field: &'static str,
    },
    EmptyIdentifier {
        field: &'static str,
    },
    EmptyScopeKey,
    ScopeKeyPrefixMismatch {
        expected_prefix: &'static str,
    },
    ScopeKeyIdentifierMismatch {
        expected_identifier: String,
        actual_identifier: String,
    },
    EmptyBufferedReflection,
    BufferedReflectionMetadataWithoutText {
        field: &'static str,
    },
}

pub fn validate_om_record_invariants(record: &OmRecord) -> Vec<OmRecordInvariantViolation> {
    let mut violations = Vec::<OmRecordInvariantViolation>::new();

    let session_id = record.session_id.as_deref().map(str::trim);
    if session_id.is_some_and(str::is_empty) {
        violations.push(OmRecordInvariantViolation::EmptyIdentifier {
            field: "session_id",
        });
    }
    let thread_id = record.thread_id.as_deref().map(str::trim);
    if thread_id.is_some_and(str::is_empty) {
        violations.push(OmRecordInvariantViolation::EmptyIdentifier { field: "thread_id" });
    }
    let resource_id = record.resource_id.as_deref().map(str::trim);
    if resource_id.is_some_and(str::is_empty) {
        violations.push(OmRecordInvariantViolation::EmptyIdentifier {
            field: "resource_id",
        });
    }

    let (required_identifier, required_field, expected_prefix) = match record.scope {
        OmScope::Session => (
            session_id.filter(|value| !value.is_empty()),
            "session_id",
            "session:",
        ),
        OmScope::Thread => (
            thread_id.filter(|value| !value.is_empty()),
            "thread_id",
            "thread:",
        ),
        OmScope::Resource => (
            resource_id.filter(|value| !value.is_empty()),
            "resource_id",
            "resource:",
        ),
    };
    if required_identifier.is_none() {
        violations.push(OmRecordInvariantViolation::MissingScopeIdentifier {
            field: required_field,
        });
    }

    let scope_key = record.scope_key.trim();
    if scope_key.is_empty() {
        violations.push(OmRecordInvariantViolation::EmptyScopeKey);
    } else if !scope_key.starts_with(expected_prefix) {
        violations.push(OmRecordInvariantViolation::ScopeKeyPrefixMismatch { expected_prefix });
    } else if let Some(expected_identifier) = required_identifier {
        let actual_identifier = scope_key
            .strip_prefix(expected_prefix)
            .unwrap_or_default()
            .trim();
        if actual_identifier != expected_identifier {
            violations.push(OmRecordInvariantViolation::ScopeKeyIdentifierMismatch {
                expected_identifier: expected_identifier.to_string(),
                actual_identifier: actual_identifier.to_string(),
            });
        }
    }

    let has_buffered_reflection = match record.buffered_reflection.as_deref() {
        Some(value) => {
            if value.trim().is_empty() {
                violations.push(OmRecordInvariantViolation::EmptyBufferedReflection);
                false
            } else {
                true
            }
        }
        None => false,
    };
    if !has_buffered_reflection {
        if record.buffered_reflection_tokens.is_some() {
            violations.push(
                OmRecordInvariantViolation::BufferedReflectionMetadataWithoutText {
                    field: "buffered_reflection_tokens",
                },
            );
        }
        if record.buffered_reflection_input_tokens.is_some() {
            violations.push(
                OmRecordInvariantViolation::BufferedReflectionMetadataWithoutText {
                    field: "buffered_reflection_input_tokens",
                },
            );
        }
    }

    violations
}

#[cfg(test)]
mod tests;
