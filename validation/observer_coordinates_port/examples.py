"""Executable canonical-driver examples; these do not expand the public PAR/TIM grammar."""
import argparse,importlib.util,json,sys
from pathlib import Path
HERE=Path(__file__).resolve().parent
s=importlib.util.spec_from_file_location('aa_examples_run',HERE/'run.py');a=importlib.util.module_from_spec(s);s.loader.exec_module(a)
p=argparse.ArgumentParser(description=__doc__);p.add_argument('--profile',choices=['debug','release'],default='debug');p.add_argument('--example',choices=['all','terrestrial','satellite','vector-ut1'],default='all');args=p.parse_args()
exe=a.build(args.profile);ident=a.identity(exe);cases=a.fixtures.cases()
for name,label in [('terrestrial','driver_method_1_ecl_0'),('satellite','polynomial_axis_0_mask_15'),('vector-ut1',None)]:
 if args.example not in ('all',name):continue
 if label:c=next(c for c in cases if c['label']==label).copy()
 else:
  actions=[['profile'],['env','TEMPO2','{case}/root'],['pulsar','0','1','2','0','0','','eop'],['observation','0','58010.25','A','1','1','0']]
  actions += [['parameter','0',str(i),'0',str(v),'1'] for i,v in enumerate([1.234,-0.3,12,-8,0.1,0.2])]
  actions += [['vector','1'],['tai','1'],['driver','1']]
  c=dict(files=a.fixtures.resources(),actions=actions,classification='initialized-canonical-example')
 c['id']='example_'+name
 dest=a.HERE/'outputs/examples'/args.profile/name
 result=a.execute(c,exe,ident,dest);assert result['status']==0
 final=result['events'][-1]['caller'];summary=dict(example=name,scope='internal canonical state driver; general input grammar/orchestration pending',pulsar=final['pulsars'][0],observations=final['observations'][:final['pulsars'][0]['nobs']],result_sha256=a.sha(dest/'result.json'))
 a.save(dest/'summary.json',summary)
 print(name+': PASS; '+str(dest/'summary.json'))
