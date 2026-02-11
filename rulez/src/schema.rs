//! JSON Schema validation for incoming hook events.
//!
//! Validates event JSON against an auto-generated schema derived from the
//! Event struct. Uses fail-open semantics for schema validation: structurally
//! unexpected events log warnings but continue processing.
//!
//! **Important distinction (REQ-SCHEMA-04):**
//! - Schema validation is fail-open: extra fields, wrong optional types, etc.
//!   produce warnings but do NOT block processing.
//! - Serde deserialization is fail-closed: if the JSON cannot be deserialized
//!   into an Event struct (missing required fields like hook_event_name or
//!   session_id), that is a fatal error handled by the caller.

use schemars::schema_for;
use std::sync::LazyLock;
use tracing::warn;

use crate::models::Event;

/// Pre-compiled JSON Schema validator for Event structs.
/// Generated from the Event type's JsonSchema derive at startup.
/// Uses LazyLock for thread-safe, one-time initialization.
static EVENT_VALIDATOR: LazyLock<jsonschema::Validator> = LazyLock::new(|| {
    let schema = schema_for!(Event);
    let schema_value = serde_json::to_value(&schema).expect("Failed to serialize Event schema");
    jsonschema::validator_for(&schema_value).expect("Failed to compile Event schema validator")
});

/// Validate an event JSON value against the Event schema.
///
/// **Fail-open semantics (REQ-SCHEMA-04):**
/// - If the event JSON has schema deviations (extra fields, wrong types in
///   optional fields, missing optional fields), this function logs a warning
///   and returns so processing continues.
/// - This function does NOT handle malformed JSON (not valid JSON at all) --
///   that is caught earlier by serde_json::from_str() in process_hook_event().
/// - This function does NOT handle serde deserialization failures (missing
///   required fields) -- that is caught later by serde_json::from_value()
///   in process_hook_event() and IS fatal.
///
/// Returns () always (fail-open). Validation errors are logged as warnings.
pub fn validate_event_schema(event_json: &serde_json::Value) {
    if !EVENT_VALIDATOR.is_valid(event_json) {
        let errors: Vec<String> = EVENT_VALIDATOR
            .iter_errors(event_json)
            .map(|e| format!("{} at {}", e, e.instance_path))
            .collect();
        warn!(
            "Event schema validation warning (fail-open): {}",
            errors.join("; ")
        );
    }
}

/// Generate the Event JSON Schema as a serde_json::Value.
///
/// Useful for schema export (`rulez schema --export`) and testing.
#[allow(dead_code)]
pub fn generate_event_schema() -> serde_json::Value {
    let schema = schema_for!(Event);
    serde_json::to_value(&schema).expect("Failed to serialize Event schema")
}

/// Return the JSON Schema draft version used by the auto-generated schema.
///
/// schemars 1.2 generates JSON Schema 2020-12 (REQ-SCHEMA-06).
/// Since the schema is auto-generated (not user-provided), the draft version
/// is fixed by the schemars library version.
#[allow(dead_code)]
pub fn schema_draft_version() -> &'static str {
    "https://json-schema.org/draft/2020-12/schema"
}
