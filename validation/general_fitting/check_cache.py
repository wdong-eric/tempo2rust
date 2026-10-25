"""Demonstrate stale polynomial cache against an otherwise identical fresh call."""
from pathlib import Path
import subprocess, hashlib, json, difflib
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
BUILD=HERE/'build/cache';BUILD.mkdir(parents=True,exist_ok=True)
REF=ROOT/'validation/reference/build';CC='/opt/homebrew/bin/g++-16';PIN='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
old=subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',PIN+':t2fit_nestlike.C']).decode()
new=old.replace('static std::map<std::tuple<int,int,param_label>, CoeffCache> cache;', 'std::map<std::tuple<int,int,param_label>, CoeffCache> cache;')
assert old.count('static std::map<std::tuple<int,int,param_label>, CoeffCache> cache;')==3
(BUILD/'correction.patch').write_text(''.join(difflib.unified_diff(old.splitlines(True),new.splitlines(True),fromfile='original/t2fit_nestlike.C',tofile='corrected/t2fit_nestlike.C')))
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
records={};commands=[]
for name,source in [('original',old),('corrected',new)]:
 path=BUILD/(name+'.C');path.write_text(source);exe=BUILD/name
 cmd=[CC,'-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(REF/'source'),str(HERE/'cache_probe.cc'),str(path),str(REF/'libtempo2-reference.a'),'-lquadmath','-lgfortran','-o',str(exe)]
 subprocess.run(cmd,check=True,capture_output=True);commands.append(cmd)
 runs=[]
 for mode in ['0','1']:
  p=subprocess.run([str(exe),mode],check=True,capture_output=True);out=BUILD/(name+'-'+mode+'.log');out.write_bytes(p.stdout+p.stderr)
  runs.append({line.split()[0]:float(line.split()[1]) for line in p.stdout.decode().splitlines() if line.startswith(('BEFORE','TOA','FREQ'))})
 records[name]={'source_sha256':sha(path),'executable_sha256':sha(exe),'runs':runs}
assert records['original']['runs'][0]['TOA']!=records['original']['runs'][1]['TOA']
for k in ['TOA','FREQ']:assert records['corrected']['runs'][0][k]==records['corrected']['runs'][1][k]
assert records['original']['runs'][0]['BEFORE']==records['corrected']['runs'][0]['BEFORE']
(BUILD/'receipt.json').write_text(json.dumps({'pin':PIN,'builder_sha256':sha(Path(__file__)),'probe_sha256':sha(HERE/'cache_probe.cc'),'archive_sha256':sha(REF/'libtempo2-reference.a'),'commands':commands,'records':records},indent=2)+'\n')
print(json.dumps(records,indent=2))
