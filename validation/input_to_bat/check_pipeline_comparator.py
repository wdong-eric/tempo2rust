#!/usr/bin/env python3
"""Targeted membership/quantity-unit mutations of actual staged integration rows."""
import copy,json
from pathlib import Path
import check_pipeline as pipeline
for profile in ['debug','release']:
 paths=list(pipeline.BUILD.glob('*/rust-'+profile+'.json'))
 if not paths:continue
 checked=[]
 for path in paths:
  result=json.loads(path.read_bytes());reference=path.with_name('native.tsv').read_text()
  errors,measurements=pipeline.compare(reference,result);assert not errors,(path,errors)
  mutants=[]
  for label,count in [('drop_all',0),('drop_last',len(result['pulsars'])-1)]:
   mutant=copy.deepcopy(result);mutant['pulsars']=mutant['pulsars'][:count];mutants.append((label,mutant))
  for p,pulsar in enumerate(result['pulsars']):
   for i,row in enumerate(pulsar['observations']):
    if row['delay_correction']!=0:continue
    mutant=copy.deepcopy(result);mutant['pulsars'][p]['observations'][i]['frequency_ssb']['value']=str(pipeline.Decimal(row['frequency_ssb']['value'])+pipeline.Decimal('0.000001'));mutants.append(('one_hz_in_mhz',mutant))
    mutant=copy.deepcopy(result);mutant['pulsars'][p]['observations'][i]['frequency_ssb']['unit']='Hz';mutants.append(('incorrect_frequency_unit',mutant))
  mutant=copy.deepcopy(result);mutant['pulsars'][0]['observations'][0]['corrections']['roemer']['unit']='day';mutants.append(('incorrect_delay_unit',mutant))
  for label,mutant in mutants:
   errors,_=pipeline.compare(reference,mutant);assert errors,(path,label,'false pass')
  checked.append({'case':path.parent.name,'measurements':len(measurements),'rejected_mutations':[label for label,_ in mutants]})
 report={'profile':profile,'comparator_sha256':pipeline.native.sha(Path(pipeline.__file__).read_bytes()),'cases':checked}
 (pipeline.BUILD/('comparator-controls-'+profile+'.json')).write_text(json.dumps(report,indent=2)+'\n')
 print(profile,len(checked),'baselines passed; all membership/unit mutations rejected')
