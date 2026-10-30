# Architecture — initial boundary contract

This document fixes **dependency boundaries**, not all future APIs. Public types will evolve through milestones.

The implemented M1 crate map, authoring/checkpoint DTOs and provisional protocol
migration are described in [the M1 brief](milestones/m1.md) and
[story/save format](story-format.md). Engine dependencies remain semantic-only;
YAML/JSON file handling belongs to the CLI composition root.

## Flow

```text
Game definition -> Engine (state/rules/graph) -> semantic output
                                                 |
                                                 v
                                         Output adapter
                                                 |
                                                 v
                                        Presentation commands
                                                 |
                                                 v
                                             Renderer
                                                 |
                                          Raw player input
                                                 |
                                                 v
                                           Input adapter
                                                 |
                                                 v
                                        Semantic game action -> Engine
```

A composition root wires concrete components. Engine modules must never import adapters, renderers, UI frameworks, or LLM SDKs.

## Domain ownership
- **Engine:** authoritative world/game state; validates actions; commits effects; records events; graph traversal.
- **Semantic protocol:** typed actions, committed game events, narrative content, currently valid interaction offers, and player-visible projections. These are related but not interchangeable.
- **Output adapter:** maps already-authorized semantic output to presentation commands; may maintain presentation state, but cannot modify game truth. Optional stylistic text transformation is deferred, not a requirement for dynamic narrative.
- **Renderer:** displays presentation commands (text, comic, 3D) and reports raw user interactions / presentation completion.
- **Input adapter:** converts raw UI input or text into proposed semantic actions; engine validates proposals.
- **Composition root:** chooses engine capabilities, adapters, renderers and transports.

## World entities, narrative scenes and progress
World entities are persistent identities in the game world: characters, locations and eventually objects or other meaningful things. Specialized entity kinds share identity and state-ownership principles but do **not** require one generic entity type, inheritance hierarchy, entity-component system, or mandatory central entity store.

Narrative scenes are authored or generated situations describing what happens, what may be shown and which actions are offered. A scene may reference zero, one or multiple entities; an entity may participate in many scenes. The current M1 story node is a narrative unit and must **not** be treated as synonymous with a physical location. A warehouse visited during daylight investigation, night surveillance and a return visit can be three separate scenes that share one location identity.

Story progress is likewise separate from a scene or location. The engine may express progress through state variables and committed effects, and later optional narrative milestones; a second parallel progress graph is not mandated. The engine remains authoritative for entity state and transitions. Presentation adapters receive only deliberately revealed context, not private entity state.

Implementation staging: M1.5 first tests these distinctions by writing a second game using existing primitives, then makes minimal format/API additions if evidence requires them. Minimal item and character state may be introduced as authored choice-driven adventure requirements demand it. Dynamic node proposals, NPC agency and narrative direction remain uncommitted later extensions, not milestones M2–M5. See [ADR-0005](adr/0005-world-entities-scenes-and-progress.md).

## Temporal semantics
Time is a cross-cutting but opt-in game rule, distinct from narrative progress and presentation animation time. **World time** advances through explicit engine-validated steps; **decision deadlines** express how long a player may act; **narrative progress** reflects consequences rather than a clock tick alone. The host/runtime supplies clock or elapsed-time input, but the engine owns committed temporal state and determines timeout outcomes. No system-clock, timer, sleep, or render-framework dependency enters the headless engine.

Decision deadlines require an explicit start condition (for example a host acknowledgement that a prompt became available), deterministic ordering for competing action/timeout inputs, and idempotent resolution for an interaction revision. Pausing and restoring must define whether the decision timer is suspended or expires offline; this policy must be documented and tested, not inferred from wall-clock timestamps. Renderers may display countdowns, but cannot authoritatively decide the story consequence. Existing untimed authored games must behave exactly as before.

Time advancement may activate conditions and future scheduled reactions, but M1.75 need not implement continuous simulation, background NPCs, or a general event scheduler. The engine validates any temporal effects and resulting transitions. See [ADR-0006](adr/0006-game-time-and-temporal-semantics.md) and [M1.75](milestones/m1.75.md).

## Narrative flow and presentation gating (M1.8)
A narrative scene is an engine-owned unit of story progression, not necessarily an immediate player choice or a physical location. Storytelling presentation and interaction activation are separate phases. A scene may be (a) narrative-only, continuing via an explicit authorized presentation-complete acknowledgement or deliberate player-paced Continue, or (b) interactive, whose offers become available only after any required storytelling has completed. Avoid mandatory dummy choices whose only purpose is to proceed after dialogue/camera motion.

The **output adapter/renderer** decides how to present the authorized narrative (text, spoken lines, comic panels, camera moves), and can signal presentation completion or interaction availability through a renderer-neutral acknowledgement to the **host/composition root**. The host validates/correlates the signal with the active presentation/interaction revision before proposing a typed lifecycle event to the **engine**. Neither renderer nor output adapter directly calls engine code, mutates canonical state, decides the next node, or authorizes timeout consequences. The engine validates and commits narrative continuation and timed-interaction activation, at most once per revision. Different renderer technologies may take different real durations without altering fictional chronology or spending a decision budget before interaction readiness.

Do not conflate presentation-complete with decision-ready: a pre-rendered choice list does not imply that an ongoing required dialogue/animation has completed, and a completed narration does not imply the next offer has been shown to the player. M1.75 `AwaitingReady` and `Ready` form the existing foundation but do not themselves guarantee the user has read CLI text. M1.8 should specify minimal state/protocol changes and prevent early, duplicate, stale and out-of-order signals from starting deadlines or committing non-choice transitions. Pending presentation, pause, restore and frontend disconnect behavior require explicit deterministic policies. Automatically chaining narrative-only nodes must be bounded and cycle-safe; waiting for renderer completion must never block the core synchronously.

CLI fallback: for reading-intensive storytelling, display the narrative and require an explicit Enter acknowledgement before exposing/activating the timed decision. A visual renderer can instead report required dialogue, animation, or camera completion. User-driven Continue remains a legitimate, intentional pacing choice, but is not the sole way to encode a non-interactive segment.

See [ADR-0007](adr/0007-narrative-flow-and-presentation-gating.md) and [M1.8](milestones/m1.8.md). These refine the existing M0 dependency boundaries and M1.75 temporal lifecycle without making visual rendering mandatory.

M1.8's implemented lifecycle is specified in the [pre-API design](milestones/m1.8-design.md)
and [schema/protocol reference](story-format.md#story-schema-3-narrative-flow-and-presentation-gating-m18).
Opt-in schema-3 nodes expose Presenting → AwaitingInteraction → Interactive,
with orthogonal pause state. Revision-scoped presentation feedback travels from
renderer to host, through adapter translation into `FlowInput`, then engine
validation. Completion reserves a new revision before offers become available;
readiness activates the existing deadline only afterward. Narrative-only
completion commits one validated successor edge. The core never waits for a
renderer, and continuation-only cycles are rejected. Required CLI reading uses
explicit Enter; EOF cannot stand in for completion. ADR-0007 retains Proposed status.

## Event and presentation semantics
Committed game events describe what actually happened, including offscreen events. They are not automatically visible to players. Narrative content describes what may be communicated; interaction offers describe currently valid inputs. A filtered projection exposes only player-visible information. A renderer must not receive hidden world state by default.

Authored narrative and semantically described choices remain the primary inputs. M2 develops **in-scene choices** that change state and available offers without changing the narrative node; M3 adds optional inventory/item combinations and RPG-like attributes as conditional choice state; M4 brings together LucasArts-style puzzle exploration and Telltale-style consequential narrative scenes; M5 adds graphical presentation of the **same** engine-authored offers. No fixed Inspect/Use/Take/Combine primitive is required. Dynamic graph/node providers and generative narration belong to deferred open-world questions, not current milestones. See [open questions](open-questions.md).

## State and lifecycle
- Engine state and presentation state are distinct.
- Adapters may be stateful; a filtered snapshot enables reconnection and adapter initialization.
- Input references an active interaction/session revision; stale choices are rejected.
- Presentation completion may be acknowledged, but simulation must not depend on rendering speed except where explicitly synchronized.
- State changes and emitted events are committed atomically; event-driven follow-up processing must be bounded and deterministic.
- For deterministic replay, record external model results or resolved actions rather than assuming an LLM can reproduce output.

## Extension model
Optional graph dynamics, intent resolution, NPC controllers, and narrative direction integrate through explicit engine interfaces and validated action/event processing. Do not implement a general-purpose plugin framework in M0.

## Deadline semantics for authored choices and scenes

Two distinct, opt-in authoritative time constraints must be preserved. The **M1.75 choice-selection deadline** ends when the player *chooses* an eligible offer; failure to respond triggers the authored no-response/timeout transition (for example a conversation partner becomes angry). A timely accepted choice is not retrospectively rejected solely because its subsequent animation/action takes longer. In contrast, a new **M2 scene-completion deadline** requires an authored objective to be *completed* before expiry; multiple choices and timed actions may occur within one scene without resetting that deadline. Beginning a solution does not count as completing it.

A scene timeout can occur in the middle of an action. The engine must deterministically compute an interruption snapshot at the event instant, including canonical action identity and elapsed/remaining execution progress, committed world/character stats, and relevant deadline state. Authored conditional transitions may use this context (e.g. a detective arrives halfway through hiding a weapon; detection depends on concealment progress and `suspicion`). Full completion effects must not apply if completion has not occurred; any partial effects require explicit rules. Handle exact ties and time observations jumping over several event boundaries consistently and atomically. Neither static timer outcomes nor generic engine-defined checks can substitute for authored intermediate-state branches. Preserve existing schema and legacy M1.75 deadline behavior.

## Choice-driven adventure evolution (M2–M5)
An ongoing narrative scene can expose authored choices that change world, inventory or character state and therefore refresh its available choices **without** requiring a new graph node. This is distinct from transitioning to a new narrative scene. A single time-critical situation may span many actions; its timer must not reset when options refresh. An authored choice can also take **execution time**: after acceptance, the engine tracks a canonical in-progress action and rejects conflicting gameplay choices until completion/interruption. A **scene-completion** deadline keeps progressing and may interrupt completion; a separate **choice-selection** deadline instead resolves when a valid choice is accepted, or fires a no-response consequence. Do not equate an action's execution duration with fictional world minutes or M1.8 presentation latency. The host supplies elapsed observations and renders progress/animation **against the same action start and authored execution duration**: a five-second action has a five-second animation/progress interval. Playback may catch up or drop frames under lag but must not add an extra delay, lengthen the input lock, or postpone a canonical completion/timeout. Assets that cannot fit should be adapted or rejected. M1.8 pre-choice narrative gating is separate from action-animation synchronization. The core neither sleeps nor waits for the renderer. Define atomic effect timing, exact-boundary ordering and pause/save/restore behavior in M2. Presentation feedback after an action must not inadvertently replay initial narrative gating. The engine remains responsible for canonical effects, revision safety and authored timeouts.

A LucasArts-style puzzle is an authored sequence of choices involving searching, discovering, collecting and combining objects; a Telltale-style scene offers tightly staged consequential choices, optionally timed or presentation-gated. Both share one conditional-choice mechanism. RPG-style stats/skills/relationships are additional optional state inputs and effects, not a required character-system framework. A renderer may display these as numbered choices, graphical hotspots or inventory views. Never bind location identity, scene identity and narrative progress into one concept.

Every milestone must ship a small playable game that demonstrates each new gameplay capability and relevant failure paths, with automated end-to-end coverage and authored playtest notes. Dynamic nodes and provider boundaries are intentionally **not** specified before the authored adventure mechanics have been exercised. Later open-world/Story-RPG proposals could come from human DMs, rule systems or AI through an engine-validated boundary, but that design is postponed.

## Open design questions
The living [open-points catalog](open-questions.md) tracks in-scene choice lifecycle, non-resetting timers, puzzle/state graph checking, inventory and character stats. Runtime node providers and generative models are deferred to a later open-world phase; resolve no proposed APIs for those yet.
