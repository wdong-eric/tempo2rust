"""Prove the isolated post-regression guard removal and run its focused gates."""
import json,subprocess,sys,time
from pathlib import Path
sys.dont_write_bytecode=True
from run import ROOT,RUST,HERE,OUT,ENV,sha,save,identity
from freeze import check

def main():
 prior=OUT/'before_guard_review';baseline=json.loads((prior/'delivery.json').read_text())
 name='rust/src/tropo.rs';before=(prior/'tropo.rs').read_text()
 assert sha(prior/'tropo.rs')==baseline['identity']['sources'][name]
 removed='''        if site.len() >= 256 {
            return Err(boundary("meteorology_site_storage"));
        }
'''
 replacement='''        // load() guarantees a '#' header of at most 255 bytes; this token
        // therefore fits siteName[256], including its NUL terminator.
'''
 assert before.count(removed)==1 and before.replace(removed,replacement)==(ROOT/name).read_text()
 old={p:h for p,h in baseline['identity']['sources'].items() if p.startswith('rust/src/')}
 current={str(p.relative_to(ROOT)):sha(p) for p in (RUST/'src').glob('*.rs')}
 assert old.keys()==current.keys() and [p for p in old if old[p]!=current[p]]==[name]
 # Existing frontend examples/build/native algorithm inputs also stay exact.
 closure=json.loads((OUT/'R/O/regressions/source_hashes.json').read_text())
 for p,h in closure.items():
  if p!=name:assert sha(ROOT/p)==h,p
 for profile in ['debug','release']:
  record=json.loads((prior/profile/'direct.json').read_text())
  assert sha(prior/'executables'/profile)==record['identity']['executable_sha256']
 started=identity();protected=check();commands=[]
 def command(label,args):
  print(label,flush=True);start=time.monotonic();p=subprocess.run(list(map(str,args)),cwd=ROOT,env=ENV,capture_output=True,text=True)
  log=OUT/(label+'.log');log.write_text(p.stdout+p.stderr)
  commands.append(dict(name=label,command=list(map(str,args)),returncode=p.returncode,elapsed_s=time.monotonic()-start,log=log.name,sha256=sha(log)))
  save(OUT/'final_commands.json',commands);assert p.returncode==0,(label,p.stdout[-2000:],p.stderr[-2000:])
 command('final_fmt',['cargo','fmt','--manifest-path',RUST/'Cargo.toml','--check'])
 command('final_clippy',['cargo','clippy','--manifest-path',RUST/'Cargo.toml','--locked','--offline','--all-targets','--','-D','warnings'])
 for test in ['clock_tests::byte_parsing_and_operation_local_table_boundaries','resource_tests::meteorology_valid_zero_missing_and_outside_coverage']:
  command('final_'+test.split('::')[-1],['cargo','test','--manifest-path',RUST/'Cargo.toml','--locked','--offline','--lib',test,'--','--exact'])
 profiles={}
 for profile in ['debug','release']:
  command('final_'+profile+'_U',[sys.executable,HERE/'run.py','--profile',profile])
  r=json.loads((OUT/profile/'direct.json').read_text());assert r['status']=='PASS' and len(r['cases'])==130
  assert identity(Path(r['identity']['executable']))==r['identity']
  profiles[profile]=dict(receipt_sha256=sha(OUT/profile/'direct.json'),controls_sha256=sha(OUT/profile/'controls.json'),executable_sha256=r['identity']['executable_sha256'])
 assert identity()==started and check()==protected
 save(OUT/'final_scope.json',dict(status='Final W focused checks PASS; full regression evidence is explicitly pre-guard-removal; central acceptance pending',identity=started,production_delta={name:dict(before=old[name],after=current[name],change='Remove site length guard already guaranteed by the successful table loader')},prior_full_receipt_sha256=sha(prior/'delivery.json'),prior_full_executed_head=baseline['executed_head'],final_head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),profiles=profiles,commands_sha256=sha(OUT/'final_commands.json'),protected_frozen_entries=protected,M2_failures=baseline['M2_failures']))
 print('Final delta is exactly one unreachable guard; focused loader/meteorology checks and all 130 U histories pass in both profiles.',flush=True)
if __name__=='__main__':main()
