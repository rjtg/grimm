# ADR-0002: Semantic boundary
Status: Accepted (initial design)

## Decision
Exchange semantic actions, committed events, narrative content, and currently valid interaction offers. Do not conflate hidden game events with player-visible output. Expose filtered projections.

## Consequences
The engine never emits renderer-specific animation or layout commands. Inputs are proposals validated by the engine.
