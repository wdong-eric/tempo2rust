"""Diagnostic-only comparison of the pinned AC preprocessing implementation."""
from pathlib import Path
import hashlib,importlib.util,io,json,os,re,subprocess,sys,tarfile,tempfile
sys.dont_write_bytecode=True
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust');BASE='18c9065'
OUT=Path(tempfile.mkdtemp(prefix='tempo2-preprocess-precision-'))
print('OUT',OUT,flush=True)
def sha(path):
 h=hashlib.sha256()
 with path.open('rb') as f:
  for b in iter(lambda:f.read(1048576),b''):h.update(b)
 return h.hexdigest()
archive=subprocess.check_output(['git','archive',BASE,'rust','validation/input_to_bat/check_preprocessing.py'],cwd=ROOT)
with tarfile.open(fileobj=io.BytesIO(archive)) as tar:tar.extractall(OUT/'snapshot')
snapshot=OUT/'snapshot';source=snapshot/'rust/src';p_test=source/'preprocessing_corpus_tests.rs'
text=p_test.read_text();matches=len(re.findall(r'Float::with_val\(\s*128\b',text));assert matches>0
# Both variants construct the fixture's exact dyadic values at the tested precision.
text=re.sub(r'Float::with_val\(\s*128\b','Float::with_val(crate::PRECISION_BITS',text)
p_test.write_text(text)
scalar=source/'longdouble.rs';original_scalar=scalar.read_text();assert original_scalar.count('pub const PRECISION_BITS: u32 = 128;')==1
rustc=Path(subprocess.check_output(['rustup','which','rustc'],text=True).strip());deps=ROOT/'rust/target/debug/deps'
externs={}
for name in ['rug','serde','serde_json','sha2']:
 paths=list(deps.glob('lib'+name+'-*.rlib'));assert len(paths)==1,(name,paths)
 externs[name]=paths[0]
dep_ids={p.name:sha(p) for p in externs.values()}
native_archives=sorted((ROOT/'rust/target/debug/build').glob('tempo2-rust-*/out/libtiming_native.a'))
assert native_archives, 'no retained SOFA archive'
native_ids={str(p):sha(p) for p in native_archives}
native_archive=native_archives[0]
compiler_environment={**os.environ,'CARGO_MANIFEST_DIR':str(ROOT/'rust'),'CARGO_PKG_VERSION':'0.1.0','CARGO_PKG_NAME':'tempo2-rust'}
runner_path=snapshot/'validation/input_to_bat/check_preprocessing.py'
spec=importlib.util.spec_from_file_location('ac_p_runner',runner_path);runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
runner.ROOT=ROOT;runner.HERE=ROOT/'validation/input_to_bat';runner.P=ROOT/'validation/preprocessing'
real_run=subprocess.run
protected={str(p):sha(p) for p in [ROOT/'validation/input_to_bat/build/handover-receipt.json',ROOT/'validation/input_to_bat/build/preprocessing/differences.json',ROOT/'validation/preprocessing/observations.json.gz',ROOT/'validation/preprocessing/initial.json.gz']}
records=[]
for bits in [128,113]:
 work=OUT/str(bits);work.mkdir()
 scalar.write_text(original_scalar.replace('pub const PRECISION_BITS: u32 = 128;',f'pub const PRECISION_BITS: u32 = {bits};'))
 executable=work/'preprocessing-tests'
 command=[str(rustc),'--crate-name','tempo2_rust','--edition=2024','--test',str(source/'lib.rs'),'-L','dependency='+str(deps),'-L','native='+str(native_archive.parent),'-l','static=timing_native','-o',str(executable)]
 for name,path in externs.items():command+=['--extern',name+'='+str(path)]
 with (work/'compile.log').open('w') as log:
  compiled=real_run(command,env=compiler_environment,stdout=log,stderr=subprocess.STDOUT)
 if compiled.returncode:raise RuntimeError((bits,'compile',compiled.returncode,(work/'compile.log').read_text()[-6000:]))
 print('compiled',bits,flush=True)
 runner.BUILD=work/'corpus'
 executions=[]
 def execute(command,*args,**kwargs):
  assert command[0]=='cargo',command
  actual=[str(executable),'preprocessing_corpus_tests::p_corpus','--ignored','--exact','--nocapture','--test-threads=1']
  with (work/'run.log').open('w') as log:
   result=real_run(actual,env=kwargs['env'],cwd=ROOT,stdout=log,stderr=subprocess.STDOUT)
  executions.append({'requested':command,'executed':actual,'returncode':result.returncode})
  return result
 runner.subprocess.run=execute;sys.argv=[str(runner_path)]
 try:runner.main()
 except SystemExit as e:code=e.code
 finally:runner.subprocess.run=real_run
 receipt=json.loads((runner.BUILD/'receipt-debug.json').read_text());assert receipt['returncode']==code
 differences=json.loads((runner.BUILD/'differences.json').read_text())
 diagnostics=json.loads((runner.BUILD/'diagnostic-differences.json').read_text())
 counts={}
 for d in differences:
  case=d['tag'].split()[0];kind=case.split('_')[0]
  counts[kind]=counts.get(kind,0)+1
 records.append({'bits':bits,'compile_command':command,'compiler_returncode':compiled.returncode,
                 'executable_sha256':sha(executable),'scalar_source_sha256':sha(scalar),'fixture_source_sha256':sha(p_test),
                 'cases':receipt['cases'],'returncode':code,'difference_records':len(differences),
                 'diagnostic_differences':len(diagnostics),'groups':counts,
                 'differences_sha256':sha(runner.BUILD/'differences.json'),'executions':executions})
 print(json.dumps(records[-1]),flush=True)
 if bits==128:
  assert len(differences)==154 and len(diagnostics)==0,'128-bit control did not reproduce delivered counts'
  old=json.loads((ROOT/'validation/input_to_bat/build/preprocessing/differences.json').read_text())
  def normalized(v):return json.dumps(v,sort_keys=True).replace(str(runner.BUILD),'{work}').replace(str(ROOT/'validation/input_to_bat/build/preprocessing'),'{work}')
  assert normalized(old)==normalized(differences),'128-bit control differs beyond execution paths'
for name,digest in protected.items():assert sha(Path(name))==digest,name
for p in externs.values():assert sha(p)==dep_ids[p.name],p
for path,digest in native_ids.items():assert sha(Path(path))==digest,path
result={'scope':'diagnosis only; no production precision decision','baseline':BASE,'fixture_precision_constructor_replacements':matches,
        'review_source_sha256':sha(Path(__file__)),'dependencies':dep_ids,'native_archive':str(native_archive),'observed_native_archives':native_ids,'protected_inputs':protected,'variants':records}
(OUT/'receipt.json').write_text(json.dumps(result,indent=2)+'\n')
print('RECEIPT',OUT/'receipt.json',sha(OUT/'receipt.json'),flush=True)
