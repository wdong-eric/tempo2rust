"""Finalize Y evidence after fresh direct and staged regression executions."""
import importlib.util,json,subprocess,sys,time
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1]

def main():
 spec=importlib.util.spec_from_file_location('y_delivery_run',HERE/'run.py');y=importlib.util.module_from_spec(spec);spec.loader.exec_module(y)
 sys.path.insert(0,str(HERE));from freeze import check
 out=y.OUT;receipt=out/'delivery.json';receipt.unlink(missing_ok=True);before=y.identity();protected=check();commands=[]
 def command(name,args):
  start=time.monotonic();p=subprocess.run(args,cwd=y.RUST,env=y.ENV,capture_output=True,text=True)
  log=out/'checks'/(name+'.log');log.parent.mkdir(parents=True,exist_ok=True);log.write_text(p.stdout+p.stderr)
  commands.append(dict(name=name,command=args,status=p.returncode,elapsed_s=time.monotonic()-start,log=str(log),sha256=y.sha(log)))
  y.save(out/'checks/commands.json',commands);assert p.returncode==0,(name,p.stdout[-2000:],p.stderr[-2000:]);print('PASS',name,flush=True)
 command('fmt',['cargo','fmt','--check'])
 command('clippy',['cargo','clippy','--locked','--offline','--all-targets','--','-D','warnings'])
 for profile in ['debug','release']:
  args=['cargo','test','--locked','--offline','--lib']+(['--release'] if profile=='release' else [])
  for name in ['time_ephemeris_tests::','resource_tests::ifte_']:
   command(profile+'-'+name.replace(':',''),args+[name,'--','--test-threads=1'])
  for name in ['stdio::tests::ownership_handoff_read_and_close','stdio::tests::descriptor_transfer_partial_items_and_failure_cleanup','pulsar_lifecycle_tests::lifecycle_logging_after_clock_fault']:
   command(profile+'-'+name.replace(':',''),args+[name,'--','--exact','--ignored'])
 def read(path):return json.loads(path.read_text())
 def live(identity):
  for name,digest in identity['sources'].items():assert y.sha(ROOT/name)==digest,('source drift',name)
  for name in ['compiler','cargo','executable']:
   if name in identity:assert y.sha(Path(identity[name]))==identity[name+'_sha256'],name
 profiles={}
 for profile in ['debug','release']:
  d=read(out/profile/'direct.json');assert d['complete'] and not d['failures'];assert y.identity(Path(d['identity']['executable']))==d['identity']
  assert len(d['cases'])==97 and sum(c['events'] for c in d['cases'])==1368 and sum(c['status']==1 for c in d['cases'])==9
  c=read(out/profile/'controls.json');assert c['independent']==36;assert y.sha(Path(c['path']))==c['sha256']
  for row in d['cases']:assert y.sha(Path(d['run_directory'])/row['id']/'result.json')==row['result_sha256']
  w=read(HERE/'w'/profile/'direct.json');live(w['identity']);assert w['status']=='PASS' and len(w['cases'])==130
  t=read(HERE/'build/regressions/T'/profile/'direct.json');live(t['identity']);assert t['status']=='PASS' and len(t['cases'])==30 and sum(r['events'] for r in t['cases'])==133
  helper=read(HERE/'build/regressions/T'/profile/'helpers/receipt.json');live(helper['identity']);assert helper['status']=='PASS'
  profiles[profile]=dict(cases=97,events=1368,native_exits=9,controls=c['count'],independent=c['independent'],identity=d['identity'],direct_sha256=y.sha(out/profile/'direct.json'),W_cases=130,T_cases=30,T_events=133)
 native=read(out/'native/receipt.json');y.native_profile.check_live(Path(native['profile']['executable_path']),native['profile'])
 mutations=read(out/'mutations/receipt.json');assert len(mutations['mutations'])==13
 for m in mutations['mutations']:y.native_profile.check_live(Path(m['profile']['executable_path']),m['profile']);assert y.sha(Path(m['path']))==m['sha256']
 r=HERE/'build/regressions/R';reg=read(r/'delivery_checks.json');m2=read(r/'O/regressions/m2/open_gate_failures.json');assert m2==read(y.RUST/'verification/m2/open_gate_failures.json') and len(m2)==4
 assert y.identity()==before and check()==protected
 artifacts=[out/'native/receipt.json',out/'native/correction-deltas.json',out/'mutations/receipt.json',out/'checks/commands.json',r/'delivery_checks.json',r/'O/delivery_checks.json',r/'O/regressions/delivery_checks.json',HERE/'build/freeze.json',HERE/'build/arithmetic/receipt.json',HERE/'build/regressions/routing/staging.json',HERE/'build/regressions/routing/unsafe_audit.json']
 y.save(out/'module_inventory.json',{str(p.relative_to(ROOT)):dict(sha256=y.sha(p),unsafe_lines=[i for i,l in enumerate(p.read_text().splitlines(),1) if 'unsafe ' in l]) for p in sorted(y.RUST.joinpath('src').glob('*.rs'))})
 y.save(receipt,dict(status='Y checks pass; central acceptance pending; M2 and 0049 unresolved',protected_files=protected,profiles=profiles,mutations=13,M2_failures=m2,artifacts={str(p.relative_to(ROOT)):y.sha(p) for p in artifacts},module_inventory_sha256=y.sha(out/'module_inventory.json'),executed_head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()))
 print('PASS Y delivery; exact existing M2 failures retained',flush=True)
if __name__=='__main__':main()
