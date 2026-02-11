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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_generate_event_schema_is_valid_json_schema() {
        let schema = generate_event_schema();

        // Verify it's an object with $schema key
        assert!(schema.is_object(), "Schema should be a JSON object");

        let schema_obj = schema.as_object().unwrap();
        assert!(
            schema_obj.contains_key("$schema"),
            "Schema should have $schema field"
        );

        // Verify top-level type is object
        assert_eq!(
            schema_obj.get("type").and_then(|v| v.as_str()),
            Some("object"),
            "Schema top-level type should be 'object'"
        );

        // Verify it has properties
        assert!(
            schema_obj.contains_key("properties"),
            "Schema should have properties field"
        );

        let properties = schema_obj.get("properties").unwrap().as_object().unwrap();

        // Verify required fields are in properties
        assert!(
            properties.contains_key("hook_event_name"),
            "Properties should include hook_event_name"
        );
        assert!(
            properties.contains_key("session_id"),
            "Properties should include session_id"
        );

        // Verify required array contains required fields
        assert!(
            schema_obj.contains_key("required"),
            "Schema should have required field"
        );

        let required = schema_obj.get("required").unwrap().as_array().unwrap();
        assert!(
            required.contains(&json!("hook_event_name")),
            "Required array should contain hook_event_name"
        );
        assert!(
            required.contains(&json!("session_id")),
            "Required array should contain session_id"
        );
    }

    #[test]
    fn test_schema_draft_version_is_2020_12() {
        // Verify the function returns the correct draft version (REQ-SCHEMA-06)
        let draft_version = schema_draft_version();
        assert_eq!(
            draft_version, "https://json-schema.org/draft/2020-12/schema",
            "Schema draft version should be 2020-12"
        );

        // Verify the generated schema matches the documented draft version
        let schema = generate_event_schema();
        let schema_field = schema
            .get("$schema")
            .and_then(|v| v.as_str())
            .expect("Schema should have $schema field");

        assert_eq!(
            schema_field, draft_version,
            "Generated schema $schema field should match schema_draft_version()"
        );
    }

    #[test]
    fn test_validate_valid_event_passes() {
        // Create a valid event JSON
        let event = json!({
            "hook_event_name": "PreToolUse",
            "session_id": "test-123",
            "tool_name": "Bash",
            "tool_input": {"command": "ls"},
            "cwd": "/tmp"
        });

        // Validate - should not panic (fail-open mode)
        validate_event_schema(&event);
        // If we reach here without panic, the test passes
    }

    #[test]
    fn test_validate_missing_required_fields_warns_but_returns() {
        // Create JSON missing required fields (fail-open schema validation)
        let event = json!({
            "tool_name": "Bash"
        });

        // Validate - should return without panic (fail-open)
        validate_event_schema(&event);
        // Cannot easily assert on tracing::warn output in unit tests.
        // The fact that the function returns is sufficient to verify fail-open.
    }

    #[test]
    fn test_validate_wrong_type_warns_but_returns() {
        // Create JSON with wrong types
        let event = json!({
            "hook_event_name": 42,
            "session_id": true
        });

        // Validate - should return without panic (fail-open)
        validate_event_schema(&event);
    }

    #[test]
    fn test_validate_empty_object_warns_but_returns() {
        // Create empty JSON object
        let event = json!({});

        // Validate - should return without panic (fail-open)
        validate_event_schema(&event);
    }

    #[test]
    fn test_validate_extra_fields_accepted() {
        // Create JSON with all required fields PLUS unknown extra fields
        let event = json!({
            "hook_event_name": "PreToolUse",
            "session_id": "test-123",
            "unknown_field": "should be accepted"
        });

        // Validate - should not produce warnings (schemars does NOT generate
        // additionalProperties: false unless #[serde(deny_unknown_fields)])
        validate_event_schema(&event);
    }

    #[test]
    fn test_schema_contains_event_type_enum_variants() {
        let schema = generate_event_schema();

        // The schema should have $defs or definitions containing EventType enum
        let schema_obj = schema.as_object().unwrap();

        // Check for $defs (JSON Schema 2020-12 uses $defs instead of definitions)
        assert!(
            schema_obj.contains_key("$defs") || schema_obj.contains_key("definitions"),
            "Schema should have $defs or definitions for EventType enum"
        );

        // Get the definitions
        let defs = schema_obj
            .get("$defs")
            .or_else(|| schema_obj.get("definitions"))
            .expect("Schema should have type definitions");

        let defs_obj = defs.as_object().unwrap();

        // EventType should be defined
        assert!(
            defs_obj.contains_key("EventType"),
            "Definitions should contain EventType"
        );

        // Verify EventType is an enum with expected variants
        let event_type_schema = defs_obj.get("EventType").unwrap();
        let event_type_obj = event_type_schema.as_object().unwrap();

        // Should have oneOf or enum field listing variants
        assert!(
            event_type_obj.contains_key("oneOf") || event_type_obj.contains_key("enum"),
            "EventType schema should have oneOf or enum field"
        );
    }
}
