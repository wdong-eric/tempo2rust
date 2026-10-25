"""Original/corrected callback witnesses, including safely padded IFUNC row."""
from pathlib import Path
import subprocess,json,hashlib,difflib
from fitting_corrections import corrected
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent;B=HERE/'build/callbacks';B.mkdir(parents=True,exist_ok=True)
R=ROOT/'validation/reference/build';CC='/opt/homebrew/bin/g++-16';PIN='fda6ad4a9753ad7dc5d291406f2f310a9161423f';sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();objects=[];commands=[];patch=''
for name in ['t2fit_stdFitFuncs.C','t2fit_fitwaves.C','constraints_covar.C']:
 old=subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',PIN+':'+name]).decode();new=corrected(name,old);assert new!=old;path=B/name;path.write_text(new);obj=B/(name+'.o');cmd=[CC,'-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(R/'source'),'-c',str(path),'-o',str(obj)];subprocess.run(cmd,check=True,capture_output=True);commands.append(cmd);objects.append(obj);patch+=''.join(difflib.unified_diff(old.splitlines(True),new.splitlines(True),fromfile='original/'+name,tofile='corrected/'+name))
(B/'corrections.patch').write_text(patch);records={}
for name,objs in [('original',[]),('corrected',objects)]:
 exe=B/name;cmd=[CC,'-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(R/'source'),str(HERE/'callback_defect_probe.cc'),*map(str,objs),str(R/'libtempo2-reference.a'),'-lquadmath','-lgfortran','-o',str(exe)];subprocess.run(cmd,check=True,capture_output=True);commands.append(cmd);p=subprocess.run([str(exe)],check=True,capture_output=True);(B/(name+'.log')).write_bytes(p.stdout+p.stderr);records[name]={'exe_sha256':sha(exe),'output':p.stdout.decode()}
 for line in p.stdout.decode().splitlines():
  parts=line.split();i=parts.index('EXPECTED');got=list(map(float,parts[1:i]));expected=list(map(float,parts[i+1:]));match=all(abs(a-b)<=5e-14*abs(b)for a,b in zip(got,expected));assert match==(name=='corrected'),line
(B/'receipt.json').write_text(json.dumps({'pin':PIN,'builder_sha256':sha(Path(__file__)),'corrections_sha256':sha(HERE/'fitting_corrections.py'),'probe_sha256':sha(HERE/'callback_defect_probe.cc'),'archive_sha256':sha(R/'libtempo2-reference.a'),'objects':{str(p):sha(p)for p in objects},'commands':commands,'records':records},indent=2)+'\n');print(json.dumps(records,indent=2))
