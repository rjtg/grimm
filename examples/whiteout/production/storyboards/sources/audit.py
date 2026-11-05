"""Check storyboard artifacts and canonical narrative/action coverage, not art quality.
Run from repository root. Requires PyYAML and Pillow for this optional audit.
"""
from pathlib import Path
import json
import re
import xml.etree.ElementTree as ET
import yaml
from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
STORY = ROOT.parents[1] / 'story.yaml'
shots = json.loads((ROOT / 'sources/shots.json').read_text())
world = yaml.safe_load(STORY.read_text())
assert len({s['id'] for s in shots}) == len(shots)
assert {s['scene'] for s in shots} == {c['id'] for c in world['contexts']}
for s in shots:
    for field in ('camera', 'location', 'beat', 'time', 'state', 'audio', 'motion', 'next'):
        assert s[field], (s['id'], field)
    ET.parse(ROOT / s['editable'])
    with Image.open(ROOT / s['panel']) as image:
        assert image.size == (1600, 900), s['id']
        image.verify()

def walk(value):
    if isinstance(value, dict):
        yield value
        for item in value.values():
            yield from walk(item)
    elif isinstance(value, list):
        for item in value:
            yield from walk(item)

narratives = {v['narrative_id'] for v in walk(world) if 'narrative_id' in v}
actions = {c['id'] + '.' + a['id'] for c in world['contexts'] for a in c.get('actions', [])}
assert narratives <= {n for s in shots for n in s.get('texts', [])}, 'Missing narrative drawings'
assert actions <= {a for s in shots for a in s.get('actions', [])}, 'Missing action drawings'
for page in ROOT.rglob('*.md'):
    for target in re.findall(r'\]\(([^)]+)\)', page.read_text()):
        if '://' not in target and not target.startswith('#'):
            assert (page.parent / target.split('#')[0]).exists(), (page, target)
for path in (ROOT / 'sheets').glob('*.png'):
    with Image.open(path) as image:
        image.verify()
cues = sum('cue_id' in v and 'kind' in v for v in walk(world))
print(f'PASS: {len(shots)} drawn panels, {len(list((ROOT / "sheets").glob("*.png")))} sheets, '
      f'{len(world["contexts"])} contexts, {len(narratives)} narratives, {len(actions)} actions, {cues} cues.')
