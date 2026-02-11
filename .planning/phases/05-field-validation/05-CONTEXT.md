# Phase 5: Field Validation - Context

**Gathered:** 2026-02-09
**Status:** Ready for planning

<domain>
## Phase Boundary

Users can enforce required fields in tool inputs with fail-closed blocking, preventing incomplete or malformed tool invocations. Covers field existence checks, type validation, and dot notation for nested paths. Does NOT include array element validation, JSON Schema, or wildcard paths.

</domain>

<decisions>
## Implementation Decisions

### Validation Syntax
- `require_fields` is a simple list of field name strings: `require_fields: ["file_path", "content"]`
- `field_types` is a separate optional companion key as a YAML map: `field_types: {file_path: string, line: number}`
- Dot notation for nested fields with unlimited depth: `require_fields: ["input.user.address.city"]`
- No wildcard (`items.*.name`) or array index (`items[0].id`) notation — keep it simple

### Failure Behavior
- Report ALL missing/invalid fields in a single error, not just the first
- Block on type mismatch — wrong type is as bad as missing field (fail-closed)
- Error messages show types only, not actual values — avoids leaking sensitive data (e.g., "field 'count' expected number, got string")

### Type Checking Depth
- Strict JSON types only — no coercion ("42" is a string, not a number)
- Supported types: string, number, boolean, array, object, any
- Array type checks only "is it an array" — no element type validation
- Object type checks only "is it an object" — use dot notation for inner field requirements
- `any` type supported — field must exist but can be any JSON type

### Edge Cases & Defaults
- Null JSON values count as missing (null = absent for require_fields)
- Validate `require_fields` and `field_types` at config load time (consistent with prompt_match validation pattern)
- Missing/invalid `tool_input` causes all require_fields checks to fail (fail-closed, block)

### Claude's Discretion
- Whether `require_fields` acts as a matching condition or post-match validation (architectural decision for hooks.rs)
- Whether empty strings and empty arrays count as "present" (choose based on JSON semantics)
- Error message formatting and structure
- How field_types interacts with require_fields when both specify the same field

</decisions>

<specifics>
## Specific Ideas

- Follow the same validation-at-load pattern established in Phase 4 (prompt_match config validation)
- Research identified jsonschema 0.41 as a potential dependency — evaluate if needed or if custom validation is simpler
- Fail-closed philosophy should mirror enabled_when and prompt_match behavior

</specifics>

<deferred>
## Deferred Ideas

- Array element type validation (`array<string>`) — future enhancement
- Wildcard/glob field paths (`items.*.name`) — future enhancement
- Nested object schema validation (JSON Schema-like) — future enhancement

</deferred>

---

*Phase: 05-field-validation*
*Context gathered: 2026-02-09*
