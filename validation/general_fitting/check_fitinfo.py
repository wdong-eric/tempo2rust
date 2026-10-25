"""Original/corrected logarithmic noise prior labels and harmonic neighbor."""
from pathlib import Path
import difflib, hashlib, json, subprocess
from fitting_corrections import corrected

ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
BUILD=HERE/'build/fitinfo';BUILD.mkdir(parents=True,exist_ok=True)
REF=ROOT/'validation/reference/build';CC='/opt/homebrew/bin/g++-16'
PIN='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
original=subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',PIN+':t2fit.C']).decode()
fixed=corrected('t2fit.C',original)
(BUILD/'corrections.patch').write_text(''.join(difflib.unified_diff(original.splitlines(True),fixed.splitlines(True),fromfile='original/t2fit.C',tofile='corrected/t2fit.C')))
records={};commands=[]
for kind,source in [('original',original),('corrected',fixed)]:
 path=BUILD/(kind+'.C');path.write_text(source);obj=BUILD/(kind+'.o');exe=BUILD/kind
 command=[CC,'-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(REF/'source'),'-c',str(path),'-o',str(obj)]
 subprocess.run(command,check=True,capture_output=True);commands.append(command)
 command=[CC,'-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(REF/'source'),str(HERE/'log_prior_probe.cc'),str(obj),str(REF/'libtempo2-reference.a'),'-lquadmath','-lgfortran','-o',str(exe)]
 subprocess.run(command,check=True,capture_output=True);commands.append(command)
 proc=subprocess.run([str(exe)],check=True,capture_output=True,timeout=30);(BUILD/(kind+'.log')).write_bytes(proc.stdout+proc.stderr)
 rows=[list(map(int,line.split()[1:])) for line in proc.stdout.decode().splitlines() if line.startswith('PRIOR ')]
 labels=[list(map(int,line.split()[1:])) for line in proc.stdout.decode().splitlines() if line.startswith('IFUNCLABEL ')]
 quad=[list(map(int,line.split()[1:])) for line in proc.stdout.decode().splitlines() if line.startswith('QUADLABEL ')]
 solar=[list(map(int,line.split()[1:])) for line in proc.stdout.decode().splitlines() if line.startswith('SOLARLABEL ')]
 assert [r[2]==r[3]for r in solar]==([True]*4 if kind=='corrected' else [True,False,True,False])
 assert [r[1]==r[2]for r in quad]==([True]*4 if kind=='corrected' else [True,False,False,False])
 records[kind]={'rows':rows,'ifunc_labels':labels,'quadrupole_labels':quad,'solar_labels':solar,'exe_sha256':sha(exe),'output_sha256':sha(BUILD/(kind+'.log'))}
 assert all(row[-1]==1 for row in rows)
assert records['original']['rows'][:4]==records['corrected']['rows'][:4]
old=[r[2] for r in records['original']['rows'] if r[0]==1 and r[3]==1]
new=[r[2] for r in records['corrected']['rows'] if r[0]==1 and r[3]==1]
assert old==[57,57,55,55] and new==[57,58,55,56],(old,new)
assert [r[1] for r in records['original']['ifunc_labels']]==[6,64,64]
assert [r[1] for r in records['corrected']['ifunc_labels']]==[6,6,64]
(BUILD/'receipt.json').write_text(json.dumps({'pin':PIN,'commands':commands,'probe_sha256':sha(HERE/'log_prior_probe.cc'),'builder_sha256':sha(Path(__file__)),'archive_sha256':sha(REF/'libtempo2-reference.a'),'patch_sha256':sha(BUILD/'corrections.patch'),'records':records},indent=2)+'\n')
print(json.dumps(records,indent=2))
