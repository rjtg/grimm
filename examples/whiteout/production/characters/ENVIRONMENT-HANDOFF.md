# Character requirements handed to #35

This is the cast-side handoff for the Environment and Cinematography Bible,
not that bible, an environment concept pack or a level implementation. All spatial
numbers below are **[A]** layout targets; final sizes/lenses remain **[O]**.
Use existing `loc.*`, `cam.*`, `prop.*` IDs from the full screenplay inventory.

| Location / shots | Cast and prop requirement | Rehearsal / acceptance target for #35 |
| --- | --- | --- |
| `loc.aurora_corridor`; B01, `cam.radio_insert`, `cam.injury_insert` | Four readable figures in torchlight; ankle and handset close-ups, Alex cast-neutral framing. | Sweep reaches each face without a new actor model; dark costumes separate from wall; protected leg visible when establishing injury. |
| `loc.emergency_stores`; S01–S04, `cam.stores_tray`, `cam.log_insert` | Low supported Jonas, Mara crouch and Alex reach; single braid between mug/spanner. | Shelf/seat height supports injured bracing; don't put tools beyond practical gloved reach. Source possession remains action-controlled. |
| `loc.dispatch_hall`; D01–D03, `cam.hall_routes`, `cam.line_insert` | Same four identities/route axes across returns; Jonas by duct, Lea remote if manually holding antenna. | Reuse actor marks, not two Leas. Warm-hand insert preserves injured Jonas in depth without implying recovery. |
| `loc.generator_annex`; G01–G08, `shot.controls`, `shot.starter`, `cam.coil_insert` | Alex's gloves operate visible external fuse/diagram; supported Jonas brace, Mara supports; no independent injured walk. | Rehearse frame/rail and protected-leg clearance, glove/lever contacts, primer reach; external fuse layout preserves actual guards. Do not hide a necessary grip behind camera edge/UI. |
| `loc.radio_cabin`; R01–R07, `cam.radio_axis`, `cam.jumper_insert` | Lea's same cap/headset profile and sustained wheel contacts, Alex nearby; installed mug gull visible. | Wheel height/radius fits shoulder and both gloved hands; cabin/mast are one clock. Audio-only Lea in remote views, never duplicate avatar. |
| `loc.ridge_crossing`; C01–C06, `cam.ridge_two_targets` | Jonas low/manifold, Lea high/shelter, Alex/Mara deciding between them; line/carry only accepted branch. | Clear separated rescue targets with snow; line routing and safe carry width; no composition implying a free sequential rescue. |
| `loc.rescue_cabin`; E01/E03/E04, `cam.manifest_four`, `cam.manifest_empty` | Four occupied places or exact named absence, splint retained if treated, exhausted same faces. | Camera match communicates count; partner carry and seat clearance; no background casualty resurrected for composition. |
| `loc.daylight_report`, `loc.aurora_exterior`; E02/E04/E05 | Same actor/equipment evidence, or complete cast absence. | Daylight preserves identities; log/tag/names not swapped. No unapproved spring survivor portrait. |

## Cast scale / lens / light constraints [A]

- Use the STYLE scale envelopes to block a shared neutral stand-in, then approve
  final scale once. Pixel height on differently arranged sheets (especially seated
  Jonas) is **not** a metric height reference.
- Test a wide four-person scene at the actual target window size with its bottom
  choice/subtitle panel. A proposed readability target is identification from
  silhouette at roughly 64px body height, but this is **not yet measured or proven**.
  Hood/headset, hem and weight distribution must survive reduced scale.
- In close-up, retain actor face/headwear landmarks; do not use a separate glamour
  portrait model. Alex unmasked close-ups wait for casting policy.
- Neutral reference swatches are not lit pixel targets. Torch, warm interior and
  snow bounce need continuous albedo/material identity, readable gloves and skin.
  Don't solve darkness by adding luminous wardrobe or changing ethnicity.
- A camera push may intentionally crop a whole body, but preserve the information
  needed for the active choice (injury, hands, line, wheel, remaining route). Put
  key contact action above UI occlusion; no new universal UI/layout framework.
- Rehearse manual hold under dialogue/gust and latch work under radio warning;
  camera motion must not cancel concurrent actor/prop performances.

Return unresolved contact/scale/lighting decisions to the character bible before
commissioning final assets. Do not alter guards, work times or casualty facts to
make a composition easier. #35 owns set/camera elaboration; #34 supplies cast
constraints only.


The [Environment and Cinematography Bible](../environments/README.md) now provides
concrete layout/camera proposals against these requirements, with inspection
findings and remaining contact/UI rehearsal gates. It does not lock open casting
or injury-side decisions from this Character Bible.
