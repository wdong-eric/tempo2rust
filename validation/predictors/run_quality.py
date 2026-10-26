"""Independent off-node timing comparison; approximation and fidelity are separate."""
from pathlib import Path
import argparse, json, os
from decimal import Decimal as D
from run_public import ROOT, HERE, run, sha
from compare import Checks, read_tsv, phase_bound, hz_bound
p=argparse.ArgumentParser();p.add_argument('--native',type=Path,required=True);p.add_argument('--rust',type=Path,required=True);p.add_argument('--public',type=Path,required=True);p.add_argument('--out',type=Path,required=True);a=p.parse_args();a.out.mkdir(parents=True,exist_ok=False)
runtime=ROOT/'validation/reference/build/source/T2runtime';env=dict(os.environ,TEMPO2=str(runtime));receipt={'native':sha(a.native),'rust':sha(a.rust),'cases':[]}
for name in ['isolated-bary','binary-bary','isolated-pks','binary-pks']:
 for kind,files in [('cheby',['t2pred.dat']),('polyco',['polyco_new.dat','newpolyco.dat'])]:
  for filename in files:
   for owner in ['native','rust']:
    case=(a.out/(name+'-'+kind+'-'+filename+'-'+owner)).resolve();case.mkdir();file=(a.public/(name+'-'+kind)/owner/filename).resolve();par=HERE/'fixtures'/('binary.par' if name.startswith('binary') else 'isolated.par')
    row={'name':name,'kind':kind,'file':filename,'owner':owner,'file_sha256':sha(file)};receipt['cases'].append(row)
    commands=[run([a.native.resolve(),'quality',case/'native.tsv',file,par],case,env,'native'),run([a.rust.resolve(),file,par,runtime,case],case,env,'rust')];row['commands']=commands
    if any(r['returncode'] for r in commands):row['execution_failure']=True;continue
    ref=read_tsv(case/'native.tsv');actual=json.loads((case/'rust.stdout').read_text());c=Checks();records=actual['records'];c.exact('point_count',len(records),sum(k=='mjd' for k,i in ref));f0=D(actual['f0_hz']);errors={'native':[],'rust':[]};hz={'native':[],'rust':[]}
    for i,r in enumerate(records):
     c.number(f'mjd[{i}]',r['mjd'],ref['mjd',i]);c.number(f'freq[{i}]',r['frequency_mhz'],ref['freq',i]);c.number(f'full_phase[{i}]',r['full_phase_turn'],ref['full_phase',i],'1e-10')
     c.number(f'phase[{i}]',r['phase_turn'],ref['phase',i],phase_bound(abs(ref['phase',i])));c.number(f'frequency[{i}]',r['frequency_hz'],ref['frequency',i],hz_bound(abs(ref['frequency',i])))
     for owner1,phase,full,freq,fullfreq in [('native',ref['phase',i],ref['full_phase',i],ref['frequency',i],ref['full_frequency',i]),('rust',D(r['phase_turn']),D(r['full_phase_turn']),D(r['frequency_hz']),D(r['full_centered_frequency_hz']))]:
      errors[owner1].append(abs(phase-full));hz[owner1].append(abs(freq-fullfreq))
    row['fidelity']={'checks':c.count,'failures':c.failures};row['approximation']={owner1:{'max_phase_turn':str(max(v)),'max_time_seconds':str(max(v)/f0),'rms_phase_turn':str((sum(x*x for x in v)/len(v)).sqrt()),'max_centered_frequency_error_hz':str(max(hz[owner1])),'target_seconds':'1e-6','passes':max(v)/f0<=D('1e-6')} for owner1,v in errors.items()}
    (a.out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');print(name,kind,filename,owner,'fidelity',len(c.failures),'max_seconds',row['approximation']['rust']['max_time_seconds'],flush=True)
(a.out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')

raise SystemExit(any(r.get('execution_failure') or r['fidelity']['failures'] or any(not v['passes'] for v in r['approximation'].values()) for r in receipt['cases']))
