"""WHITEOUT-only authored blocking drawings. Not runtime/IDE/screenshot tests.
Run from repository root; needs Python stdlib and rsvg-convert for PNG export.
The editable originals are SVG; shots.json contains editorial metadata only.
"""
import json, math, subprocess, textwrap
from pathlib import Path
from html import escape
ROOT=Path(__file__).resolve().parents[1]

def text(x,y,s,size=20,fill='#252525',anchor='start'):
 return f'<text x="{x}" y="{y}" font-size="{size}" fill="{fill}" text-anchor="{anchor}" font-family="DejaVu Sans,sans-serif">{escape(str(s))}</text>'
def line(x,y,X,Y,stroke='#333',w=3,dash=''):
 return f'<path d="M{x},{y} L{X},{Y}" fill="none" stroke="{stroke}" stroke-width="{w}"'+(f' stroke-dasharray="{dash}"'if dash else'')+'/>'
def rect(x,y,w,h,fill='#ddd',stroke='#333',r=0):
 return f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="{r}" fill="{fill}" stroke="{stroke}" stroke-width="3"/>'
def ellipse(x,y,rx,ry,fill='#ddd',stroke='#333'):
 return f'<ellipse cx="{x}" cy="{y}" rx="{rx}" ry="{ry}" fill="{fill}" stroke="{stroke}" stroke-width="3"/>'
def poly(points,fill='#bbb',stroke='#333'):
 return '<polygon points="'+' '.join(f'{x},{y}' for x,y in points)+f'" fill="{fill}" stroke="{stroke}" stroke-width="3"/>'
def arrow(x,y,X,Y,label='',colour='#8b513d'):
 a=math.atan2(Y-y,X-x);p=[(X,Y),(X-13*math.cos(a-.45),Y-13*math.sin(a-.45)),(X-13*math.cos(a+.45),Y-13*math.sin(a+.45))]
 return line(x,y,X,Y,colour,4)+poly(p,colour,colour)+(text((x+X)/2,(y+Y)/2-10,label,16,colour)if label else'')
def actor(name,x,y,h=210,pose='stand',gaze=1,treated=True):
 # Feet y, deterministic identity features retained in every authored view.
 coat={'A':'#777c7d','M':'#657b79','J':'#a58163','L':'#7b8493'}[name]; u=h/210
 parts=[]
 def P(s):parts.append(s)
 # Figure local 100x210, winter clothing contours and separate hands/boots.
 seated=pose in ['sit','support','injured'];base=155 if seated else 185
 P(ellipse(50,208,48,6,'#bdb8ae','none'))
 if seated:
  P(poly([(24,113),(67,113),(96,147),(88,169),(47,147),(18,147)],'#555'))
  P(poly([(18,139),(43,139),(35,195),(12,197)],'#555'))
  P(poly([(71,147),(92,151),(117,187),(101,196)],'#666'))
  if name=='J' and treated:P(poly([(86,165),(102,158),(118,185),(104,193)],'#ddd'))
  P(ellipse(23,199,18,8,'#333'));P(ellipse(113,193,22,8,'#333'))
 else:
  P(poly([(22,119),(49,119),(45,197),(22,197)],'#555'));P(poly([(51,119),(78,119),(83,197),(57,197)],'#555'))
  P(ellipse(31,202,20,7,'#333'));P(ellipse(73,202,20,7,'#333'))
 hem={'L':126,'J':120,'M':110,'A':112}[name]
 P(poly([(18 if name=='J'else 25,50),(80 if name=='J'else 73,50),(87 if name=='J'else 85,hem),(15 if name=='J'else 17,hem)],coat))
 if name=='A':P(poly([(25,50),(40,50),(35,66),(21,71)],'#a79e89'));P(poly([(58,50),(73,50),(79,72),(62,66)],'#a79e89'))
 P(line(49,55,49,hem,'#333',2));P(line(20,85,80,85,'#555',1))
 P(rect(24,91,17,14,coat));P(rect(58,91,17,14,coat))
 if pose in ['reach','reach_back','hold','support','carry']:
  P(poly([(25,56),(17,70),(-9,105),(3,112),(33,83)],coat));P(poly([(74,56),(86,70),(108,90),(101,104),(69,82)],coat))
  P(ellipse(-2,109,10,9,'#555'));P(ellipse(106,98,10,9,'#555'))
 else:
  P(poly([(22,56),(10,61),(7,128),(21,130),(31,77)],coat));P(poly([(73,56),(85,61),(92,128),(78,132),(67,77)],coat))
  P(ellipse(14,134,10,11,'#555'));P(ellipse(86,135,10,11,'#555'))
 P(rect(41,43,17,13,'#666'))
 P('<g transform="translate(24.5,15.5) scale(.5)">')
 P(ellipse(49,31,22,28,'#c7bcaa'))
 if name=='A':
  P(poly([(22,38),(22,16),(32,0),(65,0),(80,17),(78,50),(67,47),(67,21),(33,21),(33,47)],'#555'))
  P(rect(29,34,42,20,'#555'));P(line(35,29,43,29,'#222',3));P(line(54,29,62,29,'#222',3))
 elif name=='M':
  P(ellipse(25,30,12,13,'#3d3938'));P(poly([(28,20),(31,8),(62,6),(73,24),(63,16),(36,16)],'#45413e'))
 elif name=='J':
  P(poly([(27,20),(30,6),(66,8),(74,21),(61,16),(35,17)],'#69635d'));P(poly([(31,43),(48,52),(65,43),(60,57),(39,57)],'#8d877e'))
 elif name=='L':
  P(rect(25,6,49,17,'#444',r=6));P(ellipse(26,34,7,10,'#333'));P(line(21,41,36,46,'#333',3));P(ellipse(37,46,4,3,'#333'))
 P(line(37,30,43,30,'#222',2));P(line(55,30,61,30,'#222',2));P(line(43,44,56,44,'#555',2))
 if pose=='reach_back':P(ellipse(49,29,24,29,'#555'));P(line(49,2,49,52,'#888',2))
 P('</g>')
 P(text(50,-10,{'A':'ALEX','M':'MARA','J':'JONAS','L':'LEA'}[name],14,anchor='middle'))
 if pose=='reach_back':
  P(arrow(83,35,83,-15,'','#555'))
 else:P(arrow(50,32,50+gaze*47,32,'','#555'))
 return f'<g transform="translate({x-50*u},{y-210*u}) scale({u})">'+''.join(parts)+'</g>'

def room(scene,view):
 p=[rect(0,0,1000,700,'#ddd')]
 p+=[poly([(0,0),(150,100),(150,450),(0,560)],'#aaa'),poly([(1000,0),(850,100),(850,450),(1000,560)],'#b9b9b5'),rect(150,100,700,350,'#d7d7cf'),poly([(0,700),(0,560),(150,450),(850,450),(1000,560),(1000,700)],'#bbb')]
 for x in range(170,850,85):p.append(line(x,100,x,450,'#aaa',1))
 for x in [0,200,400,600,800,1000]:p.append(line(500,290,x,700,'#aaa',1))
 if scene=='generator':
  p += [poly([(15,145),(125,185),(125,260),(15,230)],'#9ba8aa'),line(70,165,70,245,'#555',4)]
  p += [rect(400,195,420,250,'#a5aaa2'),rect(420,235,230,185,'#b9c3c0'),rect(675,265,120,120,'#c3c4bc'),rect(680,212,110,40,'#ddd'),text(735,238,'DIAGRAM',14,anchor='middle'),rect(415,160,90,30,'#aaa'),text(460,181,'FLAG',14,anchor='middle')]
  p += [line(360,390,360,490,w=7),line(210,390,360,390,w=7),line(210,390,210,490,w=7)]
  for y in range(275,410,25):p.append(line(440,y,630,y,'#777',2))
  p += [rect(723,290,20,55,'#555'),rect(712,355,47,23,'#d4bd91'),text(740,372,'FUSE',11,anchor='middle')]
  if view in ['decision_open','decision_unsafe','decision_power','heat','return','timeout']:
   p += [poly([(420,235),(490,200),(490,390),(420,420)],'#d7dcd7'),rect(530,250,15,110,'#ddd')]
  else:p.append(line(427,245,435,410,'#edf1ef',9))
  if view in ['decision_open','decision_power','heat','return']:p += [line(740,315,780,315,w=4),rect(775,290,20,55,'#555')]
  if view in ['heat','return','ignition']:p += [ellipse(570,327,58,58,'#777'),line(515,327,625,327,'#ccc',8),arrow(625,300,650,340,'SPIN')]
  if view in ['timeout','return','abandon']:
   p += [actor('J',240,500,210,'sit'),actor('M',400,510,220,'carry'),actor('A',570,530,230,'reach'),arrow(455,395,510,415,'GRIP')]
  else:p += [actor('J',280,440,190,'sit',treated=view!='rest'),actor('M',190,450,200,'support'),actor('A',510,510,230,'reach_back')]
  if view in ['timeout','sealed','abandon','return']:p.append(arrow(520,520,300,620,'RETREAT'))
  if view=='sealed':
   p += [rect(80,50,840,510,'#888')]
   for y in range(70,560,25):p.append(line(80,y,920,y,'#444',2))
   p += [actor('A',550,530,215),actor('M',350,520,205),actor('J',200,520,195,'sit')]
  if view in ['support','rest']:
   p=[rect(0,0,1000,700,'#ccc'),rect(650,120,350,370,'#aaa'),line(150,330,650,330,w=10),line(150,330,150,540,w=8),line(650,330,650,540,w=8),actor('J',560,530,320,'sit',treated=view!='rest'),actor('M',305,540,315,'support'),arrow(250,430,350,350,'SUPPORTED'if view=='support'else'REST ONLY')]
  if view=='annex':p.append(arrow(550,530,680,300,'CAM PUSH → G04'))
  if view=='heat':
   p += [arrow(730,240,840,180,'WARM AIR'),arrow(770,270,900,220,'')]
 elif scene=='blackout':
  for y in [135,195,265]:p.append(rect(425,y,150,16,'#555'if view=='lights'else'#ddd'))
  p += [rect(650,140,170,120,'#8f9ca2')]
  if view=='carry':p += [actor('A',225,510,225,'carry'),actor('J',350,485,210,'sit',treated=False),actor('M',480,510,225,'carry'),actor('L',740,490,220),arrow(390,490,680,520,'TO STORES')]
  else:p += [actor('A',410,500,240),actor('M',290,500,225,'support'),actor('J',170,490,210,'sit',treated=False),actor('L',700,490,220)]
  p += ['<g opacity=".22">'+poly([(235,385),(90,225),(810,420)],'#fbf6dc','none')+'</g>',rect(225,370,17,45,'#555'),arrow(245,400,180,445,'TORCH')]
  if view=='lights':p.append(arrow(605,125,605,260,'LIGHTS 1 → 2 → 3'))
 elif scene=='stores':
  p += [rect(420,115,330,55,'#9ba8aa'),line(585,115,585,170,w=4)]
  p += [rect(165,335,230,120,'#888'),rect(420,310,180,100,'#888'),rect(660,275,160,170,'#999')]
  for y in [180,250,320]:p.append(line(170,y,810,y,w=5))
  if view!='decision_done':
   p += [rect(180,195,65,43,'#a78c7a'),text(213,225,'+',30,anchor='middle'),rect(270,200,90,30,'#ccc'),rect(475,280,25,30,'#eee'),ellipse(507,294,9,10,'none'),ellipse(445,300,18,9,'none','#8e7662'),line(535,304,580,280,w=8),ellipse(900,290,30,30,'none'),line(940,270,940,330,'#9c7760',12),rect(885,370,65,65,'#777')]
  p += [rect(700,285,70,50,'#555'),text(735,317,'LOG',15,anchor='middle')]
  p += [actor('J',230,445,185,'sit',treated=view in ['treat','decision_done','leave']),actor('M',330,465,195,'support'),actor('A',500,520,220,'reach'),actor('L',760,485,200)]
 elif scene=='dispatch':
  p += [poly([(0,75),(120,125),(120,180),(0,125)],'#9ba8aa'),poly([(1000,75),(880,125),(880,180),(1000,125)],'#9ba8aa')]
  p += [poly([(0,160),(120,200),(120,430),(0,510)],'#777'),poly([(1000,160),(880,200),(880,430),(1000,510)],'#777'),rect(430,155,145,280,'#888'),text(45,150,'RADIO',19),text(835,150,'ENGINEERING',18),text(500,143,'RIDGE',19,anchor='middle'),rect(165,350,155,65,'#888')]
  p += [actor('J',240,440,190,'sit'),actor('M',390,460,210,'support'),actor('A',590,535,230)]
  if view=='cold':
   p += [actor('J',510,810,620,'sit'),text(200,120,'REACTION / EARLIER UNHEATED WAIT',23)]
 elif scene=='radio':
  p += [rect(180,330,290,100,'#888'),rect(195,220,220,110,'#777'),rect(630,125,200,325,'#bac6c9'),ellipse(730,300,72,72,'none')]
  for a in range(0,360,60):p.append(line(730,300,730+70*math.cos(math.radians(a)),300+70*math.sin(math.radians(a)),w=5))
  p += [actor('A',460,500,230,'reach'),actor('L',750,450,220,'stand'if view in ['auto','shutter','return_auto','decision_auto_return']else'hold')]
  if view in ['align','align_fail','manual','auto']:
   p=[rect(0,0,1000,700,'#bec6c6'),rect(300,90,620,440,'#cdd2d0'),ellipse(650,285,130,130,'none'),actor('L',430,535,355,'stand'if view=='auto'else'hold'),poly([(0,430),(145,350),(275,365),(360,460),(210,520),(0,540)],'#777')]
   for a in range(0,360,60):p.append(line(650,285,650+125*math.cos(math.radians(a)),285+125*math.sin(math.radians(a)),w=7))
   p.append(arrow(550,370,600,330,'RELEASE'if view=='auto'else'HOLD / ALIGN'))
  if view=='gust':
   p += [arrow(650,170,820,240,'GUST / CATCH')]
   for i in range(12):p.append(line(640+(i*19)%170,160+(i*31)%200,680+(i*19)%170,175+(i*31)%200,'#eee',4))
  if view=='align_fail':p.append(arrow(760,290,820,355,'SLIP / CATCH'))
  if view in ['return','return_auto','shutter']:p.append(arrow(430,500,220,540,'EXIT; L STAYS'if view=='return'else'BOTH INSIDE'))
 elif scene=='crisis':
  p=[rect(0,0,1000,700,'#cad0d0'),poly([(0,420),(500,210),(1000,390),(1000,700),(0,700)],'#ddd'),rect(80,220,200,180,'#aaa'),rect(740,360,190,140,'#999')]
  for x,y in [(450,260),(500,320),(550,400),(600,520)]:p += [line(x,y,x,y+30,w=5),ellipse(x,y,6,6,'#bfaa7c')]
  if view in ['coordinate_end','coordinate_auto','carry_end','carry_untreated','lea_saved']:
   p += [actor('L',330,320,155),actor('J',610,350,155,'sit',treated=view!='carry_untreated'),actor('M',720,390,170,'carry'),actor('A',490,390,170,'carry'),arrow(520,390,490,250,'CROSSING END')]
  else:
   if view!='help_jonas':p.append(actor('L',230,380,170,'hold'))
   if view!='help_lea':p.append(actor('J',810,480,180,'sit'))
   p += [actor('M',580,540,220,'reach'if view=='stop_plan'else'stand'),actor('A',470,550,230,'reach'if view=='coordinate'else'stand')]
  if view=='help_jonas':
   p=[rect(0,0,1000,700,'#cad0d0'),poly([(0,420),(500,210),(1000,390),(1000,700),(0,700)],'#ddd'),rect(80,220,200,180,'#aaa'),rect(740,360,190,140,'#999'),actor('M',650,490,210,'carry'),actor('J',750,455,180,'sit'),actor('A',890,490,210,'carry'),arrow(825,520,620,430,'CARRY JONAS AWAY')]
  if view=='stop_plan':p.append(arrow(575,365,490,385,'STOP BEFORE RUN'))
  if view=='coordinate_end':p += [line(280,300,520,355,'#8e7662',6),line(520,355,700,365,'#8e7662',6),text(390,300,'SAME SAFETY LINE',17)]
  if view=='flare':p += [line(500,410,520,180,'#a06040',5),ellipse(520,170,35,35,'#b89577'),arrow(540,200,640,100,'ONE FLARE')]
  if view=='help_lea':p.append(text(780,380,'NO JONAS REPLY',19,anchor='middle'))
  if view=='help_jonas':p.append(text(220,330,'LEA LOST',20,anchor='middle'))
  if view in ['coordinate','coordinate_end','help_lea','lea_saved']:p.append(arrow(470,450,245,370,'LEA / WEST'))
  if view in ['carry','carry_end','help_jonas']:p.append(arrow(540,460,790,410,'JONAS / EAST'))
  if view=='snow_end':
   p=[rect(0,0,1000,700,'#d6d9d5'),poly([(0,420),(500,210),(1000,390),(1000,700),(0,700)],'#d1d4cf'),rect(80,220,200,180,'#b5bdb9'),rect(740,360,190,140,'#b5bdb9'),text(480,330,'CROSSING ERASED',25,anchor='middle')]
  if view in ['snow','snow_end','lost']:
   for i in range(110 if view=='snow_end'else 40):
    x=(i*71)%1000;y=(i*47)%580;p.append(line(x,y,x+35,y+10,'#eee',4))
 else:
  p=[rect(0,0,1000,700,'#d7d5cd'),rect(80,95,840,480,'#b4b9b8')]
  for i,n in enumerate(['A','M','J','L']):
   x=190+i*200;p.append(rect(x-70,280,145,210,'#888'))
   if not ((view in ['partial_jonas','tag']and n=='J')or(view in ['partial_lea','handset']and n=='L')):p.append(actor(n,x,480,225,'sit',treated=True))
  if view in ['lost','lost_near','names']:
   p=[rect(0,0,1000,700,'#bbb'),poly([(100,430),(150,260),(750,260),(900,450)],'#666'),rect(250,335,400,135,'#777'),text(500,590,'FOUR NAMES. NO ANSWERS.',28,anchor='middle'),line(190,285,190,95,w=9),line(150,285,190,95,w=4),line(230,285,190,95,w=4),ellipse(200,180,40,55,'#888'),line(780,300,780,160,w=17),line(750,160,810,160,w=10)]
 if view=='lost_near':p=['<g transform="translate(-400,-170) scale(1.8)">'+''.join(p)+'</g>',arrow(800,250,900,400,'CAM PULL BACK') ]
 return ''.join(p)

def insert(view,scene):
 p=[rect(0,0,1000,700,'#cfcfc8'),poly([(0,500),(1000,420),(1000,700),(0,700)],'#aaa')]
 # Readable close foreground gloves, actual mechanism, distinct branch results.
 if view.startswith('latch')or view in ['grip_fail','lever_open','lever_fail']:
  p += [rect(200,100,620,430,'#b4bdbb'),line(290,110,290,500,w=12),rect(650,250,100,80,'#777')]
  for y in [170,390]:p.append(ellipse(290,y,24,24,'#666'))
  if view in ['latch_open','lever_open']:p.append(poly([(305,115),(600,65),(600,440),(305,500)],'#ddd'))
  else:p.append(line(315,120,340,490,'#e9eeee',18))
  if view=='latch_nearly':p.append(line(330,180,340,370,'#555',5))
  if view=='latch_crack':p.append(line(330,180,345,250,'#555',3))
  if view.startswith('lever'):p.append(poly([(350,410),(720,200),(730,218),(360,430)],'#999'))
  p += [poly([(20,410),(160,300),(280,330),(390,250),(430,280),(340,400),(185,410),(80,510)],'#666'),arrow(350,320,300 if 'fail'in view else 470,430 if 'fail'in view else 240,'SLIP'if'fail'in view else'WORK')]
 elif view in ['fuse','diagram']:
  p += [rect(180,80,640,450,'#aaa'),rect(220,110,550,190,'#ddd')]
  p += [line(260,160,650,160,w=6),line(450,160,450,260,'#8b513d',6),text(470,240,'DE-ICER',25),rect(480,340,90,120,'#666'),poly([(40,490),(300,370),(430,370),(525,395),(520,440),(300,450),(100,570)],'#777')]
  if view=='fuse':p.append(arrow(550,380,720,380,'PULL OUT'))
  else:p.append(arrow(390,230,445,160,'READ'))
 elif view=='battery':
  p += [rect(290,150,230,300,'#777'),rect(570,170,210,170,'#bbb'),text(405,240,'HEAT',34,anchor='middle'),text(405,285,'PACK',34,anchor='middle'),line(520,350,650,350,'#333',12),arrow(500,400,600,360,'CONNECT +60s')]
 elif view=='lever_make':
  p += [poly([(230,370),(650,170),(680,210),(250,410)],'#777')]
  for i in range(7):p.append(line(290+i*22,340-i*11,320+i*22,390-i*11,'#a58163',12))
  p += [poly([(20,420),(200,320),(320,350),(250,440),(60,510)],'#666'),poly([(980,300),(760,230),(650,200),(610,280),(800,340)],'#666'),arrow(410,240,480,310,'WRAP SINGLE BRAID')]
 elif view in ['primer','starter_cold','overload','ignition']:
  p += [rect(160,100,680,440,'#aeb6b3'),ellipse(610,310,130,130,'#777'),ellipse(610,310,25,25,'#aaa'),rect(260,160,35,290,'#ddd')]
  for a in range(0,360,90):p.append(line(610,310,610+110*math.cos(math.radians(a)),310+110*math.sin(math.radians(a)),'#ccc',12))
  p += [poly([(0,440),(170,330),(250,330),(300,270),(350,280),(320,370),(200,420),(50,540)],'#666')]
  if view=='overload':p += [rect(410,120,300,55,'#9d6a59'),text(560,155,'OVERLOAD / NO POWER',22,anchor='middle')]
  elif view=='starter_cold':p.append(arrow(310,330,390,410,'MISS'))
  elif view=='ignition':p.append(arrow(730,240,760,360,'TWO TURNS'))
  else:p += [rect(265,320,25,110,'#bfae7a'),arrow(300,200,300,300,'HOLD PRIMER')]
 else:return None
 return ''.join(p)

def story_insert(s):
 v=s['view'];scene=s['scene'];p=[rect(0,0,1000,700,'#d5d2c8'),poly([(0,480),(1000,430),(1000,700),(0,700)],'#a9aca7')]
 def hand():return poly([(0,460),(190,330),(340,330),(430,280),(480,320),(370,400),(150,460),(20,540)],'#686e6e')
 if v=='coordinate':
  p += [ellipse(620,290,130,130,'none')]
  for a in range(0,360,60):p.append(line(620,290,620+120*math.cos(math.radians(a)),290+120*math.sin(math.radians(a)),w=6))
  p += [ellipse(620,300,150,50,'none','#8e7662'),line(480,300,160,460,'#8e7662',12),hand(),arrow(500,210,600,300,'SECURE POSSESSED LINE')]
 elif v=='stay':
  p=[rect(0,0,1000,700,'#aaa'),rect(330,70,400,485,'#777'),rect(390,120,270,210,'#b4bec0'),rect(50,330,190,180,'#777')]
  for y in range(350,500,25):p.append(line(70,y,220,y,'#ccc',7))
  p += [arrow(570,130,570,260,'DRIFT / NO SAFE SHELTER'),text(405,400,'CLOSED RIDGE DOOR',22)]
 elif v in ['handset','handset_empty']:
  p += [rect(350,100,220,400,'#626c6e',r=20),rect(375,150,170,100,'#bbb')]
  for y in [310,345,380]:p.append(line(390,y,535,y,'#333',6))
  p += [line(400,100,400,25,w=10),hand()if v=='handset'else rect(620,350,160,120,'none')]
  p.append(arrow(590,200,790,180,'SAME BREATH'if v=='handset'else'NO REPLY'))
 elif v in ['injury','treat','tag']:
  if v=='tag':p += [poly([(300,170),(590,180),(660,460),(330,490)],'#666'),rect(400,225,180,100,'#c9bea4'),text(490,280,'JONAS',32,anchor='middle')]
  else:
   p += [poly([(180,180),(410,150),(570,420),(410,500)],'#777'),ellipse(540,450,170,65,'#555'),actor('M',800,520,290,'support')]
   if v=='treat':
    p.append(poly([(330,220),(430,190),(570,420),(450,445)],'#ddd'))
    for i in range(3):p.append(line(355+i*45,245+i*65,460+i*40,215+i*65,w=13))
    p += [hand(),arrow(400,210,440,330,'SECURE SPLINT')]
   else:p.append(arrow(310,270,420,350,'PROTECT ANKLE'))
 elif v in ['braid','mug','spanner','line','pack','liner']:
  p += [rect(180,140,640,370,'#b8bdb6')]
  if v=='mug':
   p += [rect(350,230,190,220,'#e3dfd3',r=20),ellipse(550,320,70,70,'none'),line(390,320,430,295,w=4),line(430,295,480,325,w=4)]
  elif v in ['braid','line']:
   for i in range(7):p.append(ellipse(470,320,170-i*13,90-i*7,'none','#8b7660'if v=='braid'else'#555'))
   if v=='line':p += [rect(650,230,40,200,'#b7785b'),text(670,450,'FLARE',20,anchor='middle')]
  elif v=='spanner':p += [poly([(300,430),(610,180),(670,175),(700,220),(650,255),(610,240),(330,460)],'#777')]
  elif v=='pack':p += [rect(350,200,250,270,'#777'),text(475,290,'HEAT',36,anchor='middle'),text(475,335,'PACK',36,anchor='middle')]
  elif v=='liner':p += [poly([(250,200),(370,170),(450,230),(530,170),(650,200),(700,400),(590,420),(580,500),(320,500),(310,420),(200,400)],'#bbb'),line(450,230,450,490,w=4)]
  p += [hand(),arrow(600,410,730,480,'TAKE ONE')]
 elif v in ['log','report','report_partial','names']:
  p += [poly([(200,120),(800,160),(730,500),(160,470)],'#eee')]
  for i,t in enumerate(['AURORA / MAINTENANCE','TEST LOOP — yesterday','DE-ICER BRANCH / BYPASS','JONAS — temporary bypass']if v!='names'else['ALEX','MARA','JONAS','LEA']):p.append(text(250,205+i*65,t,26))
  if v=='names':
   p=[rect(0,0,1000,700,'#bdbfb5'),rect(120,90,230,470,'#737b7a'),rect(360,90,480,470,'#b0b7af'),line(350,90,350,560,w=12)]
   for i,name in enumerate(['ALEX','MARA','JONAS','LEA']):p.append(text(450,195+i*90,name,43))
   p.append(text(430,545,'SAME RIDGE DOOR / SPRING',20))
  if v=='log':p.append(hand())
  if v=='report_partial':p += [rect(750,370,200,140,'none'),text(850,440,'EMPTY',23,anchor='middle')]
 elif v in ['adapter_start','adapter_end']:
  p += [rect(210,160,600,350,'#999'),rect(340,220,180,200,'#ddd',r=20),ellipse(560,300,70,60,'none'),rect(680,170,40,270,'#666')]
  p += [line(260,300,580,300,'#9e7f61',15),line(580,300,650,200 if v=='adapter_start'else 350,'#9e7f61',15),hand(),arrow(570,210,570,350,'THREAD'if v=='adapter_start'else'CLAMP / ISOLATED')]
 elif v in ['transmit_fail','warm_hands']:
  p += [rect(450,130,300,340,'#777'),hand()]
  if v=='transmit_fail':p += [rect(500,220,70,80,'#999'),rect(630,220,70,80,'#b58c79'),arrow(480,250,650,260,'WRONG BANK / NO CALL')]
  else:
   for y in range(200,440,35):p.append(line(475,y,725,y,'#ccc',6))
   p += [arrow(600,220,450,170,'WARM AIR'),arrow(600,320,420,260,'')]
 else:return None
 return ''.join(p)

def map_inset(scene):
 p=[rect(1080,95,440,250,'#e4e1d8'),text(1100,120,'CAMERA / BLOCKING — local plan',18)]
 p += [rect(1130,150,330,150,'none'),poly([(1275,298),(1240,255),(1320,255)],'#c3c0b4'),arrow(1280,315,1280,190,'C → N')]
 if scene=='generator':p += [rect(1240,160,150,55,'#aaa'),text(1315,183,'ENGINE',14,anchor='middle'),text(1390,230,'F',15),text(1160,260,'M J',17),text(1290,250,'A',18)]
 elif scene=='dispatch':p += [text(1110,230,'RADIO',14),text(1460,230,'ENG',14),text(1280,145,'RIDGE',14),text(1160,185,'J',18),text(1260,250,'A M',18)]
 elif scene=='radio':p += [text(1140,190,'DESK / A',15),text(1340,185,'WHEEL / L',15)]
 elif scene=='crisis':p += [text(1140,190,'LEA / W',15),text(1350,240,'JONAS / E',15),text(1250,280,'A M',17)]
 elif scene=='stores':p += [text(1140,190,'BENCH J/M',14),text(1260,170,'TRAY',14),text(1400,180,'LOG',14),text(1410,240,'RACK',14),text(1270,275,'A',17)]
 elif scene=='blackout':p += [text(1140,180,'WINDOW',14),text(1170,240,'M J A',17),text(1360,240,'L / RADIO',14)]
 else:p += [text(1150,195,'SEAT POSITIONS / EVIDENCE',14),text(1170,240,'A  M  J  L',17)]
 return ''.join(p)

def render(s):
 detail=story_insert(s) or insert(s['view'],s['scene'])
 body=detail or room(s['scene'],s['view'])
 framing='CU / insert, south half-plane' if detail else ('MS / reaction' if s['view'] in ['support','rest','cold'] else 'WS / established south axis')
 # Actual viewport remains 1000x700; board including notes is 16:9.
 decision=s['view'].startswith('decision') or s['view']=='warning';ui=350 if decision else 565
 if decision:
  offers={
   'stores':['Put on thermal liner','Treat Jonas with medical kit','Take single copper braid','Take gull mug','Take adjustable spanner','Pack safety line + flare','Take heat-pack battery','Read maintenance log','Leave stores'],
   'dispatch':['Commit to engineering','Commit to communications'],
   'radio':['Make ceramic/copper adapter','Hand-align dish','Warm hands','Abandon radio'],
   'crisis':['Attempt coordinated rescue','Go to Lea','Carry Jonas','Fire flare / retreat [if held]','Stay inside despite warning'],
   'blackout':['Carry Jonas into emergency stores'],
   'generator':['Deploy heat-pack battery (+60s)','Work frozen latch (3s)','Make insulated lever (spanner + single braid)','Inspect generator controls','Abandon engineering']}
  profiles={
   'decision_open':['Prime/start generator (4s)','Abandon engineering'],
   'decision_unsafe':['Read generator diagram','Prime/start generator (4s)','Abandon engineering'],
   'decision_power':['Return to dispatch'],
   'decision_lever':['Deploy heat pack','Work frozen latch (3s)','Use insulated lever (3s)','Inspect controls','Abandon engineering'],
   'decision_numb':['Work latch with numb hands (5s)','Make insulated lever','Isolate de-icer','Abandon engineering'],
   'decision_send':['Transmit position/manifest (3s)','Warm hands (4s)','Abandon radio'],
   'decision_return':['Return with rescue bearing','Warm hands (4s)'],
   'decision_done':['Leave prepared stores'],
   'decision_auto_return':['Return with rescue bearing','Warm hands (4s)'],
   'decision_radio':['Commit to engineering'],
   'decision_ridge':['Warm hands','Gather crew for ridge'],
   'decision_lost':['Gather crew for ridge'],
   'decision_powerfirst':['Commit to communications','Warm hands'],
   'decision_radiofirst':['Make ceramic/copper adapter','Hand-align dish','Abandon radio'],
   'decision_sendcold':['Transmit position/manifest (3s)','Abandon radio'],
   'decision_returncold':['Return with rescue bearing'],
   'decision_manualonly':['Hand-align dish','Warm hands','Abandon radio'],
   'decision_radioonly':['Commit to communications']}
  options=profiles.get(s['view'],offers.get(s['scene'],[]))
  body=rect(0,0,1000,700,'#bcbcb4')+'<g transform="translate(170,0) scale(.66)">'+body+'</g>'
  body+=rect(0,ui,1000,700-ui,'#f1eee6')+text(20,ui+30,'PLAYER VIEW / UI PROPOSAL — SOURCE GUARDS APPLY',19)
  body+=text(20,ui+60,'20s at T=10 unextended' if s['view']=='warning' else {'generator':'Readout dark before T=10; coil already runs','stores':'173s at T=7 / reserve continues','radio':'54s at T=6 / reserve continues','crisis':'37s at T=8 / crossing deadline','dispatch':'No decision deadline','blackout':'Begin when ready'}.get(s['scene'],''),24)
  if s['view'] in ['decision_power','decision_return','decision_returncold','decision_auto_return']:
   body+=rect(10,ui+36,980,36,'#f1eee6','none')+text(20,ui+60,'Deadline removed — only actual eligible offers',24)
  elif s['view'] in ['decision_open','decision_send','decision_done']:
   body+=rect(10,ui+36,980,36,'#f1eee6','none')+text(20,ui+60,'Current remaining budget — no restart',24)
  if s['view']=='warning':
   body+=text(25,ui+110,'WORKING: input locked until at least T=11.',24)+text(25,ui+155,'Latch media + Lea radio continue concurrently.',21)
   body+='<g transform="translate(720,75) scale(.25)">'+insert('latch_crack','generator')+'</g>'+text(720,65,'PARALLEL HAND CUE',17)
  else:
   for i,o in enumerate(options):body+=text(25,ui+95+i*(24 if len(options)>6 else 32),f'{i+1}. {o}',18 if len(options)>6 else 20)
 else:body+=rect(0,ui,1000,135,'#efebe1')+text(20,ui+35,s['audio'][:88],19)+text(20,ui+75,'No new gameplay event is committed by this drawing.',16)
 out=['<svg xmlns="http://www.w3.org/2000/svg" width="1600" height="900" viewBox="0 0 1600 900">',rect(0,0,1600,900,'#f7f3e8'),text(25,38,s['id']+' / '+s['title'],28),text(25,65,'WHITEOUT — [A] drawn production storyboard / current viewport 1000×700',17)]
 out+=['<svg x="20" y="85" width="1000" height="700" viewBox="0 0 1000 700">'+body+'</svg>',rect(20,85,1000,700,'none'),'<g transform="translate(216,19) scale(.8)">'+map_inset(s['scene'])+('</g>')]
 out.append(text(1060,320,framing,18))
 if detail:
  out += ['<g transform="translate(216,19) scale(.8)">'+arrow(1280,290,1280,235,'C0→C1')+rect(1266,225,27,15,'#8b513d')+arrow(1280,225,1310,185,'CU')+'</g>']
 y=350
 for label,key in [('SCENE','scene'),('CAMERA','camera'),('BEAT','beat'),('TIME / COST','time'),('CONDITION','state'),('MOTION / EDIT','motion'),('NEXT','next')]:
  out.append(text(1060,y,label,15,'#7b4f3c'));y+=23
  for row in textwrap.wrap(str(s[key]),55):out.append(text(1060,y,row,18));y+=22
  y+=6
 out += [text(25,825,'REFERENCES: PR #77 screenplay / #78 cast / #79 fixed set. Metric contacts and final art approval pending.',18),text(25,858,'Eye arrows = gaze; rust arrows = motion/camera. [C] rules retained; [A] art/layout; [O] production review.',18),'</svg>']
 return ''.join(out)

def build():
 shots=json.loads((ROOT/'sources/shots.json').read_text())
 for s in shots:
  p=ROOT/'panels'/f'{s["id"].lower()}.svg';p.write_text(render(s));subprocess.run(['rsvg-convert','-o',str(p.with_suffix('.png')),str(p)],check=True)
 # Printable inline-vector contact sheets; each panel also has a full-res PNG.
 for scene in dict.fromkeys(s['scene']for s in shots):
  group=sorted([s for s in shots if s['scene']==scene],key=lambda s:s.get('page_order',int(s['id'][1:])))
  for page in range(0,len(group),6):
   subset=group[page:page+6];parts=['<svg xmlns="http://www.w3.org/2000/svg" width="1600" height="1450" viewBox="0 0 1600 1450">',rect(0,0,1600,1450,'#f7f3e8'),text(25,35,f'WHITEOUT / {scene} / page {page//6+1}',25)]
   for i,s in enumerate(subset):parts.append(f'<g transform="translate({(i%2)*800},{60+(i//2)*460}) scale(.5)">'+render(s).split('>',1)[1].rsplit('</svg>',1)[0]+'</g>')
   parts.append('</svg>');p=ROOT/'sheets'/f'{scene}-{page//6+1:02}.svg';p.write_text(''.join(parts));subprocess.run(['rsvg-convert','-o',str(p.with_suffix('.png')),str(p)],check=True)
 print('Rendered',len(shots),'individual actual drawings and contact sheets.')
if __name__=='__main__':build()
