"""Read-only phase/BAT stage localization for the frozen AF precision experiment."""
from pathlib import Path
from decimal import Decimal as D, getcontext
import argparse, difflib, hashlib, json, os, subprocess, sys
getcontext().prec = 70
sys.dont_write_bytecode = True

def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('precision_run', type=Path)
    parser.add_argument('--root', type=Path, required=True)
    a = parser.parse_args()
    base, root = a.precision_run.resolve(), a.root.resolve()
    out = base / 'stage-localization'
    out.mkdir(exist_ok=False)
    receipt = json.loads((base / 'receipt.json').read_text())
    snapshot = base / 'snapshot'
    sys.path.insert(0, str(snapshot / 'validation/predictors'))
    from compare import read_tsv
    ni_path = root / 'validation/predictors/build/native-corrected/identity.json'
    ni = json.loads(ni_path.read_text())
    protected = dict(ni['objects'])
    protected[str(root / 'validation/reference/build/libtempo2-reference.a')] = ni['archive']
    for path, digest in protected.items():
        assert sha(Path(path)) == digest
    extended = ['roemer', 'torb', 'correctionTT_TB', 'correctionTT_Teph', 'shklovskii']
    doubles = ['tdis1', 'tdis2', 'shapiroDelaySun', 'troposphericDelay']
    vectors = ['earth_ssb', 'observatory_earth']
    original = (snapshot / 'validation/predictors/native.cc').read_text()
    marker = 'value("freq",i,ps->obsn[i].freq);'
    assert original.count(marker) == 1
    # Only dump terrestrial fields on non-reference rows; barycentric processing
    # bypasses their producers. torb is set by formResiduals in both branches.
    additions = 'if(i>0){timing_value("audit.torb",i,ps->obsn[i].torb);if(ps->obsn[i].delayCorr){'
    additions += ''.join(f'timing_value("audit.{f}",i,ps->obsn[i].{f});' for f in extended if f != 'torb')
    additions += ''.join(f'value("audit.{f}",i,ps->obsn[i].{f});' for f in doubles)
    additions += ''.join(f'value("audit.{f}.{k}",i,ps->obsn[i].{f}[{k}]);' for f in vectors for k in range(3))
    additions += ''.join(f'value("audit.posPulsar.{k}",i,ps->posPulsar[{k}]);' for k in range(3)) + '}}'
    source = out / 'stage-native.cc'
    source.write_text(original.replace(marker, marker + additions))
    (out / 'native-observer.patch').write_text(''.join(difflib.unified_diff(original.splitlines(True), source.read_text().splitlines(True), fromfile='frozen/native.cc', tofile='diagnostic/stage-native.cc')))
    commands = []
    env = {**os.environ, **receipt['environment']}
    def run(cmd, cwd, label):
        cmd = list(map(str, cmd))
        with (cwd / (label + '.stdout')).open('w') as stdout, (cwd / (label + '.stderr')).open('w') as stderr:
            result = subprocess.run(cmd, cwd=cwd, env=env, stdout=stdout, stderr=stderr)
        commands.append({'command': cmd, 'cwd': str(cwd), 'returncode': result.returncode})
        (out / 'commands.json').write_text(json.dumps(commands, indent=2) + '\n')
        assert result.returncode == 0, commands[-1]
    cmd = ni['commands'][-1].copy()
    cmd[cmd.index(str(root / 'validation/predictors/native.cc'))] = str(source)
    cmd[-1] = str(out / 'native')
    run(cmd, out, 'compile-native')
    rust_source = '''use std::path::PathBuf;
use tempo2_rust::{RuntimeContext, Scalar, predictor, polyco::{self, PolycoRequest}};
fn main() -> Result<(), Box<dyn std::error::Error>> {
 let a: Vec<_> = std::env::args().skip(1).collect();
 assert_eq!(a.len(),4);
 let root=PathBuf::from(&a[1]);let out=PathBuf::from(&a[3]);std::fs::create_dir(&out)?;
 let mut input=predictor::load_par(&PathBuf::from(&a[0]),&root,&[])?;
 let mut runtime=RuntimeContext::from_root(&root)?;
 predictor::cli_defaults(&mut input.context,&a[2])?;
 let q=|s:&str|s.parse::<Scalar>().map(|x|x.as_float().clone());
 let request=PolycoRequest{start_mjd:q("58000")?,end_mjd:q("58000")?,span_minutes:60,coefficients:12,max_hour_angle:q("1")?,site:a[2].clone(),frequency_mhz:q("1400")?,true_dm:true,file_prefix:out.join("diagnostic-")};
 let result=polyco::polyco(&mut input.context,&mut runtime,&request)?;
 for (i,m) in result.models.segments.iter().enumerate(){println!("log10rms\\t{}\\t{}",i,m.log10rms);}
 let (p,rows)=input.context.pulsar_observations_mut(0)?;
 let mut text=String::new();
 for (i,o) in rows.iter().take(p.nobs.unwrap() as usize).enumerate(){
  for (name,v) in [("sat",&o.sat),("bat",&o.bat),("phase",&o.phase)]{text+=&format!("{}\\t{}\\t{}\\n",name,i,v.as_ref().unwrap());}
  if i==0{continue;}
  text+=&format!("audit.torb\\t{}\\t{}\\n",i,o.torb.as_ref().unwrap());
  if o.delayCorr==Some(0){continue;}
  EXTENDED
  DOUBLES
  VECTORS
 }
 print!("{}",text);Ok(())
}
'''
    rust_source = rust_source.replace('EXTENDED', '\n'.join(f'text+=&format!("audit.{f}\\t{{}}\\t{{}}\\n",i,o.{f}.as_ref().unwrap());' for f in extended if f != 'torb'))
    rust_source = rust_source.replace('DOUBLES', '\n'.join(f'text+=&format!("audit.{f}\\t{{}}\\t{{}}\\n",i,rug::Float::with_val(113,o.{f}.unwrap()));' for f in doubles))
    rust_source = rust_source.replace('VECTORS', '\n'.join(f'text+=&format!("audit.{f}.{k}\\t{{}}\\t{{}}\\n",i,rug::Float::with_val(113,o.{f}[{k}].unwrap()));' for f in vectors for k in range(3)) + '\n' + '\n'.join(f'text+=&format!("audit.posPulsar.{k}\\t{{}}\\t{{}}\\n",i,rug::Float::with_val(113,p.posPulsar[{k}].unwrap()));' for k in range(3)))
    rs = out / 'stage-rust.rs'
    rs.write_text(rust_source)
    rustc = receipt['commands'][0]['command'][0]
    deps = Path(next(k for k in receipt['dependency_archives'] if '/deps/' in k)).parent
    sofa = Path(next(k for k in receipt['dependency_archives'] if k.endswith('libtiming_native.a')))
    externs = []
    for name in ['rug','serde','serde_json','sha2']:
        path = next(k for k in receipt['dependency_archives'] if Path(k).name.startswith('lib' + name + '-'))
        externs += ['--extern', name + '=' + path]
    cases = [('binary-bary','binary','@'),('isolated-pks','isolated','PKS'),('binary-pks','binary','PKS'),('ecliptic-tdb','ecliptic_tdb','PKS')]
    results = []
    for bits in [128,113]:
        binary = out / ('rust-' + str(bits))
        run([rustc, '--edition=2024', '-C','opt-level=3', rs, '-L','dependency='+str(deps), '-L','native='+str(sofa.parent), *externs, '--extern','tempo2_rust='+str(base/str(bits)/'libtempo2_rust.rlib'), '-o',binary],out,'compile-rust-'+str(bits))
        for name,model,site in cases:
            case = out / str(bits) / name
            nr, rr = case/'native', case/'rust'
            nr.mkdir(parents=True);rr.mkdir()
            par = snapshot/'validation/predictors/fixtures'/(model+'.par')
            run([out/'native','polyco',nr/'result.tsv',par,site,'58000','58000','60','12','1','1400','1',''],nr,'generate')
            run([binary,par,env['TEMPO2'],site,rr/'products'],rr,'generate')
            nv,rv = read_tsv(nr/'result.tsv'),read_tsv(rr/'generate.stdout')
            old_n = read_tsv(base/str(bits)/name/'native/result.tsv')
            assert all(nv[k]==v for k,v in old_n.items()), ('native observer affected existing results',bits,name)
            old_r = json.loads((base/str(bits)/name/'rust/generate.stdout').read_text())
            for i,o in enumerate(old_r['observations']):
                for k,f in [('sat','sat_mjd'),('bat','bat_mjd'),('phase','phase_turn')]:
                    assert rv[k,i]==D(o[f]), ('Rust public producer changed',bits,name,k,i)
            for i,m in enumerate(old_r['result']['models']['segments']):
                assert float(rv['log10rms',i])==m['log10rms']
            differences = []
            for (key,i),value in rv.items():
                if key.startswith('audit.'):
                    # Extended decimal transports need not print identical tails.
                    differences.append({'field':key,'index':i,'rust':str(value),'native':str(nv[key,i]),'difference':str(value-nv[key,i]),'same_binary64_cell':float(value)==float(nv[key,i])})
            f0 = old_n['frequency_psr_0',0]
            rows = [{'index':i,'phase_difference_turn':str(rv['phase',i]-nv['phase',i]),'BAT_difference_s':str((rv['bat',i]-nv['bat',i])*86400),'phase_minus_BAT_turn':str(rv['phase',i]-nv['phase',i]-(rv['bat',i]-nv['bat',i])*86400*f0)} for i in range(len(old_r['observations']))]
            row = {'bits':bits,'name':name,'differences':differences,'phase_decomposition':rows}
            results.append(row)
            print(bits,name,[(f,sum(x['field']==f and not x['same_binary64_cell'] for x in differences)) for f in sorted({x['field'] for x in differences})],flush=True)
    for path,digest in protected.items():assert sha(Path(path))==digest,path
    result={'scope':'Observation-only stage localization; no production change or numerical acceptance.', 'precision_receipt_sha256':sha(base/'receipt.json'),'review_source_sha256':sha(Path(__file__)),'native_identity_sha256':sha(ni_path),'native_observer_sha256':sha(source),'rust_observer_sha256':sha(rs),'native_executable_sha256':sha(out/'native'),'rust_executable_sha256':{str(bits):sha(out/('rust-'+str(bits))) for bits in [128,113]},'protected_files':protected,'commands':commands,'results':results}
    (out/'receipt.json').write_text(json.dumps(result,indent=2)+'\n')
    print('RECEIPT',out/'receipt.json',sha(out/'receipt.json'),flush=True)

if __name__=='__main__':main()
