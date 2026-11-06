# Generator slice base assets — in progress (#40 / #26)

Baseline main `3dc9b8a` after #81. This increment imports a single spatial annex
and reusable Alex/Mara/Jonas GLBs into the existing WHITEOUT Bevy application.
The draft is not yet a passed graphical playtest; final verification follows.
No full-world production, story-rule rewrite or generic adapter is included.

## Existing inventory audit

Before this change `stage.rs` built coloured Sprite rectangles in pixel units,
including two simplified crew sprites, flywheel, fuse, starter and shutter. No
imported model, Alex model, material texture or limb hierarchy existed. Camera
aliases were `shot.wide`, `shot.hall`, `shot.controls`, `shot.starter`; motion,
audio instances, epoch replacement, selection/deadline arbitration already worked.
The three existing original mono 22050 Hz WAVs (`wind`, `motor`, `alarm`) are kept.
Character/environment/reference images and storyboard panels are production
references, not runtime model assets. #39's whole-game inventory is not closed by
this slice; the manifest here records concrete produced and still missing work.

## Production / import

```bash
python3 crates/whiteout-game/assets/production/sources/build_assets.py
cargo run -p whiteout-game --locked
```

Python 3 standard library only; source exporter version 1. glTF 2.0 GLB embeds
mesh buffers, PBR materials and a seeded 64×64 wear PNG. No external file or
online service is required by the models. New WAVs are deterministic signed
16-bit mono 22050 Hz PCM. Checked-in exports load after a fresh checkout without
running the exporter. Bevy 0.20.1's native glTF/WorldAssetRoot and PBR are used;
asset paths resolve from this crate's asset directory, independent of shell cwd.

Coordinates: metres, right-handed, +Y up, +Z south/front. Room 4.5×5 m; north
engine, east external fuse, southwest support bench, sole south threshold. The
south camera wall is a deliberate cutaway; threshold jambs/header remain real.
No unrelated replica set is spawned for inserts. Named room pivots connect to
the existing cue Motion jobs; rigid named limb pivots support deterministic
breath/look/work/support animation on the same scene clock, without changing
narrative truth. No skinned final rig or general animation engine is introduced.

## Ownership and rights

All geometry, procedural wear pixels and new PCM are original authored source
created for this repository. The repository currently declares no distribution license; that maintainer decision remains open. No third-party meshes, textures, recordings,
fonts or generated bitmap concepts are embedded in these GLBs. Existing own
reference art guides silhouette/material direction; it is not copied into meshes.
No DCC executable, cloud account or asset-store download is needed to reproduce.

Provisional low-poly design follows #77 screenplay, #78 cast, #79 environments
and #81 storyboard, with the existing prepared-route #76 gameplay unchanged.
It is a reusable production base, not photorealistic final art or full canonical
story parity. Exact rig contacts, final character likeness and sound mixing remain
production review items. See the adjacent manifest for IDs and shot mappings.
