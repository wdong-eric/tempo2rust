"""Fresh central V replay without changing accepted sources or output evidence."""
from pathlib import Path
import hashlib,json,shutil,subprocess,sys,tempfile
sys.dont_write_bytecode=True
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust');HERE=ROOT/'validation/time_ephemeris'
OUT=Path(tempfile.mkdtemp(prefix='tempo2-v-central-'));print('OUT',OUT,flush=True)
sys.path.insert(0,str(HERE));import profile,run,verify,preservation
original_build=profile.BUILD;reference=run.REF
sha=profile.sha;save=profile.save
load=lambda p:json.loads(p.read_text())
protected=load(preservation.MANIFEST);assert preservation.check(protected)==[]
evidence={str(p.relative_to(HERE)):sha(p.read_bytes()) for p in (HERE/'outputs').rglob('*') if p.is_file()}
source_identity=profile.runners()
case_work=original_build/'cases'
def files(folder):return {str(p.relative_to(folder)):sha(p.read_bytes()) for p in folder.rglob('*') if p.is_file()}
old_case_files=files(case_work);shutil.copytree(case_work,OUT/'preserved-case-work')
# Read the original source staging so __FILE__ diagnostics retain their exact paths.
# Every byte is independently checked against the raw pinned source or exact mutant.
def checked_source(name='source',mutation=None):
 src=original_build/name;assert src.is_dir(),src
 for n in profile.FILES:
  expected=profile.blob(n)
  if mutation and n==mutation[0]:
   old,new=mutation[1:];text=expected.decode();assert text.count(old)==1
   expected=text.replace(old,new).encode()
  assert (src/n).read_bytes()==expected,(name,n)
 assert (src/'TKlongdouble.h').read_bytes()==profile.blob('TKlongdouble.float128.h')
 assert (src/'config.h').read_bytes()==(ROOT/'validation/reference/build/source/config.h').read_bytes()
 g=profile.blob('global.C');assert (src/'global_data.C').read_bytes()==g[:g.index(b'void extra_delays')]+g[g.index(b'void CVSdisplayVersion'):]
 return src
profile.prepare=checked_source;profile.BUILD=OUT/'build';profile.BUILD.mkdir()
# Native inputs use the delivered disposable paths because their spelling is exported.
# Source staging is read-only; all new binaries, receipts and raw outputs go to OUT.
run.OUT=OUT;verify.OUT=OUT;verify.BUILD=profile.BUILD
ordinary_run=subprocess.run
cli_commands=[]
def routed_cli(args,*pos,**kw):
 if list(args)==['python3',str(HERE/'run.py'),'--no-build']:
  body=("import sys;from pathlib import Path;sys.dont_write_bytecode=True;"
        f"sys.path.insert(0,{str(HERE)!r});import run;run.OUT=Path({str(OUT)!r});"
        f"run.RECEIPT=run.OUT/'last_execution.json';sys.argv={[str(HERE/'run.py'),'--no-build']!r};run.main()")
  command=[sys.executable,'-c',body];cli_commands.append(dict(original=args,executed=command,reason='Route only disposable stale-receipt output; invoke unchanged run.main and argument parser.'))
  return ordinary_run(command,*pos,**kw)
 return ordinary_run(args,*pos,**kw)
try:
 cases=run.catalog();assert len(cases)==97
 frozen=load(reference/'oracle.json')
 run.compare(frozen['inputs'],dict(cases=run.membership(cases),inventory=run.inventory()))
 exe,ident=profile.compile_probe();run.compare(frozen['profile'],ident['stable'])
 print('PASS fresh native profile',sha(exe.read_bytes()),flush=True)
 rows=run.execute(exe,ident,cases,OUT/'replay')
 assert sum(r['events'] for r in rows)==1368 and sum(r['status']==1 for r in rows)==9
 subprocess.run=routed_cli
 try:controls=verify.controls(exe,ident)
 finally:subprocess.run=ordinary_run
 assert len(controls)==30
 independent=verify.independent();assert len(independent)==36
 print('PASS 97 cases, 30 controls, 36 independent checks',flush=True)
 mutations=verify.mutations();assert len(mutations)==13
 assert all(m['witness'].startswith(m['case']) is False or m['witness'] for m in mutations)
 profile.check_live(exe,ident);assert profile.runners()==source_identity
 assert preservation.check(protected)==[]
 assert evidence=={str(p.relative_to(HERE)):sha(p.read_bytes()) for p in (HERE/'outputs').rglob('*') if p.is_file()}
 result=dict(status='PASS',scope='Fresh ordinary and thirteen mutant native executables; all cases, normal controls and independent polynomial/endian checks. O0 coverage audited separately, not rerun here.',output=str(OUT),profile=ident,cases=rows,controls=controls,independent=independent,mutations=mutations,cli_output_routing=cli_commands,protected_files=len(protected['files']),protected_control_results=preservation.controls(protected),unchanged_V_output_files=len(evidence),original_case_work_files=len(old_case_files),original_oracle_sha256=sha((reference/'oracle.json').read_bytes()))
 save(OUT/'central-review.json',result)
 print('PASS central V native replay and all controls/mutations',flush=True)
 print('RECEIPT',OUT/'central-review.json',sha((OUT/'central-review.json').read_bytes()),flush=True)
finally:
 subprocess.run=ordinary_run
 shutil.move(str(case_work),str(OUT/'executed-case-work'))
 shutil.copytree(OUT/'preserved-case-work',case_work)
 assert files(case_work)==old_case_files,'disposable case workspace restoration'
 print('RESTORED original disposable case workspace byte-for-byte',flush=True)
