# Environment continuity / review record

Baseline main **`6f46a94`**, merged PR #78. Canonical story/text/scopes and full
screenplay remain unchanged. Existing stable IDs are reused; new geometry,
dimensions, local axes, palette and the three establishing cameras are **[A]**,
not new game rules. **[O]** Human layout/art approval is still required.

## Coverage / evidence map

| Requirement | Artifact / review evidence |
| --- | --- |
| All canonical locations | Nine inherited `loc.*` IDs mapped to contexts/beats in ASSETS; seven specifications cover corridor, stores, hall, annex, radio, ridge and all three terminal settings. |
| Fixed accesses / spatial relation | PLANS station adjacency, five room/area diagrams, coordinate marks and named threshold registration. Stores one shared threshold; hall four fixed routes; no added gameplay traversal. |
| Blocking / continuity | Source actor identities, supported Jonas path/ankle clearance, one Lea at actual mast duty, correct world radio-west/engineering-east. Previous/next threshold movement table. |
| Cinematography | 21 inherited `cam.*`, three new [A] establishing references and four concrete `shot.*` associations; axis, gaze, transitions, timing overlap and decision framing. |
| UI visibility | Conservative planning around variable bottom panel, essential object/target regions and explicit crop/rehearsal requirements. Concept art is not claimed a tested game screenshot. |
| Lighting/weather | STATES maps source conditions/deadlines/outcomes to variants; no new timer, rate, power or weather gameplay. Local battery reserve is separate from restored supply. |
| Art references | Actual image artifacts, exact initial/edit prompts, original outputs, provenance hashes and selected/draft status in REFERENCES. No new character design. |
| Assets / sound | Existing prop/FX/sound identities, reusable new [A] module kit and one-off dressing; locations/cameras fully catalogued. |
| Bevy feasibility | Per-set/per-shot depth, layer sufficiency, optional 3D benefits, later animation needs; BEVY-HANDOFF preserves existing epoch/REPLACE/missing-binding contracts. |
| Production order | README starts with cast/door/contact/GUI rehearsal and shared kit, then interactive sets and terminal variants, not bulk polished assets. |

## Continuity problems caught before asset production

1. **Station sheet v1:** small plan omitted south entrance and exterior had warm
   windows/exhaust despite initial blackout. Selected v2 restores fourth threshold,
   removes powered exterior cues and keeps the same hall architecture/bench.
   Original and edit prompt retained; the old draft is not the selected reference.
2. **Generator v1:** model invented a padlock and warm overhead lamp. These would
   imply a noncanonical key requirement/normal supply. Selected v2 uses an iced
   mechanical latch, accessible external tagged fuse and unlit overhead fixture.
   Its small engine-face gauge is illustrative, **not** relocation of the source
   coil gauge/readout from the entry shutter behind camera. Final labels/diagram
   art must be controlled, not copied from generated typography.
3. **World/local axis ambiguity:** hall east connects annex south, hall west radio
   south by registered room rotation. PLANS now states local drawing frames and
   joins; no fictional extra corridor is added to repair coordinate labels.
4. **Radio / ridge continuity:** radio desk-left/wheel-right is one connected set;
   ridge uses world west-left mast and east-right manifold. Empty layout studies
   do not remove the required operator or grant auto mode. Ridge draft's invented
   outside shelter door is removed in selected v2, with radio sheet as continuity
   input and the existing operator bay/wheel made readable.
5. **Stores draft:** tray was on floor rather than proposed .85m work height, and
   rear window suggested clear daylight. Selected v2 preserves
   same objects/one threshold and corrects tray height and storm/night exterior.
6. **Concept versus gameplay crop:** full art uses architecture/floor space that
   may lie behind variable captions. CINEMATOGRAPHY explicitly requires reframe
   and actual GUI checks rather than pretending references are already playable.

## Character and source-rule review

- Same four actor roots and existing cast palette/silhouettes; empty concepts
  deliberately avoid inventing faces. Proposed left injury side stays [O] and
  anatomical, not a mirrored screen-side shortcut.
- Clear proposed 2m transfer/door targets and low support/handwork marks are
  checked for conceptual placement, not medically safe final contacts. Trays,
  rack/duct/bench don't intentionally fill the route. Final body/rig/prop sizes
  and actual layered/3D camera occlusion require rehearsal.
- External fuse placement preserves guards that do not require latch-open.
  No padlock/key, new tool cache, permanent ridge rail or alternative safe route.
- Cold grip, work-duration cutoff, source deadline start/reveal/extension/removal,
  distinct rescue predicates and exact survivor outcomes remain canonical.
  Effects are not inferred from weather, lighting or animation duration.
- Repeated hall uses same geometry/bench/doors; heat changes light/acting, not
  repair order or healed ankle. Manual radio keeps one Lea physically required.
- Terminal frames keep exact absence, log condition and same station silhouette.
  Coastal-helper flare imagery is still the screenplay's open source question;
  no invented consuming/firing action resolves it here.

## Paper-fit check [A] — not a rig/collision benchmark

The coordinate plan was inspected against the Character Bible's proposed adult
scale ranges (Alex up to 1.82m, Mara 1.74m, Jonas 1.88m, Lea 1.76m), including
protected low-leg space rather than only healthy standing silhouettes.

| Working pair / path | Planned fit / adjustment | Remaining production check |
| --- | --- | --- |
| Shared transfer routes | Proposed 2m clear lanes/thresholds leave room for a provisional ≤1.8m partnered support envelope; tray/rack/bench stay outside main lane. | Final bodies, carried posture, turning radius and actual margins; not a building/medical-safety certification. |
| Stores tray / rack / log | Alex steps to each station; tray (2.1,3.3,z.85) can be reached from about (2.1,2.8) with a short bend; rack/terminal have adjacent working marks, not a 2m standing reach. | Hand grip/prop and glove occlusion; no new pickup duration. |
| Generator latch / fuse | Latch working mark refined to (2.2,1.9), about .5m south of face at y2.4; fuse reached by east shift to (3.4,2.0), not through casing. Control heights .9–1.2m. | Prime/lever/fuse articulation, body contact and interrupted pose. |
| Hall warmth / ankle | Duct outlet (0,5.0), warm mark (.5,5.0) beside protected foot zone near (1,5.0); west door opening ends south of this mark. Bench/doors unchanged. | Anatomical side, exact foot/body width and safely clear stance with final rig. |
| Radio wheel | .5m operator-to-wheel distance at z1.1, radius .32m, allows a leaning gloved/shoulder study consistent with Lea sheet. | Actual shoulder/hand contacts, cable and safe wheel catch; not proof of physical ergonomics. |
| Ridge targets | Manifold (6.5,2.2) and shelter (1.5,5) are about 5.7m apart in the proposed excerpt, with separate approach lines; no single easy pass rescues both. | Final terrain occlusion, line/carry route and UI-safe silhouettes; no new logical travel cost. |

This check finds plausible working/transfer room and corrects known reach/axis
ambiguities before assets. It does **not** claim every skeletal trajectory or
camera crop is already implemented. Those remain explicit subsequent rehearsal
gates; the task here delivers the spatially specified production plan.

## Open production decisions [O]

Final metric dimensions/door/rail/wheel contacts, wind vector and repeated wear
masks, actual lens/GUI framing, actor casting/injury side, material/texture/export
budgets, sound/voice mix and final pickup vehicle dressing require production
review. No generated concept is a metric scan, a final rig/level or approved art.
Distant/spring/report images can remain layered vignettes; no new season/room
simulation is justified. Later assets must be derived from the reviewed selected
revision, not unrelated regenerations.

## Verification record

Local results on 2026-11-05, Rust/Cargo 1.98.1:

| Check | Result |
| --- | --- |
| `cargo fmt --all --check` | Passed. |
| `cargo build --workspace --locked` | Passed. |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Passed, no warnings. |
| `cargo test --workspace --locked` | **258 passed**, zero failed, including existing CLI and 37 Bevy WHITEOUT tests. |
| Local documentation/reference audit | All nine inherited locations specified; all referenced prop/sound/FX IDs inherited; 21 existing + three proposed camera IDs catalogued; every new `env.*` reference catalogued; local links/PNGs valid. |
| Integrity | Canonical story/text/scopes and four original Character Bible reference PNGs byte-identical to main `6f46a94`; all image hashes match REFERENCES. |

Build/test/Clippy used `CARGO_PROFILE_DEV_DEBUG=0 CARGO_BUILD_JOBS=2`.
The local audit is a one-off reference/integrity check, not a new repository
compiler, asset pipeline or mandatory screenplay-validation subsystem. Source
behavior is covered by the full existing suite; no art-quality assertions or
new reversible-document tests were added. No new GUI/window/screenshot test is
claimed: the runtime is unchanged, and this task excludes screenshot automation. No source-file LSP diagnostics
apply to this Markdown/text/PNG-only task; no new compiler, art test gate,
authoring schema or runtime code is added. Image consistency is agent visual
inspection, not something a hash/count script can prove.
