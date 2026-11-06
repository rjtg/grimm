"""WHITEOUT generator production originals: deterministic glTF 2.0 binary + PCM.
Python 3 standard library only. Metres, +Y up, +Z south/front, right handed.
Rigid named limb pivots are intentional; this is not a general asset exporter.
"""
from pathlib import Path
import json, math, struct, zlib, wave, random
ROOT = Path(__file__).resolve().parents[1]

def png():
    rng=random.Random(40); raw=bytearray()
    for y in range(64):
        raw.append(0)
        for x in range(64):
            v=210+rng.randrange(30)
            if (x*7+y*13)%101<3:v=150
            raw.extend((v,v,v,255))
    def chunk(t,b):return struct.pack('>I',len(b))+t+b+struct.pack('>I',zlib.crc32(t+b)&0xffffffff)
    return b'\x89PNG\r\n\x1a\n'+chunk(b'IHDR',struct.pack('>IIBBBBB',64,64,8,6,0,0,0))+chunk(b'IDAT',zlib.compress(bytes(raw),9))+chunk(b'IEND',b'')

class Model:
    def __init__(self):
        self.bin=bytearray();self.d={'asset':{'version':'2.0','generator':'WHITEOUT build_assets.py v1'},'scene':0,'scenes':[{'nodes':[]}],'nodes':[],'meshes':[],'materials':[],'bufferViews':[],'accessors':[],'buffers':[]}
        image=self.buffer(png());self.d.update(images=[{'bufferView':image,'mimeType':'image/png'}],samplers=[{'magFilter':9729,'minFilter':9729,'wrapS':10497,'wrapT':10497}],textures=[{'sampler':0,'source':0}]);self.meshes={}
    def buffer(self,data):
        self.bin.extend(b'\0'*((-len(self.bin))%4));i=len(self.d['bufferViews']);self.d['bufferViews'].append({'buffer':0,'byteOffset':len(self.bin),'byteLength':len(data)});self.bin.extend(data);return i
    def accessor(self,values,typ,n,component=5126):
        fmt='f' if component==5126 else 'I';flat=[v for row in values for v in row];a={'bufferView':self.buffer(struct.pack('<'+fmt*len(flat),*flat)),'componentType':component,'count':len(values),'type':typ}
        if typ=='VEC3':a.update(min=[min(v[k] for v in values)for k in range(n)],max=[max(v[k] for v in values)for k in range(n)])
        self.d['accessors'].append(a);return len(self.d['accessors'])-1
    def material(self,name,color,metal=0,rough=.85,emit=None):
        p={'baseColorFactor':[*color,1],'metallicFactor':metal,'roughnessFactor':rough,'baseColorTexture':{'index':0}}
        m={'name':name,'pbrMetallicRoughness':p}
        if emit:m['emissiveFactor']=emit
        self.d['materials'].append(m);return len(self.d['materials'])-1
    def mesh(self,kind,mat):
        if (kind,mat)in self.meshes:return self.meshes[kind,mat]
        pos=[];norm=[];uv=[];idx=[]
        if kind=='box':
            faces=[((1,0,0),[(.5,-.5,-.5),(.5,.5,-.5),(.5,.5,.5),(.5,-.5,.5)]),((-1,0,0),[(-.5,-.5,.5),(-.5,.5,.5),(-.5,.5,-.5),(-.5,-.5,-.5)]),((0,1,0),[(-.5,.5,-.5),(-.5,.5,.5),(.5,.5,.5),(.5,.5,-.5)]),((0,-1,0),[(-.5,-.5,.5),(-.5,-.5,-.5),(.5,-.5,-.5),(.5,-.5,.5)]),((0,0,1),[(-.5,-.5,.5),(.5,-.5,.5),(.5,.5,.5),(-.5,.5,.5)]),((0,0,-1),[(.5,-.5,-.5),(-.5,-.5,-.5),(-.5,.5,-.5),(.5,.5,-.5)])]
            for normal,points in faces:
                start=len(pos);pos+=points;norm += [normal]*4;uv += [(0,0),(1,0),(1,1),(0,1)];idx += [start,start+1,start+2,start,start+2,start+3]
        else:
            # Faceted cylinder along Y, unit diameter/height; capped, 12 sides.
            for k in range(12):
                a=2*math.pi*k/12;b=2*math.pi*(k+1)/12;mid=(a+b)/2;start=len(pos)
                pos += [(.5*math.cos(a),-.5,.5*math.sin(a)),(.5*math.cos(a),.5,.5*math.sin(a)),(.5*math.cos(b),.5,.5*math.sin(b)),(.5*math.cos(b),-.5,.5*math.sin(b))];norm += [(math.cos(mid),0,math.sin(mid))]*4;uv += [(0,0),(0,1),(1,1),(1,0)];idx += [start,start+1,start+2,start,start+2,start+3]
                for y,normal in [(.5,(0,1,0)),(-.5,(0,-1,0))]:
                    start=len(pos);pts=[(0,y,0),(.5*math.cos(a),y,.5*math.sin(a)),(.5*math.cos(b),y,.5*math.sin(b))]
                    if y>0:pts.reverse()
                    pos+=pts;norm += [normal]*3;uv +=[(.5,.5),(0,0),(1,1)];idx +=[start,start+1,start+2]
        primitive={'attributes':{'POSITION':self.accessor(pos,'VEC3',3),'NORMAL':self.accessor(norm,'VEC3',3),'TEXCOORD_0':self.accessor(uv,'VEC2',2)},'indices':self.accessor([(i,)for i in idx],'SCALAR',1,5125),'material':mat}
        self.d['meshes'].append({'name':kind+str(mat),'primitives':[primitive]});i=len(self.d['meshes'])-1;self.meshes[kind,mat]=i;return i
    def node(self,name,pos=(0,0,0),size=None,mat=None,parent=None,kind='box',rotation=None):
        d={'name':name,'translation':list(pos)}
        if size:d.update(mesh=self.mesh(kind,mat),scale=list(size))
        if rotation:d['rotation']=rotation
        self.d['nodes'].append(d);i=len(self.d['nodes'])-1
        if parent is None:self.d['scenes'][0]['nodes'].append(i)
        else:self.d['nodes'][parent].setdefault('children',[]).append(i)
        return i
    def save(self,name):
        self.d['buffers']=[{'byteLength':len(self.bin)}];js=json.dumps(self.d,separators=(',',':'),sort_keys=True).encode();js+=b' '*((-len(js))%4);self.bin+=b'\0'*((-len(self.bin))%4)
        body=struct.pack('<I4s',len(js),b'JSON')+js+struct.pack('<I4s',len(self.bin),b'BIN\0')+self.bin
        (ROOT/'models'/name).write_bytes(struct.pack('<4sII',b'glTF',2,12+len(body))+body)

def q(axis,angle):
    return [math.sin(angle/2)*v for v in axis]+[math.cos(angle/2)]

def annex():
    m=Model(); wall=m.material('worn_sage_panels',(.47,.55,.52),.3);steel=m.material('brushed_steel',(.24,.3,.32),.75,.55);dark=m.material('rubber_charcoal',(.055,.07,.08));frost=m.material('frost',(.72,.83,.85));red=m.material('oxide_red',(.62,.13,.07));amber=m.material('primer_amber',(.8,.53,.12));window=m.material('sealed_frosted_glass',(.24,.43,.5),.1,.45)
    def box(n,p,s,c,parent=None):return m.node(n,p,s,c,parent)
    box('loc.generator_annex.floor',(0,-.10,0),(4.5,.2,5),dark)
    box('north_wall',(0,1.5,-2.55),(4.5,3,.1),wall)
    for x in [-2.3,2.3]:box('side_wall',(x,1.5,0),(.1,3,5),wall)
    # South cutaway is the camera wall, with a physically bounded sole threshold.
    for x in [-1.7,1.7]:box('threshold_wing',(x,1.3,2.5),(1.1,2.6,.15),wall)
    box('threshold_header',(0,2.65,2.5),(2.3,.3,.2),steel)
    box('threshold_sill',(0,.025,2.5),(2.3,.05,.25),frost)
    for x in [-1.5,-.75,0,.75,1.5]:box('panel_seam',(x,1.5,-2.47),(.018,3,.025),steel)
    box('sealed_west_window',(-2.23,1.9,-.5),(.035,.65,1.3),window)
    for z in [-1.2,.2]:box('window_frame',(-2.2,1.9,z),(.08,.78,.05),steel)
    box('overhead_cable_tray',(0,2.7,-2),(3.8,.08,.28),steel)
    box('prop.generator',(.35,.85,-1.55),(1.8,1.5,1.1),wall)
    box('generator_plinth',(.35,.12,-1.55),(2,.24,1.25),steel)
    m.node('exhaust',(.25,2,-1.85),(.16,1.2,.16),steel,kind='cylinder')
    door=m.node('prop.generator_latch',(-.55,.35,-.965))
    box('starter_access_cover',(.6,.55,0),(1.2,1.1,.07),steel,door)
    for y in [.2,.4,.6,.8]:box('cover_cooling_fin',(.6,y,.05),(1.06,.035,.055),dark,door)
    box('latch_pin',(.99,.52,.09),(.08,.22,.05),amber,door)
    for y in [.18,.84]:m.node('hinge',(0,y,0),(.06,.2,.06),dark,door,kind='cylinder')
    ice=m.node('prop.latch_ice',(.45,.87,-.895));box('ice_bridge',(0,0,0),(.16,.25,.04),frost,ice)
    wheel=m.node('prop.generator_wheel',(.1,.8,-.92))
    m.node('flywheel',(0,0,0),(.7,.13,.7),dark,wheel,kind='cylinder',rotation=q((1,0,0),math.pi/2))
    for angle in [0,math.pi/2]:m.node('wheel_spoke',(0,0,.08),(.63,.06,.04),steel,wheel,rotation=q((0,0,1),angle))
    starter=m.node('prop.generator_starter',(.58,1.12,-.86));box('primer_lever',(0,.11,0),(.05,.25,.06),amber,starter)
    box('prop.primer_tube',(.72,1.1,-.84),(.045,.34,.04),amber)
    box('control_box',(1.65,1.2,-1.05),(.48,.7,.15),steel)
    fuse=m.node('prop.deicer_fuse',(1.65,1.2,-.94));box('fuse_handle',(0,.05,.055),(.08,.25,.06),red,fuse)
    box('prop.generator_diagram',(1.65,1.73,-.94),(.52,.27,.02),frost)
    for x in [1.48,1.65,1.82]:box('diagram_schematic',(x,1.73,-.92),(.035,.15,.01),dark)
    box('tag',(1.65,.87,-.93),(.18,.1,.03),amber)
    shutter=m.node('prop.annex_shutter',(0,4.1,2.48));box('shutter_panel',(0,0,0),(2.25,2.5,.08),steel,shutter)
    for y in range(-5,6):box('shutter_rib',(0,y*.2,.05),(2.25,.035,.04),dark,shutter)
    for z in [.25,.85]:box('rest_bench',(-1.5,.48,z),(.8,.08,.12),steel)
    for x in [-1.9,-1.1]:box('bench_leg',(x,.24,.55),(.06,.48,.6),dark)
    box('brace_rail',(-1.1,.95,-.1),(.9,.06,.06),steel)
    for x in [-1.55,-.65]:box('brace_leg',(x,.48,-.1),(.045,.95,.045),steel)
    box('prop.generator_coil',(.92,.45,2.1),(.22,.8,.18),red)
    for i in range(8):box('coil_fins',(.92,.12+i*.09,2.2),(.3,.025,.04),dark)
    box('status_lamp',(1.04,1.53,-.96),(.12,.065,.025),amber)
    box('cold_strip_light',(0,2.8,-.65),(1.4,.045,.18),frost)
    for i in range(16):box('threshold_frost',(-1+i*.13,.045,2.35+(i%3)*.03),(.12,.035,.17),frost)
    m.save('generator-annex.glb')

def actor(name):
    m=Model();colors={'alex':(.27,.31,.32),'mara':(.2,.39,.38),'jonas':(.53,.34,.19)}
    coat=m.material('textile_'+name,colors[name],rough=.96);skin=m.material('skin',(.58,.44,.33));dark=m.material('boots_gloves',(.065,.075,.085));hair=m.material('hair',(.13,.12,.11));trim=m.material('sand_reflective_trim',(.6,.55,.42));splint=m.material('splint',(.67,.7,.66))
    root=m.node('actor.'+name);seated=name=='jonas';hip=m.node('bone.hips',(0,.62 if seated else .9,0),parent=root)
    m.node('coat_hips',(0,.04,0),(.42,.25,.28),coat,hip)
    chest=m.node('bone.chest',(0,.14,0),parent=hip);m.node('coat_torso',(0,.19,0),(.5 if seated else .43,.46,.28),coat,chest)
    for x in [-.12,.12]:m.node('coat_pocket',(x,.13,.15),(.1,.1,.03),dark,chest)
    m.node('zip',(0,.23,.151),(.018,.4,.02),trim,chest)
    head=m.node('bone.head',(0,.58,0),parent=hip)
    m.node('head',(0,.10,0),(.21,.27,.21),skin,head,kind='cylinder')
    m.node('hair_cap',(0,.23,-.015),(.235,.095,.23),hair,head,kind='cylinder')
    for x in [-.052,.052]:m.node('eyes',(x,.13,.105),(.03,.018,.01),dark,head)
    m.node('nose',(0,.09,.115),(.04,.045,.045),skin,head)
    if name=='alex':
        m.node('hood',(0,.10,-.065),(.3,.35,.22),coat,head);m.node('mask',(0,.025,.11),(.19,.08,.04),dark,head)
        for x in [-.15,.15]:m.node('sand_shoulder',(x,.36,.145),(.13,.1,.035),trim,chest)
    if name=='mara':m.node('hair_bun',(-.105,.16,-.12),(.14,.14,.13),hair,head,kind='cylinder')
    if name=='jonas':m.node('beard',(0,-.005,.085),(.18,.08,.07),hair,head)
    for side,x in [('L',-.26),('R',.26)]:
        arm=m.node('bone.arm.'+side,(x,.38,0),parent=chest)
        m.node('sleeve_upper',(0,-.16,0),(.14,.32,.16),coat,arm)
        elbow=m.node('bone.elbow.'+side,(0,-.32,0),parent=arm)
        m.node('sleeve_lower',(0,-.14,0),(.12,.28,.14),coat,elbow);m.node('glove',(0,-.31,.015),(.12,.14,.16),dark,elbow)
        leg=m.node('bone.leg.'+side,(x*.52,-.02,0),parent=hip,rotation=q((1,0,0),-math.pi/2 if seated else 0))
        m.node('trousers_thigh',(0,-.2,0),(.16,.4,.19),dark,leg)
        knee=m.node('bone.knee.'+side,(0,-.4,0),parent=leg,rotation=q((1,0,0),math.pi/2 if seated else 0))
        m.node('trousers_shin',(0,-.18,0),(.15,.36,.16),dark,knee);m.node('boot',(0,-.36,.06),(.18,.12,.3),dark,knee)
        if seated and side=='L':m.node('treated_left_splint',(0,-.19,.1),(.18,.27,.035),splint,knee)
    m.save(name+'.glb')

def sounds():
    # New short cue sounds; preserve the existing wind/motor/alarm originals.
    rng=random.Random(40);rate=22050
    for name,length in [('latch',.75),('mechanism',1.1),('ignition',1.5),('overload',.8)]:
        samples=[]
        for i in range(int(rate*length)):
            t=i/rate;noise=rng.uniform(-1,1)
            if name=='latch':v=(math.sin(t*2*math.pi*1700)+noise*.4)*math.exp(-t*11)
            elif name=='mechanism':v=(noise*.5+math.sin(t*2*math.pi*130))*(.4+.6*math.sin(t*24)**2)*(1-t/length)
            elif name=='ignition':v=(math.sin(2*math.pi*(40*t+30*t*t))+noise*.2)*(.2+.8*t/length)
            else:v=(math.sin(t*2*math.pi*90)+noise*.6)*math.exp(-t*5)
            samples.append(int(max(-1,min(1,v*.3))*32767))
        with wave.open(str(ROOT.parent/'audio'/f'{name}.wav'),'wb')as w:w.setparams((1,2,rate,0,'NONE','not compressed'));w.writeframes(struct.pack('<'+'h'*len(samples),*samples))

if __name__=='__main__':
    annex()
    for name in ['alex','mara','jonas']:actor(name)
    sounds()
    print('Exported four self-contained GLB models and four new PCM WAV cues.')
