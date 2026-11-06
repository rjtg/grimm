from pathlib import Path
import json,collections,re
r=Path('examples/whiteout/production/storyboards'); ss=json.loads((r/'sources/shots.json').read_text())
locs={'blackout':'loc.aurora_corridor','stores':'loc.emergency_stores','dispatch':'loc.dispatch_hall','generator':'loc.generator_annex','radio':'loc.radio_cabin','crisis':'loc.ridge_crossing','ending_full':'loc.rescue_cabin','ending_partial':'loc.rescue_cabin','ending_lost':'loc.aurora_exterior'}
for s in ss:
 s['location']='loc.daylight_report'if s['view']in ['report','report_partial']else('loc.dispatch_hall'if s['view']=='stay'else locs[s['scene']])
 s['panel']='panels/'+s['id'].lower()+'.png'; s['editable']='panels/'+s['id'].lower()+'.svg'
(r/'sources/shots.json').write_text(json.dumps(ss,indent=2,ensure_ascii=False)+'\n')
order=['blackout','stores','dispatch','generator','radio','crisis','ending_full','ending_partial','ending_lost']
index=['# Complete shot index','', 'Stable editorial shot IDs, not new GRIMM nodes/edges or camera protocol.','Each linked PNG is an actual drawn image; SVG is editable original. Order within','a scene is a browsing order, **not** a claim all alternatives play linearly.','Follow BRANCHES for the actual interactive sequence.','', '| Shot / image | Scene / location | Camera | Beat | Clock / duration |','| --- | --- | --- | --- | --- |']
for scene in order:
 group=sorted([s for s in ss if s['scene']==scene],key=lambda s:s.get('page_order',int(s['id'][1:])))
 lines=[f'# {scene} — visual storyboard pages','', '[Reading guide](../README.md) · [Branches](../BRANCHES.md) · [Shot index](../SHOTS.md)','', 'The sheets are contact overviews. Full-size panels and readable metadata are','listed below on this same page; no separate file needs opening for each shot.','All imagery is [A] blocking/art, with [C] rule associations and [O] final approval.','']
 for page in range((len(group)+5)//6):lines += [f'## Sheet {page+1}','',f'![{scene} contact sheet {page+1}](../sheets/{scene}-{page+1:02}.png)','']
 for s in group:
  id=s['id'];lines += [f'<a id="{id.lower()}"></a>',f'## {id} — {s["title"]}','',f'![{id} actual drawn shot](../{s["panel"]})','',f'**Scene:** `{scene}` · **Location:** `{s["location"]}` · **Camera:** `{s["camera"]}` · **Beat:** `{s["beat"]}`.', '',f'**Time:** {s["time"]}.', '',f'**Condition / variant:** {s["state"]}.', '',f'**Audio/dialogue:** {s["audio"]}', '',f'**Motion/edit:** {s["motion"]}. **Next:** {s["next"]}.','',f'[Editable SVG](../{s["editable"]}) · [Source metadata](../sources/shots.json)','']
  index.append(f'| [{id}]({s["panel"]}) — {s["title"]} | `{scene}` / `{s["location"]}` | `{s["camera"]}` | {s["beat"]} | {s["time"]} |')
 if scene=='generator':lines += ['## Generated arrival artist study (alternate development, not extra coverage)','', 'Original tool-generated pencil study using cast/environment references. The','normalized blocking panel above remains the indexed board; this alternate','development drawing is not counted as a second completed shot.','', '![Generator artist study](../panels/g01-artist-study.png)','']
 if scene=='radio':lines += ['## Generated connected-set artist study (alternate development)','', 'Same role as generator study; no additional coverage count.','', '![Radio artist study](../panels/r01-artist-study.png)','']
 (r/'scenes'/f'{scene}.md').write_text('\n'.join(lines))
(r/'SHOTS.md').write_text('\n'.join(index)+'\n')
print('Published pages/index for',len(ss),'authored panels; rendering and validation remain separate checks.')
