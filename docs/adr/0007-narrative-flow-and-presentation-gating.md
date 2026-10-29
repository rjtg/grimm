# ADR-0007: Narrative flow and presentation-gated interactions

Status: Proposed
Date: 2026-10-29

## Context
M1.75 introduced engine-owned timed decisions with an explicit `AwaitingReady` / `Ready` lifecycle. The CLI currently acknowledges Ready once the prompt has been printed and flushed. That is **not** evidence that a player has read a long text, or that a future renderer has finished voice acting, cinematic camera movements, animations or comic-panel sequencing. A countdown can therefore start during required storytelling, stealing time from an interaction the player cannot yet reasonably act on.

Authors can split a long presentation into a story node with one fake Continue choice followed by a timed node. That workaround confuses narrative pacing with meaningful agency, adds repetitive graph nodes and forces explicit input even when a visual renderer could advance after an authored sequence. Conversely, automatic continuation without a host acknowledgement could skip unseen narrative content. Both are needed, but neither should introduce renderer authority over game rules.

## Proposed decision

1. **Separate narrative presentation from actionable interaction.** A scene/segment can be narrative-only or interactive. Both remain semantic/engine concepts independent of renderer mechanics and physical world locations. An interactive node may itself include a required presentation phase before choices become available.
2. **Allow engine-owned narrative-only continuation.** An authored narrative-only segment may have a validated successor without an artificial gameplay choice. It advances only following an explicit, revision-scoped, authorized presentation-completion signal from the host, never from elapsed wall time or merely constructing an output DTO. Explicit player-paced Continue remains supported as an alternative.
3. **Distinguish presentation-complete from interaction-ready.** Completion of necessary storytelling and actual availability of offers are separate facts. A decision deadline begins only after both are satisfied and the host sends the lifecycle input accepted by the engine; the existing M1.75 `Ready` model may be extended or reused without inventing a second clock. No countdown is consumed during reading, required dialogue, animation or camera sequences.
4. **Keep dependency direction unchanged.** Output adapter/renderer reports a typed acknowledgement to the composition root/host via the presentation boundary. The host correlates the report with the active presentation/interaction revision and proposes an engine lifecycle event. Renderers cannot invoke engine methods, mutate narrative state, choose transitions, or resolve timeouts. A renderer acknowledgement is an observation, not a canonical story command.
5. **Make asynchronous acknowledgement safe.** Stale, duplicated, late or out-of-order completion/ready events must not change a later scene, restart a deadline, or resolve a continuation twice. The engine validates the successor and commits an authorized non-choice transition atomically. Prevent unbounded automatic chains/cycles; do not build a general-purpose timeline scheduler.
6. **Define interruption semantics.** A disconnected frontend or failed presentation must not silently skip content or activate unseen timers. Save/load and pause/resume preserve or explicitly reconstruct the pending presentation/readiness phase with deterministic replay and documented compatibility rules. An unacknowledged timed choice may be displayed but not executed before readiness, consistent with engine validation.
7. **Provide renderer-neutral fallbacks.** Text CLI should allow explicit reader acknowledgement (e.g. Enter) after required narrative content and before starting a short timer. A future visual renderer may acknowledge completion of required sequences. Presentation is allowed to be instant if a host can genuinely establish readiness; do not impose a mandatory pause on every untimed CYOA node.
8. **Stage minimally in M1.8.** Prefer limited story/schema extensions and explicit typed protocol events over a full cutscene API, cinematic sequencing framework, generic animation DSL, ECS, or new engine-to-renderer dependency. Preserve existing M1/M1.5/M1.75 story behavior by default.

## Consequences and acceptance
- A text-heavy timed scene can present its text and wait for a user acknowledgement before beginning the decision countdown. No unread text is timed.
- A narrative-only scene can advance to a validated successor when its required presentation completes, with no invented meaningful choice and no renderer-authoritative story mutation.
- Delayed/duplicated renderer callbacks are harmless; replay, restore, interaction revisions and world/decision time remain deterministic.
- The same semantic narrative/interaction works through a CLI and a mock visual renderer whose presentation completion arrives later.
- This does not mandate fully automatic cutscene playback; authors can intentionally choose player-paced continuation.

## Alternatives considered
- **Always use a single Continue choice between scenes.** Safe as a fallback, but makes non-interactive cinematic progression depend on artificial player input and cannot gate an interaction's own long presentation cleanly.
- **Start the countdown at node entry or output flush.** Conflates delivery with consumption/presentation completion and steals decision time.
- **Allow renderer to call the engine directly.** Breaks the established protocol and authority boundaries.
- **Start all content only after an arbitrary fixed delay.** Renderer-dependent, unreliable for voice/cameras/reading speed and non-deterministic.
- **Build a full cutscene timeline engine now.** Unnecessary before M2/M7 and incompatible with the narrow milestone.
