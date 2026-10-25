"""AD native timing integration in owned staging, reusing AC's verified build recipe.
Accepted AC/G/A2 artifacts are read-only. Every rebuilt unit and patch is identified.
"""
from pathlib import Path
import importlib.util,subprocess,json,hashlib,difflib
from corrections import corrected
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
BUILD=HERE/'build/native'
spec=importlib.util.spec_from_file_location('ac_native_recipe',ROOT/'validation/input_to_bat/native_pipeline.py')
ac=importlib.util.module_from_spec(spec);spec.loader.exec_module(ac)
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def build():
 ac.BUILD=BUILD/'ac';ac.build()
 src=BUILD/'source';src.mkdir(parents=True,exist_ok=True)
 original=ac.blob('formResiduals.C').decode();res=original
 start=res.index('        double newmean = 0;');end=res.index('        // deal with the REFPHS',start)
 res=res[:start]+res[end:]
 edits=[('psr->param[param_start]','psr[p].param[param_start]'),('psr->param[param_finish]','psr[p].param[param_finish]'),('observation *o = psr[0].obsn+i;','observation *o = psr[p].obsn+i;'),('psr->TNShapelet','psr[p].TNShapelet'),('averageResiduals(psr, 1);','averageResiduals(&psr[p], 1);'),('averageDMResiduals(psr,1);','averageDMResiduals(&psr[p],1);'),('i<=psr[p].nobs','i<psr[p].nobs')]
 for a,b in edits:
  assert a in res,a
  res=res.replace(a,b)
 init=(ac.BUILD/'source/initialise.C').read_text();assert 'psr->AverageResiduals=0;' in init
 init=init.replace('psr->AverageResiduals=0;','psr->AverageResiduals=0;\n    psr->AverageDMResiduals=0; // AD: omitted opt-in default.')
 init=init.replace('psr->nWhite = 0;', 'psr->nWhite = 0; psr->nWhite_dm = 0;')
 (src/'formResiduals.C').write_text(res);(src/'initialise.C').write_text(init)
 patch=''.join(difflib.unified_diff(original.splitlines(True),res.splitlines(True),fromfile='original/formResiduals.C',tofile='corrected/formResiduals.C'))
 patch+=''.join(difflib.unified_diff((ac.BUILD/'source/initialise.C').read_text().splitlines(True),init.splitlines(True),fromfile='AC/initialise.C',tofile='AD/initialise.C'))
 (BUILD/'corrections.patch').write_text(patch)
 binary_names=['DDKmodel.C','T2model.C','ELL1model.C','ELL1Hmodel.C']
 for name in binary_names:
  old=ac.blob(name).decode();new=corrected(name,old);(src/name).write_text(new)
  patch+=''.join(difflib.unified_diff(old.splitlines(True),new.splitlines(True),fromfile='original/'+name,tofile='corrected/'+name))
 (BUILD/'corrections.patch').write_text(patch)
 objects=[];commands=[]
 for name in ['formResiduals.C','initialise.C']+binary_names:
  obj=BUILD/(name+'.o');cmd=[ac.CC,'-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(ac.REF/'source'),'-c',str(src/name),'-o',str(obj)]
  result=subprocess.run(cmd,capture_output=True);(BUILD/(name+'.log')).write_bytes(result.stdout+result.stderr);assert result.returncode==0,result.stderr.decode();objects.append(obj);commands.append(cmd)
 objects += [p for p in sorted(ac.BUILD.glob('*.C.o')) if p.name!='initialise.C.o']
 exe=BUILD/'timing';cmd=[ac.CC,'-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(ac.REF/'source'),str(HERE/'native_timing.cc'),*map(str,objects),str(ac.REF/'libtempo2-reference.a'),'-lquadmath','-lgfortran','-Wl,-map,'+str(BUILD/'timing.map'),'-o',str(exe)]
 result=subprocess.run(cmd,capture_output=True);(BUILD/'link.log').write_bytes(result.stdout+result.stderr);assert result.returncode==0,result.stderr.decode();commands.append(cmd)
 receipt={'pin':ac.PIN,'builder_sha256':sha(Path(__file__)),'binary_corrections_sha256':sha(HERE/'corrections.py'),'ac_recipe_sha256':sha(ROOT/'validation/input_to_bat/native_pipeline.py'),'ac_identity_sha256':sha(ac.BUILD/'identity.json'),'executable_sha256':sha(exe),'harness_sha256':sha(HERE/'native_timing.cc'),'patch_sha256':sha(BUILD/'corrections.patch'),'objects':{str(p):sha(p) for p in objects},'sources':{str(p):sha(p) for p in src.iterdir()},'commands':commands}
 line_maps={}
 for source in [*ac.BUILD.joinpath('source').glob('*.C'),*src.glob('*.C')]:
  old=ac.blob(source.name).decode().splitlines();new=source.read_text().splitlines();mapping={}
  for block in difflib.SequenceMatcher(a=old,b=new,autojunk=False).get_matching_blocks():
   for offset in range(block.size):mapping[str(block.b+offset+1)]=block.a+offset+1
  line_maps[source.name]=mapping
 receipt['diagnostic_line_maps']=line_maps
 (BUILD/'identity.json').write_text(json.dumps(receipt,indent=2)+'\n');return exe
if __name__=='__main__':print(build())
