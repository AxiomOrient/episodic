# XML Utility Review

Reviewed file:
- `src/xml.rs`

## `src/xml.rs`

Role:
- Escapes XML-sensitive text for text-node and attribute contexts.

What is implemented:
- Single internal function `escape_xml(text, attribute)` with char scan.
- Two wrappers:
  - `escape_xml_text`
  - `escape_xml_attribute`

Data-first notes:
- Explicit contract for text-vs-attribute escaping.

Simplicity notes:
- One-pass character matching; no chained replacements.

Performance notes:
- `String::with_capacity(text.len())` and single pass reduce transient allocations.

Risks:
- Escaping logic is intentionally minimal; does not validate or parse XML structures.

