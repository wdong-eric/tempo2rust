"""Shared-phase refit localization; it does not waive any pipeline comparison."""
from pathlib import Path
import argparse, json, subprocess, hashlib
from decimal import Decimal as D
from compare import read_tsv
from run_public import ROOT, HERE
p=argparse.ArgumentParser();p.add_argument('--rust',type=Path,required=True);p.add_argument('--public',type=Path,required=True);p.add_argument('--out',type=Path,required=True);a=p.parse_args();a.out.mkdir(parents=True,exist_ok=False);records=[]
for name,fixture,site in [('binary-bary','binary','@'),('isolated-pks','isolated','PKS'),('binary-pks','binary','PKS'),('ecliptic-tdb','ecliptic_tdb','PKS'),('defaults-odd','defaults','PKS')]:
    case=a.public/(name+'-polyco');ref=case/'native/result.tsv';cmd=[str(a.rust.resolve()),str(HERE/'fixtures'/(fixture+'.par')),str(ROOT/'validation/reference/build/source/T2runtime'),site,str(ref.resolve()),str((a.out/name).resolve())]
    result=subprocess.run(cmd,text=True,capture_output=True);(a.out/(name+'.stdout')).write_text(result.stdout);(a.out/(name+'.stderr')).write_text(result.stderr);assert result.returncode==0,result.stderr
    record=json.loads(result.stdout);native=read_tsv(ref);record['name']=name;record['command']=cmd;record['native_log10rms']=[str(v) for (k,i),v in native.items() if k=='log10rms']
    assert len(record['shared_phase_log10rms'])==len(record['native_log10rms'])
    record['shared_native_rounding_cells_match']=all(x==float(y) for x,y in zip(record['shared_phase_log10rms'],record['native_log10rms']));assert record['shared_native_rounding_cells_match']
    observations=json.loads((case/'rust/generate.stdout').read_text())['observations'];record['maximum_original_phase_difference_turn']=str(max(abs(D(o['phase_turn'])-native['phase',i]) for i,o in enumerate(observations)))
    records.append(record)
(a.out/'receipt.json').write_text(json.dumps({'rust':hashlib.sha256(a.rust.read_bytes()).hexdigest(),'records':records},indent=2)+'\n');print('All 10 shared-phase log10RMS values match native rounding cells')
