# Contracts & Invariants

Document **rules that must always hold true** — for inputs, outputs, data integrity, or system behavior.  
Cursor and contributors must not violate these.

---

## [2025-10-04] HTTP Response Format
- What changed: All Lambda handlers must return structured JSON:
  `{ "status": "ok" | "error", "data": ... }`
- Why: Ensures consistent parsing and predictable client behavior.
- Affected modules: gateway/, core/http.rs

## [2025-10-04] Time Standardization
- What changed: All timestamps stored or emitted in UTC (RFC3339).
- Why: Prevents timezone drift and simplifies logging.
- Affected modules: core/time.rs, db/

## [Template]
- What changed:
- Why:
- Affected modules: