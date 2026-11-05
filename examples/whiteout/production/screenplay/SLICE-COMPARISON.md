# PR #76 comparison — keep the proof, restore the whole game

Reference: [implemented excerpt screenplay](../../../../crates/whiteout-game/SCREENPLAY.md),
[actual playtest](../../../../crates/whiteout-game/PLAYTEST.md), graph
`crates/whiteout-game/assets/bootstrap.yaml`. Main baseline `a4adba6`.
The excerpt is evidence of real camera/audio/animation work, not the canonical
full adventure or a grammar to which all later scenes must conform.

| Concern | Existing Bevy excerpt | Complete screenplay / canonical requirement |
| --- | --- | --- |
| IDs | `DISPATCH_APPROACH`, `GENERATOR_ANNEX`, `GENERATOR_STARTUP`, `DISPATCH_WARM`, `DISPATCH_COLD`, `ANNEX_SEALED`. | Preserve canonical contexts `blackout`, `stores`, `dispatch`, `generator`, `radio`, `crisis`, three endings. Do not relabel excerpt nodes as canonical ones without a future explicit mapping. |
| Preparation | Latch opened, controls known, warm hands/treatment assumed. | Stores decisions, cumulative progress, all cold failures, optional log and insulated lever are real branches. |
| Prelude | 18-second approach outside the coil clock. | No added free approach budget in canonical generator entry. Filmic arrival fits its existing four seconds; any bridge must respect zero-cost travel and entry deadline. |
| Isolation | Scripted fuse animation/effect at annex T=2000. | A two-second player action, offered only with controls known and not yet isolated; diagram or log grants knowledge separately. |
| Start availability | Requires de-icer already isolated. | Requires latch open and power not done; unisolated attempt is offered but resolves to overload. |
| Start commit / cutoff | Effect after four seconds, protected startup replaces coil deadline on timely selection. | Action resolves once at selection with authored duration/cutoff. Late work cannot create power. Canonical success cancels timer immediately; presentation reveals result under input lock. |
| Warning | Radio + camera + alarm at T=10000; reveals current budget. | Retain overlap and reveal-without-reset; add real heat-pack +60000 extension and skip pending warning after success. |
| Input | Explicit panel gates and keyboard offers. | Preserve sequential startup lock and cost floor; timed latch release can overlap media. No cue ACKs are added to graph core by this script. |
| Environment | Geometric annex, snow, two coloured actor roots. | Reuse named identities; expand to full set layouts, injured/support/cold poses, Lea/manual mast and Alex hands. Finished art is not required to validate rules. |
| Return | “Hall” is another camera framing of the same set. | Reusable physical dispatch hall with changed conditions, two route doors, warm hands and later ridge decision. |
| Outcomes | Warm, cold, sealed excerpt endpoints; replay restarts prepared setup. | Nine canonical contexts, three terminal endings, named survivor differences. Do not reset inventory/order on hall return or add replay as an in-story action. |
| Media lifecycle | Concurrent cue-owned audio/motion; REPLACE despawns old work; missing resources visibly fail. | Retain these guarantees for new bindings. A dialogue caption cannot replace all active media; no implicit duration-success contract. |
| Sound | Original synthetic wind, motor and alarm; local ALSA errors noted. | These remain placeholders. Add distinct loop/live reply, crew speech, wheel strain and branch-aware ending beds; target-device listening pass still needed. |

## Reusable production pattern

The excerpt already separates authored graph timings, concrete content/staging
bindings, typed gameplay rules, ECS truth, active media instances and the UI.
Its separate screenplay beat table and source-parity notes are retained here,
expanded with [C]/[A]/[R] labels, full branch coverage and inventories. Concrete
`shot.*`, `move.*`, `motor.spin`, `sound.*` references are listed in FORMAT and
INVENTORY; proposed `actor.gesture` etc. are conceptual categories, not newly
implemented handlers or GRIMM variants.

An ordinary transition node can host startup or a threshold camera bridge in a
future graph adaptation, then traverse with the current epoch. It cannot quietly
replace canonical work cutoff with a protected cutscene. Equivalent effects,
conditions, timing and outcome preservation must be demonstrated first. This
PR changes neither implementation, story data nor architecture decisions.
