"""CLI malformed input/output boundaries and valid recovery, with immutable inputs."""
from pathlib import Path
import argparse,json,os
from run_public import run,ROOT,sha
p=argparse.ArgumentParser();p.add_argument('--rust',type=Path,required=True);p.add_argument('--public',type=Path,required=True);p.add_argument('--out',type=Path,required=True);a=p.parse_args();a.out=a.out.resolve();a.out.mkdir(parents=True,exist_ok=False);rust=a.rust.resolve();env=dict(os.environ);records=[]
good=(a.public/'isolated-bary-cheby/rust/t2pred.dat').resolve();source=good.read_text()
mutations={'missing_dimension':source.replace('NCOEFF_TIME 8\n',''),'negative_dimension':source.replace('NCOEFF_FREQ 4','NCOEFF_FREQ -1'),'derivative_extent':source.replace('NCOEFF_TIME 8','NCOEFF_TIME 1'),'incomplete_row':source.replace('COEFFS','COEFFS BAD',1),'empty':''}
for name,content in mutations.items():
 file=a.out/(name+'.dat');file.write_text(content);cmd=[rust,'predictor','evaluate','--input',file,'--mjd','58000.01','--frequency-mhz','1400'];r=run(cmd,a.out,env,name);assert r['returncode'] and not (a.out/(name+'.stdout')).read_bytes();records.append(r)
 r=run([rust,'predictor','evaluate','--input',good,'--mjd','58000.01','--frequency-mhz','1400'],a.out,env,name+'-recovered');assert r['returncode']==0;records.append(r)
# T1 accepts a completed prefix, while preserving its trailing parse diagnostic.
t1=a.out/'trailing.dat';t1.write_bytes((a.public/'isolated-bary-polyco/rust/newpolyco.dat').read_bytes()+b'malformed trailing input\n');r=run([rust,'predictor','evaluate','--input',t1,'--mjd','58000.01','--frequency-mhz','1400'],a.out,env,'trailing');assert r['returncode']==0;assert json.loads((a.out/'trailing.stdout').read_text())['input']['predictor']['T1']['trailing_error'];records.append(r)
# Failed output directory publishes no JSON success; a new valid process recovers.
blocked=a.out/'not-a-directory';blocked.write_text('occupied')
base=[rust,'predictor','cheby','--par',ROOT/'validation/predictors/fixtures/isolated.par','--runtime',ROOT/'validation/reference/build/source/T2runtime','--site','@','--start-mjd','58000','--end-mjd','58000.01','--freq-start-mhz','1000','--freq-end-mhz','2000','--time-coeffs','8','--freq-coeffs','4','--segment-seconds','3600','--output-dir']
for name,path,success in [('output-failure',blocked,False),('output-recovery',a.out/'recovered',True)]:
 r=run(base+[path],a.out,env,name);assert (r['returncode']==0)==success
 if not success:assert not (a.out/(name+'.stdout')).read_bytes()
 records.append(r)
(a.out/'receipt.json').write_text(json.dumps({'rust':sha(rust),'commands':records},indent=2)+'\n');print(len(records),'failure/recovery operations checked')
