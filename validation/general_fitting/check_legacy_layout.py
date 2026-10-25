"""Original/corrected legacy global constraint labels, offsets and empty rows."""
from pathlib import Path
import difflib, hashlib, json, subprocess
from fitting_corrections import correct_legacy_global
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
BUILD=HERE/'build/legacy-layout';BUILD.mkdir(parents=True,exist_ok=True)
REF=ROOT/'validation/reference/build';PIN='fda6ad4a9753ad7dc5d291406f2f310a9161423f';sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
old=subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',PIN+':t2fit.C']).decode()
new=correct_legacy_global(old)
(BUILD/'correction.patch').write_text(''.join(difflib.unified_diff(old.splitlines(True),new.splitlines(True),fromfile='original/t2fit.C',tofile='corrected/t2fit.C')))
records=[]
for version,text in [('original',old),('corrected',new)]:
 start=text.index('void TKleastSquares_global_pulsar(');end=text.index('void TKfit_getPulsarDesignMatrix(',start);block=text[start:end]
 token='    // go ahead and do the fit!'
 assert block.count(token)==1
 block=block.replace(token,'    extern void ae_legacy_matrix(double**,double**,int,int,int);ae_legacy_matrix(designMatrix,constraintsMatrix,totalObs,totalFit,totalConstraints);\n'+token)
 source=BUILD/(version+'.C');source.write_text(text[:start]+block+text[end:]);obj=BUILD/(version+'.o');exe=BUILD/version
 flags=['/opt/homebrew/bin/g++-16','-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(REF/'source')]
 commands=[flags+['-c',str(source),'-o',str(obj)],flags+[str(HERE/'legacy_layout_probe.cc'),str(obj),str(REF/'libtempo2-reference.a'),'-lquadmath','-lgfortran','-o',str(exe)]]
 for cmd in commands:
  p=subprocess.run(cmd,capture_output=True);assert p.returncode==0,p.stderr.decode()
 for mode in range(3):
  p=subprocess.run([str(exe),str(mode)],capture_output=True,timeout=30)
  (BUILD/f'{version}-{mode}.log').write_bytes(p.stdout+p.stderr)
  values=[r.split() for r in p.stdout.decode().splitlines() if r.startswith(('CONSTRAINT','SHAPE'))]
  assert (p.returncode!=0)==(version=='original'and mode==2),(version,mode,p.returncode)
  records.append({'version':version,'mode':mode,'commands':commands,'source_sha256':sha(source),'executable_sha256':sha(exe),'exit':p.returncode,'values':values})
for r in records:
 if r['exit']==0:
  c={(int(a[1]),int(a[2])):float(a[3]) for a in r['values'] if a[0]=='CONSTRAINT'}
  if r['version']=='corrected' and r['mode']<2:
   assert c[1,4]==-.5 and c[1,5]==2.5
   if r['mode']==1:assert c[0,0]==.5 and c[0,1]==.5 and c[0,2]==0
  print(r['version'],r['mode'],c)
 else:print(r['version'],r['mode'],'exit',r['exit'])
(BUILD/'receipt.json').write_text(json.dumps({'pin':PIN,'builder_sha256':sha(__file__),'probe_sha256':sha(HERE/'legacy_layout_probe.cc'),'patch_sha256':sha(BUILD/'correction.patch'),'records':records},indent=2)+'\n')
