# Drawing sources and provenance

The 107 indexed panels are original programmatically drawn WHITEOUT blocking
sketches. `sources/draw_boards.py` constructs perspective rooms, cast silhouettes,
props, pose variants, arrows, camera plans and decision overlays as editable SVG.
`rsvg-convert` exports PNGs. No screenshots or extracted third-party art were used.
The 21 contact sheets embed those same vector drawings; they are not new shots.
DejaVu Sans is requested from the local system; no font files are bundled.

`sources/shots.json` is the editorial shot ledger; `sources/publish_pages.py`
rebuilds the Markdown galleries/index from it. This is optional WHITEOUT source
art tooling, not a generic authoring product or runtime pipeline. References are
the unchanged own character sheets `../characters/references/*-v1.png` and selected
environment sheets `../environments/references/` (station-dispatch-v2,
generator-v2, radio-v1, ridge-v2 and stores-v2). Story/rules remain unchanged.

## Built-in image generation

Two supporting pencil development studies were made with the built-in imagegen
tool, using the existing own cast/environment sheets as visual inputs:

| Output | Exact prompt | References |
| --- | --- | --- |
| `panels/g01-artist-study.png` | `sources/generator-style-prompt.txt` | Alex, Mara, Jonas; generator-v2 |
| `panels/r01-artist-study.png` | `sources/radio-style-prompt.txt` | Alex, Lea; radio-v1 |

Tool outputs were copied unchanged; they are alternate development studies shown
on the generator/radio pages, not extra coverage or canonical reference replacements.
The backend model/version and seed were not supplied by the tool; regenerating
from the saved prompt need not produce identical pixels. The checked-in originals
are the reviewable result. No external image-search inputs were used.

## Reproduce the deterministic boards

From repository root (Python 3 and system `rsvg-convert`):

```bash
python3 examples/whiteout/production/storyboards/sources/draw_boards.py
python3 examples/whiteout/production/storyboards/sources/publish_pages.py
python3 examples/whiteout/production/storyboards/sources/audit.py
```

The audit additionally uses PyYAML and Pillow to read canonical YAML and verify
PNG integrity. These are optional art-audit tools, not Cargo/runtime dependencies.
The individual SVG originals can be opened or printed without running Python;
print one full panel per landscape page for readable metadata. Native SVG sheets
are also printable, with full notes available in each scene gallery.
