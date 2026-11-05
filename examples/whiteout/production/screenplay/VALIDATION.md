# Validation, editorial findings and open production questions

Baseline **main `a4adba6`**, after merged PR #76. Sources are the executable
`examples/whiteout/story.yaml`, `text.yaml`, `scenes.yaml`, their DESIGN and
WALKTHROUGH, the original blackout draft and the real Bevy excerpt. Historical
`legacy-v1` is retained compatibility content, not a second story to adapt.

## Coverage audit

| Requirement | Evidence / scope |
| --- | --- |
| Complete canonical scenes | Nine context sections in the five screenplay chapters; PARITY maps each to its canonical ID. No new playable plot or renamed canonical context. |
| Decisions / consequences | All 35 qualified actions appear in the chapters and exact RULES ledger, including ordered failure outcomes, local retries, GOTO and timeouts. |
| Text / dialogue | All 57 narrative IDs have cinematic beats and individual parity rows. Existing quoted dialogue [C] is distinguished from adapted lines [A]. This is correspondence, not a text-equality requirement. |
| Cue traceability | All 95 authored cue instances have source paths in PARITY; RULES records predicates, durations, timer operations, input gates and effects. Reused latch text is not mistaken for duplicate cue identity. |
| State | All 30 initial globals and every authored assignment/calculation appear in RULES. Creative chapters describe effects rather than showing raw hidden counters to players. |
| Cast / locations | Four persistent named actors plus offscreen coast/searcher roles; nine stable location identities, including terminal vignettes, in INVENTORY. Connected mast is part of radio set; repeated hall is one set. |
| Assets / conceptual cueables | Every explicit `actor.*`, `loc.*`, `prop.*`, `fx.*`, `cam.*`, `anim.*`, `snd.*`, `music.*`, `voice.*` production ID referenced in cinematic chapters is inventoried. Conceptual operation names are not asset IDs. |
| Timing | Four entry deadlines, extension/reveal/removal, work versus playback, concurrent radio/gust, startup lock and cutoff cases documented below. No new cinematic time budget silently replaces canonical timing. |
| Original content integrity | `story.yaml`, `text.yaml`, `scenes.yaml` byte-identical to baseline; SHA-256 in PARITY. Original playthrough files and Rust implementation also unchanged. |
| Existing slice | SLICE-COMPARISON maps all six prototype nodes and its protected startup/preparation assumptions to full canonical requirements. |

A one-off local audit read the YAML and documents, checked counts/qualified
references, chapter-to-inventory references, source hashes and relative links.
RULES/PARITY are checked-in static reference documents, **not generated runtime
manifests**. No audit tool, compiler, grammar or required screenplay gate is added
to the repository. These checks prove correspondence, not cinematic quality.
Human review below is part of the validation, not replaced by counting IDs.

## Dramaturgical / timing review

- **Blackout:** four faces, injury, identical breath and two failed systems are
  staged without revealing the test-loop cause before the optional log. The
  adapted scene does not execute `begin` while the camera looks at the door.
- **Stores:** three-minute timer begins alongside seven-second briefing. Liner
  acts after old-rate exposure; treatment is not healing; braid is visibly one
  resource. The log grants knowledge, not isolation. Timeout loses untaken gear,
  not carried gear or lives.
- **Hall:** return variants can repeat whenever entry predicates hold. No global
  storm timer is invented. Recovered hands cannot erase the radio-first branch.
- **Generator:** four-second arrival consumes hidden coil budget. Warning and
  reveal at T=10000 overlap work, not restart it. Heat-pack extends by 60000ms.
  External diagram/fuse remain accessible because their authored guards do not
  require latch-open. An unisolated start is an offered failure outcome, not a
  prohibited action. Post-accounting cold determines outcomes.
- **Startup:** primer 1200 + ignition 1500 + heat 1800 = 4500ms presentation,
  against 4000ms logical work, under explicit sequential input lock. Completion
  exactly at D can succeed/cancel; selection at D loses; work beyond D is cut off.
  The prototype's protected start is deliberately not adopted as canonical.
- **Radio:** reserve starts at entry alongside six-second briefing; T=15000 gust
  checks `!radio_auto`, not merely whether a mug exists. Real reply contrasts the
  test loop. Manual success cancels deadline but preserves Lea's bearing duty.
- **Ridge:** eight-second briefing consumes 45-second clock. Coordinate failure
  stays local with clock intact. Help-Lea and help-Jonas have distinct full-rescue
  predicates; possessing a line does not silently coordinate on a one-person
  action. Flare retreat remains partial. Staying or timeout is explicitly fatal.
- **Endings:** full count, exact named empty place and four names without answers
  are visual parallels. Truth cards are gated by the optional log, not by a
  camera insert or an invented interrogation scene.

### Source reachability findings (retained, not silently repaired)

The context topology has no orphan contexts: opening/stores/hall/each repair/
ridge/full/partial/lost appear in the intended routes. Existing scripted routes
are concrete evidence, not an exhaustive proof over all possible action times.

Two ordered outcomes of `generator.work_latch_with_numb_hands` (assisted and
steady, outcomes 2 and 3) test cold **below 40000**, while the action requires
cold **at least 40000**. Its normal accounting adds a nonnegative interval at a
nonnegative rate; no warming effect occurs inside that action. Under legal time
observations these branches cannot be reached. Their eight cue definitions stay
in the reference matrix and reuse latch variants; no new gameplay path or code
change is proposed here. Numb failure and slow progress remain meaningful.

The timeout prose in generator/radio defensively preserves completed power/call.
Ordinary successful work already removes its deadline. Do not force a later
post-success timeout scene merely because the prose mentions retained success.
No new unreachable scene is created to illustrate that defensive case.

### Open production decisions, not invented facts

1. **Dialogue length:** seven-second stores briefing and 1.8-second feedback/
   ending cards cannot contain long naturalistic speeches. Screenplay proposes
   compressed lines, subtitles and overlap within current budgets. Table-read
   and subtitle legibility pass must determine VO lengths; any actual duration
   change needs explicit rule/timing review before implementation. Native brief
   cards can remain mostly visual. No claim of finished VO timing is made.
2. **Injured blocking:** canonical hall and assisted-latch lines imply supported
   movement. Proposed threshold bracing protects the ankle; rehearse geometry
   and contacts before locking rigs. Do not add a mandatory escort action or
   assume independent walking before the relevant outcome.
3. **Flare-enabled helper branches:** `help_lea`/`help_jonas` permit a partial
   result with a possessed flare even without a call, but do not consume that
   flag. Only `flare_retreat` explicitly consumes/fires it. Ending prose allows
   coastal pickup. Stage the helper outcome without inventing an extra fire
   action/effect; whether to depict an offscreen signal needs maintainer editorial
   review. This source ambiguity is recorded, not fixed by changing rules.
4. **Rescue geography / casting:** vehicle dressing, lens, distances, ages,
   accents and detailed costumes remain [A] production choices. No new named
   rescuer, personal relationship or sabotage motive is approved by this script.
5. **Media implementation:** rigs, voice playback/mixing, actual scene assets and
   complete rule-faithful Bevy mapping remain future work. The existing slice
   demonstrates concurrent media but reported local ALSA errors; production
   audio quality needs listening on target hardware.

## Verification record

Source/coverage/link audit and native story validation are run during this PR.
Full Rust formatting/build/lint/tests and actual existing CLI routes are recorded
here when complete. No new runtime feature, story format, screenshot automation,
asset generation or generic authoring framework is introduced by validation.

Current verification: in progress; final commands/results will replace this line
before the PR is marked ready for review.
