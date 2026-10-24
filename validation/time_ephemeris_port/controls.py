"""Y recursive type, actual identity and original/corrected behavior controls."""
import copy,gzip,importlib.util,json,subprocess,sys
from pathlib import Path

def checks(run,profile,exe,ident,cases,dest):
 rows=[]
 def reject(name,f):
  try:f()
  except (AssertionError,ValueError,KeyError) as e:rows.append(dict(name=name,rejected=True,witness=str(e)[:300]));return
  raise AssertionError('accepted corrupt input: '+name)
 def payload(name):return run.project(json.loads((dest/name/'result.json').read_text()))
 frozen=payload('ifte_reinit')
 edits={
  'missing case identity':lambda x:x.pop('id'),
  'missing event':lambda x:x['events'].pop(3),
  'action order':lambda x:x['events'].reverse(),
  'missing field':lambda x:x['events'][1]['state'].pop('ifte.pc'),
  'field order':lambda x:x['events'][1].update(state=dict(reversed(list(x['events'][1]['state'].items())))),
  'retained tail':lambda x:x['events'][1]['state']['output'][-1].update(bits='0000000000000000'),
  'knownness':lambda x:x['events'][1]['state'].update({'ifte.title':{'unwritten':True}}),
  'cache':lambda x:x['events'][5]['state'].update({'ifte.irec':2}),
  'owner lifetime':lambda x:x['events'][7]['state'].update({'old0.owner':'released-by-caller'}),
  'signed zero':lambda x:x['events'][1]['state']['ifte.twot'].update(bits='0000000000000080'),
  'numeric class':lambda x:x['events'][1]['state']['ifte.twot'].update({'class':'NaN'}),
  'stdout':lambda x:x.update(stdout='20'),'stderr':lambda x:x.update(stderr='20'),'status':lambda x:x.update(status=1),
  'resource file':lambda x:x['final_files'].update(eph='0'*64),
  'nested integer to bool':lambda x:x['events'][1]['state'].update({'ifte.irec':False}),
  'nested integer to float':lambda x:x['events'][1]['state'].update({'ifte.irec':0.0}),
  'array integer to bool':lambda x:x['events'][1]['state']['ifte.ipt0'].__setitem__(0,False),
  'array integer to float':lambda x:x['events'][1]['state']['ifte.ipt0'].__setitem__(0,0.0),
  'known byte count to bool':lambda x:x['events'][1]['state']['ifte.title'].update(known_bytes=True),
  'nested bits to number':lambda x:x['events'][1]['state']['ifte.twot'].update(bits=0),
 }
 for name,edit in edits.items():
  actual=copy.deepcopy(frozen);edit(actual);reject(name,lambda:run.compare(frozen,actual))
 for field in ['executable_sha256','compiler_sha256','sources']:
  stale=copy.deepcopy(ident);stale[field]='stale';reject('stale '+field,lambda:run.execute(cases[0],exe,stale,dest/'controls/never'))
 changed=dest/'controls/changed-executable';changed.parent.mkdir(parents=True,exist_ok=True);changed.write_bytes(exe.read_bytes()+b'control')
 reject('actual changed executable',lambda:run.execute(cases[0],changed,ident,dest/'controls/never'));changed.unlink()
 ids=[c['id'] for c in cases];reject('missing case',lambda:run.compare(ids,ids[:-1]));reject('case order',lambda:run.compare(ids,list(reversed(ids))))
 meta=run.fixtures.membership(cases)
 for name,edit in [('input action',lambda x:x[0]['actions'][0].__setitem__(0,'other')),('input resource',lambda x:x[-1]['files'][next(iter(x[-1]['files']))].update(sha256='0'*64))]:
  bad=copy.deepcopy(meta);edit(bad);reject(name,lambda:run.compare(meta,bad))
 # Actual native execution identity controls retain the accepted V machinery.
 native=json.loads((run.OUT/'native/receipt.json').read_text())['profile'];nexe=Path(native['executable_path'])
 for name,edit in [('native executable',lambda p:p.update(executable_sha256='0'*64)),('native compiler',lambda p:p['stable'].update(compiler_sha256='0'*64)),('native runtime',lambda p:p['stable']['runtime']['libquadmath.0.dylib'].update(sha256='0'*64)),('native source',lambda p:p['stable']['sources'].update({'ifteph.C':'0'*64})),('native checkout',lambda p:p['stable']['checkout_sources'].update({'tt2tdb.C':'0'*64})),('native runner',lambda p:p['stable']['runner_sources'].update({'run.py':'0'*64})),('native config',lambda p:p['stable']['generated'].update({'config.h':'0'*64})),('native header',lambda p:p['stable']['header_dependencies'].update({next(iter(p['stable']['header_dependencies'])):'0'*64}))]:
  bad=copy.deepcopy(native);edit(bad);reject(name,lambda:run.native_profile.check_live(nexe,bad))
 case=next(c for c in cases if c['id']=='ifte_reinit');fresh=run.project(run.execute(case,exe,ident,dest/'controls/fresh'));run.compare(frozen,fresh);rows.append(dict(name='fresh challenged positive',passed=True))
 # Real launch challenge and input checks, not only stored result comparison.
 launch=json.loads((dest/'controls/fresh/events.jsonl').read_text().splitlines()[0]);original_launch=json.loads((dest/'ifte_reinit/events.jsonl').read_text().splitlines()[0]);assert launch['nonce']!=original_launch['nonce']
 for key in ['nonce','input_sha256']:
  bad=copy.deepcopy(launch);bad[key]='stale';reject('launch '+key,lambda:run.compare(launch,bad))
 # Re-run all 36 independent analytic checks on the actual Rust exports by
 # supplying the unchanged V checker with its expected record transport.
 spec=importlib.util.spec_from_file_location('v_controls',run.V/'verify.py');v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)
 v.REF=dest;v.unpack=lambda path:dict(id=path.stem.replace('.json',''),events='\n'.join(json.dumps(e) for e in json.loads((path.parent/path.stem.replace('.json','')/'result.json').read_text())['events']))
 v.projected=lambda raw:dict(events=[json.loads(l) for l in raw['events'].splitlines()])
 independent=v.independent();assert len(independent)==36
 # Independently time the same supplied CALCEPH model alone and as pulsar 1.
 # Recognized IF99 and FB90 selectors still execute their real readers; the
 # full driver also initializes/closes IFTE and processes another model first.
 files=next(c for c in cases if c['id']=='tt_mixed_calceph_backing')['files']
 for selector in [1,2]:
  results=[]
  for multiple in [False,True]:
   index=int(multiple);name=f'y_calceph_{selector}_{index}'
   actions=[['root','@A'],['p',index,'timeEphemeris',selector],['p',index,'useCalceph',1],['p',index,'units',2]]
   if not multiple:actions += [['obs',0,i,'correctionTT_calcEph',0.375+i*0.0625] for i in range(3)]
   case=dict(id=name,actions=actions+[['tt',index+1,'present']],files=files,status=0)
   actual=run.project(run.execute(case,exe,ident,dest/'controls'/name));assert actual['status']==0
   state=actual['events'][-1]['state'];assert state['ifte.owner']=='released-by-tt'
   results.append([[state[f'p{index}.o{i}.'+field] for field in ['correctionTT_TB','correctionTT_Teph','einsteinRate']] for i in range(3)])
  run.compare(*results);rows.append(dict(name=f'full-driver CALCEPH single/multiple selector {selector}',passed=True))
 # These actual truncated external records are native-uninitialized cases,
 # so check the named Rust boundary instead of inventing a native golden.
 for count in range(4):
  name=f'y_record_short_{count}';target=dest/'controls'/name
  case=dict(id=name,actions=[['open','@short'],['read','record']],files={'short':b'abc'[:count]},status=2)
  actual=run.execute(case,exe,ident,target);assert actual['status']==2
  error=json.loads((target/'events.jsonl.error.json').read_text())
  assert error['field']=='Fortran_incomplete_record_int'
  rows.append(dict(name=f'incomplete record int {count} bytes',boundary=error['field']))
 accepted=json.loads((run.V/'outputs/verification-mutations-final.json').read_text());assert len(accepted['controls'])==30 and len(accepted['mutations'])==13
 # Preserve the actual old source-bug witnesses and reject their numerical
 # fields where the selected correction now has a different target.
 for case,field in [('ifte_reinit','output'),('fb_cache','output'),('tt_multi_calc_1','p1.o0.correctionTT_TB')]:
  old=run.project(run.native_payload(run.unpack(run.V/'outputs/reference-corrected'/(case+'.json.gz'))))
  new=payload(case)
  a=[e['state'][field] for e in old['events'] if 'state'in e];b=[e['state'][field] for e in new['events'] if 'state'in e]
  reject('original '+case+' bug rejected',lambda:run.compare(a,b))
 # CLI invocation uses a disposable output root via a staged routing copy;
 # original success receipts remain intact while testing early removal.
 script=(run.HERE/'run.py').read_text();before="OUT=HERE/'outputs'";assert script.count(before)==1
 scratch=dest/'controls/cli';scratch.mkdir(parents=True,exist_ok=True);staged=scratch/'run.py'
 # Keep the original ROOT/HERE context; only the receipt destination differs.
 script=script.replace("HERE=Path(__file__).resolve().parent",f"HERE=Path({str(run.HERE)!r})").replace(before,f"OUT=Path({str(scratch/'outputs')!r})")
 staged.write_text(script);stale=scratch/'outputs'/profile/'direct.json';run.save(stale,{'stale_success':True})
 p=subprocess.run([sys.executable,str(staged),'--profile',profile,'--no-build'],capture_output=True,text=True)
 assert p.returncode!=0 and not stale.exists();rows.append(dict(name='real CLI no-build rejected and stale success removed',status=p.returncode,stderr=p.stderr))
 assert run.identity(exe)==ident
 run.save(dest/'controls.json',dict(controls=rows,independent=independent,accepted_V_controls=30,accepted_V_mutations=13,accepted_V_sha256=run.sha(run.V/'outputs/verification-mutations-final.json')))
 run.save(run.OUT/profile/'controls.json',dict(path=str(dest/'controls.json'),sha256=run.sha(dest/'controls.json'),count=len(rows),independent=len(independent)))
 print('PASS controls',len(rows),'independent',len(independent),flush=True)
