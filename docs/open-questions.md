# Open design questions

This is GRIMM's running **open-points catalog**. Questions here are deliberately **not** accepted design decisions or API contracts. Record the selected option, rationale, ADR reference, tests and migration policy when the relevant milestone is designed. Requirements and invariants in `docs/architecture.md` and `docs/requirements.md` remain binding. Do not resolve an OPEN question merely by choosing a convenient implementation detail.

Status vocabulary: **Open**, **Investigating**, **Decided** (with link), **Deferred**. Owners may be assigned in implementation PRs.

## Generative narrative and presentation

The former M2 “Generative Presentation” was replaced by M2 **Dynamic Narrative Foundation** (typed, validated and persisted runtime proposals without requiring LLMs). M4 **Generative Narrative** may later supply proposals through the same boundary. These questions should be revisited while designing M2 and M4, **before** promising that an LLM can reliably verify prose against game truth.

| ID | Status | Target | Question / choice to resolve |
|---|---|---|---|
| OPEN-GEN-01 | Open | M2/M4 | **Boundary of generation:** Which information is authoritative narrative content in a committed node, and which is purely a renderer's stylistic transformation? Should optional style rewriting exist at all, or be left to later presentation experimentation? |
| OPEN-GEN-02 | Open | M2 | **Proposal granularity:** Should a generator propose a whole node, an event plus presentation content, a bounded sequence of nodes, or a constrained combination? How do these relate to authored nodes, narrative-only `continue_to` and M1.8 gated interactions? |
| OPEN-GEN-03 | Open | M2 | **Proposal contracts:** What minimum typed structure carries node identity, content, choice targets, conditions/effects, references, new facts and event provenance? What can safely remain free text? |
| OPEN-GEN-04 | Open | M2/M4 | **Truth versus prose:** How can text be constrained not to claim canonical events, character deaths, opened doors, learned secrets or other state changes absent from committed events? Which checks can be deterministic, and which are only best-effort? What happens when prose contradicts state? |
| OPEN-GEN-05 | Open | M2/M4 | **New facts and authority:** Which new characters, objects, locations, relationships, knowledge or world variables may a proposal introduce, and under whose authorization? When must a proposed fact be represented by an explicit validated event/effect rather than narration? |
| OPEN-GEN-06 | Open | M2/M4 | **Safe context projection:** What world state, prior events, character beliefs, author constraints and player history may a generator see? How do we avoid leaking hidden facts via prompt, output, logs, traces and persisted proposals? |
| OPEN-GEN-07 | Open | M4 | **Author constraints:** Which text/dialogue is immutable, which may be paraphrased, and what about canon, tone, genre, character voice, safety rules and continuity? Can constraints be declared in authoring data? |
| OPEN-GEN-08 | Open | M2/M4 | **Validation and fallback:** Which failures cause reject, repair/retry (bounded), asking a player for clarification, or authored/rule-based fallback? How do we avoid loops, softlocks and a generator silently manufacturing an unearned success? |
| OPEN-GEN-09 | Open | M2/M4 | **Identity and graph lifetime:** How are dynamic node/entity IDs assigned and collision-free across save/restore, branching, replay and imported authored stories? Are rejected proposals stored? How are overlay revisions and stale proposals ordered against timeouts/actions? |
| OPEN-GEN-10 | Open | M2/M4 | **Determinism:** Save raw model response, normalized accepted proposal, resolved action/event, or all of these? What is needed for exact engine replay, audit, crash recovery and optional re-rendering without recalling a model? |
| OPEN-GEN-11 | Open | M4 | **Presentation variance:** If wording is generated after canonical commit, can different users or renderers see divergent implications? What must be pinned to a session, preserved on save/load, and held fixed for exact authored dialogue? |
| OPEN-GEN-12 | Open | M2/M4 | **Latency and interaction fairness:** When may content generation occur relative to `Presenting`, `PresentationComplete`, `InteractionReady`, pause/resume and timed decisions? Generation or render latency must not consume unseen decision time or silently advance the world clock. |
| OPEN-GEN-13 | Open | M4 | **Provider lifecycle and privacy:** What optional interface handles timeouts, unavailable/offline providers, redaction, output limits and cancellation? No LLM SDK or network dependency may enter the headless engine. |
| OPEN-GEN-14 | Open | M2/M4 | **Testing and evidence:** How do rule-based fixtures, adversarial invalid proposals, bounded graph checks, schema fuzzing, recorded model responses and human narrative playtests demonstrate continuity and reject invented facts? |
| OPEN-GEN-15 | Open | M3/M4 | **Player action versus generated narrative:** When free-text intent introduces an unscripted action, who proposes the resulting node/event, and how are resolution, player consent/ambiguity and engine validation separated? |
| OPEN-GEN-16 | Open | M4+ | **Milestone scope:** Is a separate optional generative presentation adapter valuable enough to implement later? If so, how does it avoid duplicate narrative authority and reuse the existing presentation interface? This does not block M2. |

### Suggested decision order

1. **Before M2 implementation:** resolve OPEN-GEN-02, -03, -05, -08, -09 and -10 for a **rule-based** provider with no LLM. Establish invariants and tests for accepted/rejected proposals, persistence and deterministic replay.
2. **Before M4 provider implementation:** resolve OPEN-GEN-01, -04, -06, -07, -11, -13 and -14 with explicit limits on prose consistency guarantees, safety projections and fallback. Reuse M2's canonical proposal model.
3. **As relevant to later capabilities:** revisit OPEN-GEN-12, -15 and -16 with M1.8 timed presentation, M3 free input and future visual/NPC systems.

### Non-negotiable premises

- The engine alone validates and commits canonical changes.
- An LLM produces **proposals**, never commits world truth.
- An output adapter renders only authorized semantic content; optional style changes are not canonical events.
- Deterministic tests must run without external models; old CYOA and M1.75/M1.8 stories must remain playable.
- A successfully parsed proposal is not proof that the narrative is consistent, playable or free of invented facts.

## General architecture backlog

| ID | Status | Question |
|---|---|---|
| OPEN-ARCH-01 | Open | Which exact Rust protocol API and extension points should be stabilized, and when? |
| OPEN-ARCH-02 | Open | Which asynchronous host/transport conventions will be needed without making the core async or renderer-dependent? |
| OPEN-ARCH-03 | Open | How should future story schema and save formats evolve while retaining compatibility? |
| OPEN-ARCH-04 | Open | How granular should adapter/presentation snapshots be for reconnection and replay? |

The earlier unstructured list lived in `docs/architecture.md#open-design-questions`. This catalog preserves those concerns and expands the unresolved generation questions; no existing accepted ADR is retroactively altered.
