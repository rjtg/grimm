# ADR-0004: Canonical state ownership
Status: Accepted (initial design)

## Decision
Only the engine validates and commits changes to canonical game state. AI and adapters may propose actions or create non-canonical presentation details; they cannot directly write game truth.

## Consequences
Generated prose must not be treated as evidence of new game facts. Private NPC plans remain hidden unless the engine intentionally reveals them. External model results are recorded if deterministic replay is required.
