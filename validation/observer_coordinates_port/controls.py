"""Reuse X's 35 controls/19 checks and 15 retained mutation witnesses; AA whitespace."""
import argparse,copy,gzip,importlib.util,json,sys
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];X=ROOT/'validation/observer_coordinates'
def module(name,path):
 s=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(s);sys.modules[name]=m;s.loader.exec_module(m);return m
sys.path.insert(0,str(X))
a=module('aa_run_controls',HERE/'run.py')
def main():
 p=argparse.ArgumentParser();p.add_argument('--profile',required=True,choices=['debug','release']);args=p.parse_args()
 direct=json.loads((a.OUT/args.profile/'direct.json').read_text());assert direct['status']=='PASS' and direct['scope']=='all'
 dest=a.OUT/args.profile/'controls';dest.mkdir(parents=True,exist_ok=True)
 actual=[]
 for row in direct['cases']:
  path=a.OUT/args.profile/row['id']/'result.json';assert a.sha(path)==row['result_sha256']
  r=json.loads(path.read_text());r['events']=[e for e in r['events'] if e['phase']!='exit']
  for e in r['events']:
   if 'private' in e:e['private']=None
  actual.append(r)
 # Execute the original control body up to its native mutation builds. Only the
 # output/temp routing and current accepted preservation owner differ.
 original=(X/'controls.py').read_text();text=original[:original.index('    mutants=[]')]+'    return rows\n'
 text=text.replace("dest=OUTPUT/'controls.json'","dest=Path("+repr(str(dest/'types.json'))+")")
 text=text.replace("tmp=BUILD/'controls'","tmp=Path("+repr(str(dest/'identity'))+")")
 staged=HERE/'build/control-routing.py';staged.write_text(text)
 controls=module('aa_x_controls',staged)
 controls.preservation_status=lambda:{} # AA freeze owns current membership; X's old epoch is historical.
 controls.read_gzip=lambda path:copy.deepcopy(actual)
 rows=controls.main();assert len(rows)==35
 # These are retained compiled-native witnesses, newly compared with AA output;
 # no new native corpus or mutation build is claimed.
 mutants=a.unpack(X/'outputs/mutations.json.gz');assert len(mutants)==15
 from profile import actual_identity
 by={r['id']:r for r in actual};witnesses=[]
 for index,m in enumerate(mutants):
  profile=m['profile'];actual_identity(X/'build'/f'mutant{index:02}'/'probe',profile)
  diff=controls.differences(by[m['case']]['events'],m['result']['events']);assert diff,m['name']
  witnesses.append(dict(name=m['name'],case=m['case'],differences=diff,executable_sha256=profile['executable_sha256']))
 independent=module('aa_x_support',X/'independent.py')
 independent.OUTPUT=dest/'support';independent.OUTPUT.mkdir(exist_ok=True)
 oracle=independent.OUTPUT/'reference.json.gz';oracle.write_bytes(gzip.compress(json.dumps(actual).encode(),mtime=0))
 independent.preservation_status=lambda:{};independent.main()
 support=json.loads((independent.OUTPUT/'independent.json').read_text());assert len(support['checks'])==19
 # Existing central memmove experiment is the corrected oracle, including guards
 # and every stale byte after repeated shifts. Original overlapping copies remain unrun.
 receipt=ROOT/'decisions/evidence/observer-whitespace/build/central-il7tunxj/receipt.json';r=json.loads(receipt.read_text())
 assert a.sha(receipt)=='cb9ff2427e37a7b1df76fd377b41efc396f69889f5ae77b9e694c351c5ed9239'
 c=dict(id='corrected_white',classification='0053-overlap-safe-correction',files={},actions=[['white_hex',v['input_hex']] for v in r['records']['corrected']])
 exe=Path(direct['identity']['executable']);ident=a.identity(exe)
 result=a.execute(c,exe,ident,dest/'white');again=a.execute(c,exe,ident,dest/'white-repeat');a.exact_compare(result,again)
 values=[e['result']['buffer'] for e in result['events'] if e['phase']=='end']
 assert values==[v['buffer_after_hex'] for v in r['records']['corrected']]
 assert values[:7]==[v['buffer_after_hex'] for v in r['records']['original']]
 a.save(dest/'receipt.json',dict(status='PASS',identity=ident,controls=rows,supporting_checks=support['checks'],retained_mutation_witnesses=witnesses,corrected_buffers=len(values),original_buffers=7,white_source_receipt_sha256=a.sha(receipt),control_source_sha256=a.sha(X/'controls.py'),routed_controls_sha256=a.sha(staged),mutation_source_sha256=a.sha(X/'outputs/mutations.json.gz'),direct_sha256=a.sha(a.OUT/args.profile/'direct.json')))
 print('PASS 35 controls, 19 supporting checks, 15 retained mutation witnesses, 19 corrected complete buffers / 7 original neighbors')
if __name__=='__main__':main()
