"""WHITEOUT artifact integrity + fresh deterministic source reproduction. No GPU."""
import hashlib
import importlib.util
import json
from pathlib import Path
import shutil
import struct
import tempfile
import wave

ROOT=Path(__file__).resolve().parents[1]
manifest=json.loads((ROOT/'manifest.json').read_text())
for asset in manifest['assets']:
    path=ROOT/asset['path'];data=path.read_bytes()
    assert hashlib.sha256(data).hexdigest()==asset['sha256'],path
    if path.suffix=='.glb':
        magic,version,length=struct.unpack_from('<4sII',data)
        assert (magic,version,length)==(b'glTF',2,len(data)),path
        count=struct.unpack_from('<I',data,12)[0];doc=json.loads(data[20:20+count])
        assert doc['asset']['version']=='2.0'
        assert doc['meshes'] and doc['textures'] and doc['images'][0]['mimeType']=='image/png'
        assert all('uri' not in b for b in doc['buffers']), 'external buffer'
        assert all('uri' not in b for b in doc['images']), 'external texture'
        assert {n['name']for n in doc['nodes']}==set(asset['nodes'])
        for mesh in doc['meshes']:
            for primitive in mesh['primitives']:
                assert set(primitive['attributes'])=={'POSITION','NORMAL','TEXCOORD_0'}
                assert doc['accessors'][primitive['indices']]['count'] % 3 == 0
        if asset['id'].startswith('actor.'):
            assert all(any(n['name']==name for n in doc['nodes'])for name in ['bone.head','bone.chest','bone.arm.R','bone.elbow.R','bone.leg.L','bone.knee.L'])
    else:
        with wave.open(str(path))as source:
            assert (source.getnchannels(),source.getsampwidth(),source.getframerate())==(1,2,22050)
            assert source.getnframes()>1000
spec=importlib.util.spec_from_file_location('whiteout_assets',ROOT/'sources/build_assets.py')
module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
with tempfile.TemporaryDirectory(prefix='whiteout-assets-')as temp:
    module.ROOT=Path(temp)/'production';(module.ROOT/'models').mkdir(parents=True);(Path(temp)/'audio').mkdir()
    for name in ['wind','motor','alarm']:shutil.copyfile(ROOT.parent/'audio'/f'{name}.wav',Path(temp)/'audio'/f'{name}.wav')
    module.annex()
    for name in ['alex','mara','jonas']:module.actor(name)
    module.sounds();module.manifest()
    assert (module.ROOT/'manifest.json').read_bytes()==(ROOT/'manifest.json').read_bytes()
    for asset in manifest['assets']:
        assert (module.ROOT/asset['path']).read_bytes()==(ROOT/asset['path']).read_bytes(),asset['id']
print('PASS: four self-contained GLBs, embedded wear textures, seven PCM masters; fresh exports byte-identical.')
