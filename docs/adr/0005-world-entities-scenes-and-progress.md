# ADR-0005: Separate world entities, narrative scenes and story progress
Status: Accepted (domain-model direction; implementation deferred)
Date: 2026-10-29

## Context
M1 models authored story nodes, choices and state variables. A player may visit the same warehouse by day, surveil it at night and return later: one physical location but distinct narrative situations. A character likewise exists independently of any scene containing that character. Conflating these concepts would make future persistent world state, NPC agency and visual representation unnecessarily difficult.

## Decision
1. **World entity:** something with stable identity and potentially persistent canonical state, such as a location, character or object. Specialized entity kinds need not share a monolithic type or generic ECS.
2. **Narrative scene:** a specific story situation or narrative unit, with content, choices and context. It can reference entities and is neither a location nor necessarily a durable world object. Existing M1 `Node` types can represent scenes without an immediate rename.
3. **Story progress:** the accumulated consequences, knowledge, flags, and optional future milestones of the story. It is not solely a current node ID or current location. Do not prescribe a parallel progress graph.
4. The engine continues to own canonical state; adapters and renderers receive only authorized player-visible projections. Entities are independent of presentation assets.
5. A narrative scene can reference multiple entities and the same entity can be used by many scenes. Being at a location does not imply any particular scene, time of day or narrative phase.

## Staging
- **M1.5:** author a separate reference game to test location reuse across three distinct scenes and stable character identity, using current M1 capabilities first. Add only small, justified identity/reference support if the authoring exercise demonstrates need. Capture problems explicitly.
- **M4:** introduce persisted entity state if dynamic-world features require it.
- **M5:** add optional NPC knowledge, beliefs and agency.
- **M6:** support optional narrative milestones independent of location.
- Do not retrofit the finished M0/M1 milestones or implement an all-purpose entity system during documentation work.

## Consequences
Scenes and locations remain independent, improving reuse in text, comics and future 3D frontends. Additional state and relationship models may be needed later, requiring explicit validation and versioning. M1 graph traversal and authored content remain valid. This ADR defines semantics and dependency direction, not final Rust structs or YAML syntax.
