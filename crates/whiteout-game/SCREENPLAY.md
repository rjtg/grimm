# WHITEOUT — the generator annex (#70)

An authored, prepared excerpt from the canonical text adventure, **not a new
relay-repair story**. Runtime graph: [assets/bootstrap.yaml](assets/bootstrap.yaml).
Text bindings: `src/content.rs`; staging: `src/cueables.rs` and `src/stage.rs`.
All times below are milliseconds **relative to the current node entry**. GRIMM
supplies the complete sheet; WHITEOUT owns clock, input, rules and presentation.

## Set, characters and atmosphere

A fixed side-on annex: corrugated blue wall, dark floor, generator housing,
flywheel, amber starter, red tagged de-icer fuse and access shutter. Jonas wears
orange; Mara teal. Their persistent named stage entities move towards the
controls. Snow crosses the exterior; wind continues under the dialogue. These
are intentionally simple geometric placeholders, not finished character art.
Lea speaks over the radio; she is not a third visible character.

## Playable screenplay

| Node / time | Action, camera and sound | Implemented cues |
| --- | --- | --- |
| `DISPATCH_APPROACH` / 0 | Wide annex, wind. Mara keeps Jonas near the door; Lea has the backup set. Prepared setup resets this attempt. | `setup`, `exterior`, `wind`, `approach` |
| 4000–10000 / 6000–10000 | Jonas approaches, Mara follows. The camera holds; both movements overlap the wind. | `jonas_moves`, `mara_moves` |
| 12000–13600 | Camera closes on the fuse diagram. Jonas recognizes his maintenance bypass: the short is in the de-icer, not the engine. | `diagram_shot`, `diagram` |
| 18000 | Enter the annex automatically through a hidden edge. | `enter` → `GENERATOR_ANNEX` |
| `GENERATOR_ANNEX` / 0 | Wide shot, shutter coil has 30 seconds; its display is dark. Pull the tagged fuse over two seconds. Poses are restaged explicitly. | `wide`, `arrival`, `fuse_motion`, `wind`, actor poses |
| 2000 | De-icer becomes isolated; Jonas exhales. The timed effect is separate from the lever animation. | `isolate`, `isolated` |
| 4000 | Enable **1 Leave the annex** / **2 Start the generator**. | `ready`; exposed edges `abandon`, `start_generator` |
| 10000 | Lea calls after ten seconds. Reveal the existing countdown (20 seconds left), beep once and move to the controls close-up concurrently. No timer reset. | `radio`, `alarm`, `closeup` |
| 30000, no selection | Coil expires; hidden edge replaces all old work and reaches the sealed annex. Timeout wins at exact equality. | `timeout` → `ANNEX_SEALED` |
| `GENERATOR_STARTUP` / 0–1200 | Ordinary story node: close-up, primer line, starter lever turns, wind. No input. | `closeup`, `prime`, `lever`, `wind` |
| 1200–2700 | Cylinders catch. Flywheel starts rotating and motor sound loops under the ignition text. | `ignition`, `motor_sound`, `flywheel` |
| 2700–4500 | Warm air clears the duct; Mara calls the player back to Lea. | `heat` |
| 4000 / 4500 | Commit powered generator, then traverse the hidden return edge. Playback duration is **not** an effect-completion signal. | `stable_power`, `return` → `DISPATCH_WARM` |
| `DISPATCH_WARM` / 0–6000 | Camera moves towards the hall; wind, motor and wheel continue as new instances. Green lamp, warm-air text; Jonas stops shivering. | `wide`, `motor`, `wheel`, `return`, `settled` |
| `DISPATCH_COLD` or `ANNEX_SEALED` / 0–9000 | Seal annex, shutter descends; no motor, red lamp. Abandonment and expired-coil text differ; timeout adds alarm. Mara brings the player back. | `seal`, `wide`, `shutter`, `return` or `timeout`, `settled` |
| Either settled outcome | **1 Replay the excerpt**. A new approach entry resets the prepared attempt. | `restart_slice` |

Earliest complete powered route: **32.5 seconds**; abandonment: **31 seconds**;
unanswered timeout: **57 seconds**. Waiting at outcomes is unbounded. Reading and
making a decision may extend the powered route. These are scene-clock budgets,
not frame-exact wall-clock performance promises.

## Concrete binding / asset map

| Presentation ID | Concrete resource / operation |
| --- | --- |
| `canon.*` | Authored title/body and explicit input/countdown gate in `content.rs`; latest caption only |
| `shot.wide` | Camera `(0,100)`, scale `1` |
| `shot.controls` | Camera `(40,90)`, scale `.72` |
| `shot.starter` | Camera `(100,80)`, scale `.62` |
| `shot.hall` | Camera `(-120,0)`, scale `1`; same connected placeholder set |
| `move.jonas`, `move.mara` | Named stage roots to `(-210,70)`, `(-285,65)` |
| `move.deicer`, `move.starter` | Fuse/starter rotations to `-1.1` / `1.1` radians |
| `move.shutter` | Shutter root to `(310,65)` |
| `motor.spin` | Flywheel rotation from current scene time |
| `sound.wind`, `sound.motor` | Embedded `assets/audio/wind.wav`, `motor.wav`; loop |
| `sound.alarm` | Embedded `assets/audio/alarm.wav`; one-shot |

`duration_ms` controls WHITEOUT's linear motion interpolation where applicable.
It neither acknowledges success to GRIMM nor causes gameplay effects. Independent
authored effect cues commit game truth. Audio length comes from decoded audio;
missing bindings, missing stage objects and missing source data stop the entry
with a visible diagnostic. Fixed embedded WAVs are decoded in tests.

All currently executed cues have `(epoch, cue_id)` instance identity. A caption
cannot erase audio/camera work. REPLACE clears the active descriptor collection,
despawns obsolete audio/motion entities (stopping their playback), resets the
fixed set and installs explicitly authored destination poses. Completed one-shot
IDs remain deduplicated until replacement; fixed set entities and three source
assets are reused. No frames, cue ACKs or new graph lifecycle are introduced.

## Canonical parity and deliberate adaptation

Sources: [story.yaml](../../examples/whiteout/story.yaml), `generator` context,
and [text.yaml](../../examples/whiteout/text.yaml).

| Original | Slice | Parity / adaptation |
| --- | --- | --- |
| `generator.arrival`, `generator.controls` | approach, diagram, arrival | Condensed prose; same annex, damaged de-icer and Jonas's bypass. Approach recap is added staging outside the coil clock. |
| `generator.isolated` / `isolate_deicer` | fuse movement + effect at 2000 | Original text preserved; two-second action enacted as an already chosen part of the prepared route. |
| `generator.radio_warning` | radio at 10000 | Same ten-second warning; dark countdown becomes visible without resetting the 30-second coil budget. |
| `generator.startup_prime`, `startup_ignition`, `startup_heat` | startup captions | Original text preserved verbatim apart from YAML whitespace; regression-tested. |
| `start_generator` | start edge + effect at 4000 | Isolated de-icer required; success powers generator. Four-second work, 4.5-second staged transition. |
| `abandon_generator`, timeout | cold / sealed outcomes | Engineering access lost; stopped generator stays stopped. Sealing never stops a generator that already runs. |
| `dispatch.heat_returns` | warm return | Same warm-air consequence for Jonas; shortened presentation around excerpt ending. |

The fixture begins **after** opening the latch, inspecting controls and treating
Jonas, with warm enough hands. Full cold/exposure accounting, inventory/tools,
heatpack extension, alternative repairs, the unisolated-overload path, medic bay,
radio and final escape are **not ported**. Preparation is explicit, not a claim
of full canonical rule parity. The former relay demo remains only as frozen
regression fixtures and old handler bindings.

One gameplay simplification remains: a selection strictly before the coil
boundary enters a protected startup node and replaces that deadline. Unlike the
canonical duration-resolved engine, this excerpt does **not** truncate a
four-second start chosen just before expiry. Implementing canonical near-deadline
work truncation belongs to a deliberate later game-rule adaptation; do not infer
it from animation completion. The hall is a camera reframe of the connected
placeholder set, not another finished location. Audio is synthesized placeholder
sound, with no voice acting. Save/Restore (#63) is explicitly deferred by this
assignment; code generation (#75) and full migration remain separate.

## Graphic-novel pilot overlay (#83)

The pilot retains the same prepared route, authored effects, decision readiness,
30-second deadline and outcome edges. Only WHITEOUT presentation bindings and
cues change. `art.*` / `pose.*` are ordinary concrete presentation IDs, not new
GRIMM instructions. The existing host scheduler remains their only time source.

| Storyboard beat | Illustrated treatment |
|---|---|
| G01 | Empty annex master + registered machine, bench and three cast cutouts |
| G04 | Wiring/isolator detail; connected and isolated variants |
| G06–G08 | Three latch inserts at approach seconds 6, 8, 10; open machine crossfade |
| G16–G17 | Same running deadline/radio caption; restrained red warning pulse and reframing |
| G18 | Primer insert, work pose, existing lever/motor cues |
| G19–G20 | Wide reframe, powered-state amber crossfade only after the power effect |
| G23–G24 | Withdrawal poses, descending shutter behind crew after 1.2 seconds |

The room, cast and props use the same art in staged and layered modes (F2).
Layered mode adds bounded background/foreground parallax; contacting props and
cast share depth. Breathing and mechanical vibration are subpixel transforms,
not independent animation clocks. `move.starter`, `move.deicer` and `motor.spin`
retain logical presentation anchors; inserts/state variants/vibration replace
literal spinning geometry. The latch sequence illustrates the previously
prepared latch, not a newly introduced gameplay test.

Alex's masked face treatment and Jonas's protected left splint remain provisional
production choices from the bibles. These drawings do not resolve those open
canon questions. See [asset sources and import](assets/graphic-novel/README.md).
