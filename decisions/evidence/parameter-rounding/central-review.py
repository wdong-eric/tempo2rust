"""Bounded central diagnostic for decision 0073; never changes production files."""
from pathlib import Path
from fractions import Fraction
from decimal import Decimal, localcontext
import hashlib, json, subprocess, sys, tempfile

ROOT = Path('/Users/Eric.Dong/Downloads/tempo2rust')
OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(tempfile.mkdtemp(prefix='tempo2-parameter-rounding-'))
OUT.mkdir(parents=True, exist_ok=True)
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
reference = ROOT / 'validation/parameters/observations.json'
ref_hash = sha(reference)
records = json.loads(reference.read_text())
kin = next(c for c in records if c['id'] == 'link_KIN_after')
kin_value = next(e for e in kin['events'] if e['phase'] == 'end' and e['op'] == 'get')['result']['value']
scaling = next(c for c in records if c['id'] == 'threshold_scaling')
scale_value = next(e for e in scaling['events'] if e['phase'] == 'end' and e['op'] == 'stream')['result'][0]['changes']['0.param_pbdot.0.val']['after']['value']
rust = r'''
use rug::Float;
fn emit(name:&str,p:u32,v:Float) {
    let (n,e)=v.to_integer_exp().unwrap();
    println!("{name}\t{p}\t{n}\t{e}");
}
fn main() {
    let args:Vec<String>=std::env::args().collect();
    emit("native_scale",113,Float::with_val(113,Float::parse(&args[1]).unwrap()));
    emit("native_kin",113,Float::with_val(113,Float::parse(&args[2]).unwrap()));
    for p in [113,128,256,512] {
        let x=Float::with_val(p,Float::parse("1e-7").unwrap());
        assert!(x.clone().abs()>1e-7f64,"source scaling branch changed");
        emit("scale",p,x*1e-12f64);
        let s=Float::with_val(p,0.5);
        let q=s.clone()*2f64/(Float::with_val(p,1)+s.square());
        emit("kin",p,q.asin()*180f64/std::f64::consts::PI);
    }
}
'''
source=OUT/'probe.rs'; source.write_text(rust)
deps=ROOT/'rust/target/debug/deps'
rlibs=list(deps.glob('librug-*.rlib')); assert len(rlibs)==1
rustc=Path(subprocess.check_output(['rustup','which','rustc'],text=True).strip())
exe=OUT/'probe'
command=[str(rustc),'--crate-name','parameter_rounding_probe','--edition=2024',str(source),'--extern','rug='+str(rlibs[0]),'-L','dependency='+str(deps),'-o',str(exe)]
build=subprocess.run(command,text=True,capture_output=True)
(OUT/'compile.log').write_text(build.stdout+build.stderr)
assert build.returncode==0,build.stderr
run=subprocess.run([str(exe),scale_value,kin_value],text=True,capture_output=True)
(OUT/'values.tsv').write_text(run.stdout); (OUT/'stderr.txt').write_text(run.stderr)
assert run.returncode==0,run.stderr
values={}
for line in run.stdout.splitlines():
    name,p,n,e=line.split('\t'); values[(name,int(p))]=Fraction(int(n))*Fraction(2)**int(e)
def decimal(v):
    with localcontext() as ctx:
        ctx.prec=22
        return str(Decimal(v.numerator)/Decimal(v.denominator))
results=[]
for name,multiplier in [('scale',4),('kin',8)]:
    native=values[('native_'+name,113)]
    assert values[(name,113)]==native,'113-bit source replay differs from native'
    high=values[(name,512)]
    errors=[abs(values[(name,p)]-high) for p in [113,128,256]]
    assert errors[0]>errors[1]>errors[2]>0,'precision convergence failed'
    actual=values[(name,128)]
    bound=multiplier*Fraction(1,2**113)*max(abs(native),abs(actual))
    difference=abs(actual-native)
    assert difference>0 and difference<=bound
    assert abs(actual*(1+Fraction(1,10**30))-native)>bound,'material perturbation accepted'
    results.append({'quantity':name,'difference':decimal(difference),'relative_difference':decimal(difference/abs(native)),
        'comparison_bound':decimal(bound),'native113_replay_exact':True,
        'absolute_errors_against512':{str(p):decimal(e) for p,e in zip([113,128,256],errors)},
        'source_branch_retained':True,'relative_1e-30_mutation_rejected':True})
assert sha(reference)==ref_hash
identities={'reference_sha256':ref_hash,'diagnostic_source_sha256':sha(source),'diagnostic_executable_sha256':sha(exe),
    'runner_sha256':sha(Path(__file__)),'rustc':str(rustc),'compile_command':command,
    'dependencies':{p.name:sha(p) for prefix in ['rug','gmp_mpfr_sys','az','libm','libc'] for p in deps.glob('lib'+prefix+'-*.rlib')},
    'pinned_source':{name:hashlib.sha256(subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show','fda6ad4a9753ad7dc5d291406f2f310a9161423f:'+name])).hexdigest() for name in ['readParfile.C','tempo2Util.C']}}
receipt={'kind':'source-expression precision diagnosis, not integrated production acceptance','identities':identities,'results':results}
(OUT/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps({'out':str(OUT),'receipt_sha256':sha(OUT/'receipt.json'),'results':results},indent=2))
