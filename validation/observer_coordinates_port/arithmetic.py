"""Actual accepted object/assembly identities and bounded contraction witnesses."""
import hashlib,json,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];X=ROOT/'validation/observer_coordinates';OUT=HERE/'build/arithmetic';OUT.mkdir(parents=True,exist_ok=True)
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
profile=json.loads((X/'build/ordinary/profile.json').read_text());commands=[];objects={}
for name in ['get_obsCoord','tai2ut1']:
 source=X/'build/ordinary/source'/(name+'.C');obj=X/'build/ordinary'/(name+'.o')
 command=['/opt/homebrew/bin/g++-16','-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(source.parent),'-S',str(source),'-o',str(OUT/(name+'.s'))]
 subprocess.run(command,check=True);commands.append(command)
 dump=subprocess.check_output(['otool','-tvV',str(obj)],text=True);(OUT/(name+'.object.s')).write_text(dump)
 assert any(op in dump for op in ['fmadd','fmsub','fmla'])
 objects[name]=dict(source=str(source),source_sha256=sha(source),accepted_object=str(obj),accepted_object_sha256=sha(obj),assembly_sha256=sha(OUT/(name+'.s')),object_disassembly_sha256=sha(OUT/(name+'.object.s')))
initial=json.loads((HERE/'build/initial-debug/run/direct.json').read_text())
current=json.loads((HERE/'outputs/debug/direct.json').read_text());assert current['status']=='PASS' and current['scope']=='all'
failed=[r for r in initial['cases'] if r['status']=='FAIL'];assert len(failed)==18
assert {r['id'] for r in failed}<={r['id'] for r in current['cases'] if r['status']=='PASS'}
receipt=dict(status='PASS',scope='18 accepted-X whole-history witnesses plus actual compiled native assembly; no global contraction or tolerance change',commands=commands,compiler_sha256=sha('/opt/homebrew/bin/g++-16'),objects=objects,witnesses=failed,initial_receipt_sha256=sha(HERE/'build/initial-debug/run/direct.json'),current_receipt_sha256=sha(HERE/'outputs/debug/direct.json'),initial_identity=initial['identity'],current_identity=current['identity'])
(OUT/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');print('PASS 18 bounded contraction witnesses and actual accepted native object identities')
