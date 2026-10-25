"""Focused original/corrected nonzero-RHS and robust evidence; no accepted writes."""
from pathlib import Path
import subprocess,json,hashlib,difflib
from fitting_corrections import corrected as correct
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent;BUILD=HERE/'build/solver';BUILD.mkdir(parents=True,exist_ok=True)
REF=ROOT/'validation/reference/build';CC='/opt/homebrew/bin/g++-16';PIN='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
original=subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',PIN+':TKfit.C']).decode()
corrected=correct('TKfit.C',original)
assert corrected!=original
(BUILD/'correction.patch').write_text(''.join(difflib.unified_diff(original.splitlines(True),corrected.splitlines(True),fromfile='original/TKfit.C',tofile='corrected/TKfit.C')))
commands=[];records={}
for name,src in [('original',original),('corrected',corrected)]:
 path=BUILD/(name+'.C');path.write_text(src);obj=BUILD/(name+'.o');exe=BUILD/name
 cmd=[CC,'-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(REF/'source'),'-c',str(path),'-o',str(obj)]
 subprocess.run(cmd,check=True,capture_output=True);commands.append(cmd)
 cmd=[CC,'-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(REF/'source'),str(HERE/'solver_probe.cc'),str(obj),str(REF/'libtempo2-reference.a'),'-lquadmath','-lgfortran','-o',str(exe)]
 p=subprocess.run(cmd,capture_output=True);(BUILD/(name+'-build.log')).write_bytes(p.stdout+p.stderr);assert p.returncode==0,p.stderr.decode();commands.append(cmd)
 p=subprocess.run([str(exe)],capture_output=True);assert p.returncode==0,p.stderr.decode();(BUILD/(name+'.log')).write_bytes(p.stdout+p.stderr)
 records[name]={'source':sha(path),'executable':sha(exe),'output':sha(BUILD/(name+'.log')),'values':[x for x in p.stdout.decode().splitlines() if x.startswith(('TARGET','ROBUST','WRAPPER'))]}
assert ' P 0 ' in records['original']['values'][1]
assert abs(float(records['corrected']['values'][1].split()[3])-8/7)<1e-15
for name,record in records.items():
 direct=next(line for line in record['values'] if line.startswith('ROBUST W'))
 wrapper=next(line for line in record['values'] if line.startswith('WRAPPER W'))
 assert (direct.split()[2:]==wrapper.split()[2:]) == (name=='corrected')
(BUILD/'receipt.json').write_text(json.dumps({'pin':PIN,'probe_sha256':sha(HERE/'solver_probe.cc'),'builder_sha256':sha(Path(__file__)),'archive_sha256':sha(REF/'libtempo2-reference.a'),'patch_sha256':sha(BUILD/'correction.patch'),'commands':commands,'records':records},indent=2)+'\n')
print(json.dumps(records,indent=2))
