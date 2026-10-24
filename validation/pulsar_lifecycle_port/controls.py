"""Comparator and live identity rejection controls over complete accepted S records."""
import copy,gzip,json
from pathlib import Path
from run import ROOT,OUT,identity,project,compare,save,validate_launch

def run(profile,exe,ident):
 results=[]
 def rejects(name,fn):
  try:fn()
  except (AssertionError,KeyError,ValueError,FileNotFoundError):results.append(dict(name=name,status='rejected'));return
  raise AssertionError(name+' was accepted')
 def case(name):return [project(e) for e in json.load(gzip.open(ROOT/'validation/pulsar_lifecycle/expected'/(name+'.json.gz')))['events']]
 baseline=case('copy_partial');compare(baseline,baseline);results.append(dict(name='unchanged positive',status='accepted'))
 def mutation(name,change,reference=baseline):
  actual=copy.deepcopy(reference);change(actual);rejects(name,lambda:compare(actual,reference))
 def record(events,path,phase=None):
  return next(r for e in events if phase is None or e['phase']==phase for r in e['state'] if r['path']==path)
 def unwritten(events,path,phase=None):
  r=record(events,path,phase);r.clear();r.update(path=path,state='unwritten')
 def change_byte(events,path,phase=None):
  r=record(events,path,phase);n,h=r['runs'][0];b=bytearray.fromhex(h);b[0]^=1;r['runs'][0]=[n,b.hex()]
 mutation('omitted event',lambda x:x.pop(1));mutation('permuted event',lambda x:x.reverse());mutation('omitted field',lambda x:x[1]['state'].pop(3));mutation('permuted fields',lambda x:x[1]['state'].reverse())
 mutation('known to unwritten',lambda x:unwritten(x,'destination.offset','after_copy_0'))
 mutation('lost string suffix',lambda x:record(x,'destination.name','after_copy_0')['runs'].pop())
 mutation('retained FD count changed',lambda x:change_byte(x,'destination.nfdJumps','after_copy_0'))
 mutation('inclusive FD endpoint lost',lambda x:change_byte(x,'destination.fdjumpVal','after_copy_0'))
 mutation('inclusive ordinary jump endpoint lost',lambda x:change_byte(x,'destination.jumpVal','after_copy_0'))
 mutation('omitted field copied',lambda x:change_byte(x,'destination.fdjumpIdx','after_copy_0'))
 mutation('covariance destination owner replaced',lambda x:record(x,'destination.covar','after_copy_0').update(symbol='source.covariance'))
 mutation('callback identity changed',lambda x:record(x,'destination.fitinfo.paramDerivs','after_copy_0')['symbols'].__setitem__(0,[10000,'callback:deriv0']))
 mutation('callback null versus unwritten',lambda x:record(x,'destination.fitinfo.paramDerivs','after_copy_0')['symbols'].__setitem__(0,[10000,None]))
 mutation('payload owner lost',lambda x:record(x,'destination.fitinfo.constraintSpecial','after_copy_0')['symbols'].__setitem__(0,[10000,'null']))
 mutation('destroy event lost',lambda x:x.pop())
 repeat=case('seeded_reinitialise')
 mutation('reset embedded linkTo',lambda x:change_byte(x,'p.param[0].linkTo','reinitialised'),repeat)
 mutation('known retained links hidden',lambda x:unwritten(x,'p.param[0].linkFrom','reinitialised'),repeat)
 mutation('original owner identity lost',lambda x:record(x,'old.covar','reinitialised').update(symbol='second.covariance'),repeat)
 mutation('original extra covariance lost',lambda x:record(x,'p.ToAextraCovar','caller_seeded').update(symbol='null'),repeat)
 overlap=case('copy_forward_overlap')
 mutation('overlap view offset lost',lambda x:record(x,'destination.obsn','after_copy_0').update(symbol='shared-observations@0'),overlap)
 mutation('forward copy history changed',lambda x:change_byte(x,'shared.data[3].freq','after_copy_0'),overlap)
 natural=case('natural_1')
 mutation('unwritten source default invented',lambda x:record(x,'p.ne_sw_ifuncN').update(unit=4,count=1,runs=[[1,'00000000']]),natural)
 mutation('full covariance extent reduced',lambda x:record(x,'p.covar.data.cells')['runs'].__setitem__(0,[4,'0000000000000000']),natural)
 mutation('covariance signed zero changed',lambda x:record(x,'p.covar.data.cells')['runs'].__setitem__(0,[100000000,'0000000000000080']),natural)
 mutation('matrix stride changed',lambda x:change_byte(x,'p.covar.data.row_offsets'),natural)
 stale=copy.deepcopy(ident);stale['executable_sha256']='0'*64
 rejects('stale executable',lambda:compare(stale,identity(exe)))
 stale=copy.deepcopy(ident);key=next(iter(stale['sources']));stale['sources'][key]='0'*64
 rejects('stale runner/source',lambda:compare(stale,identity(exe)))
 for name,launch,token,ih in [('stale challenge',dict(phase='launch',nonce='old',input_sha256='hash'),'new','hash'),('stale input',dict(phase='launch',nonce='new',input_sha256='old'),'new','hash'),('missing identity field',dict(phase='launch',nonce='new'),'new','hash')]:rejects(name,lambda l=launch,t=token,h=ih:validate_launch(l,t,h))
 save(OUT/profile/'controls.json',results);return results
