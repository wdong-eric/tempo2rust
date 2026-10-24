"""Additional focused witnesses for angular, output, annual and retained-state branches."""
import math
from inventory import HERE,sha,save,check
from profile import compile_probe
from controls import differences
from run import OUTPUT,read_gzip,write_gzip,execute,frozen_cases,compare

MUTATIONS=[
 ('deleted_termination','iteration_fast_0_deleted_1','calculate_bclt.C','psr[p].obsn[i].deleted!=1','psr[p].obsn[i].deleted!=7'),
 ('annual_sine','annual','dm_delays.C','val[0]*sin(2*M_PI/(365.25)*dt);','val[0]*cos(2*M_PI/(365.25)*dt);'),
 ('other_solar_retention','solar_selector_2_1','dm_delays.C','    //        logdbg("Exiting dm_delays with pulsar %d; number of obs = %d",p,psr[p].nobs);','    if(psr[p].swm==2) psr[p].obsn[i].tdis2=0; // retained-state test mutation'),
 ('antipodal_angle','angle_antipodal','sw_delay.C','c = M_PI;','c = M_PI/2;'),
 ('output_one_step_wrap','output_wrapping','sw_delay.C','if (psrx[i] <0) psrx[i] += 360.0;','if (psrx[i] <0) psrx[i] += 720.0;')]
def main():
 check();catalog=frozen_cases();baseline=read_gzip(OUTPUT/'reference.json.gz');hash_before=sha((OUTPUT/'reference.json.gz').read_bytes())
 by={c['id']:(c,r) for c,r in zip(catalog,baseline)};compare(list(by),[c['id'] for c in catalog]);rows=[]
 for i,(name,label,file,old,new) in enumerate(MUTATIONS):
  if label=='angle_antipodal':label=next(c['id'] for c in catalog if c['actions'][-1]==['angle','0','0',str(math.pi),'0'])
  c,expected=by[label];exe,p=compile_probe(f'branch_mutant{i:02}',(file,old,new));actual,receipt=execute(c,exe,p,OUTPUT/'branch_mutations'/name)
  witness=differences(expected,actual);assert witness,(name,'no behavior witness')
  rows.append(dict(name=name,case=label,classification='deliberate-control-not-correction',witnesses=witness,profile=p,receipt=receipt,result=actual))
  print('PASS mutation',name,witness[0]['path'],flush=True)
 check();assert sha((OUTPUT/'reference.json.gz').read_bytes())==hash_before
 write_gzip(OUTPUT/'branch_mutations.json.gz',rows)
 save(OUTPUT/'branch_controls.json',dict(status='PASS',script_sha256=sha((HERE/'branch_controls.py').read_bytes()),mutations=len(rows),mutation_sha256=sha((OUTPUT/'branch_mutations.json.gz').read_bytes())))
if __name__=='__main__':main()
