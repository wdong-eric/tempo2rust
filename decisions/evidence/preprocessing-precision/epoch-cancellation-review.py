from pathlib import Path
from fractions import Fraction
from decimal import Decimal,localcontext
import argparse,gzip,hashlib,json,subprocess
ap=argparse.ArgumentParser();ap.add_argument('preprocessing_run',type=Path);ap.add_argument('--output',type=Path,required=True);args=ap.parse_args()
root=Path(__file__).resolve().parents[3]
base=args.preprocessing_run.resolve()
out=args.output.resolve();out.mkdir(parents=True,exist_ok=False)
rows=json.loads((base/'128/corpus/differences.json').read_text())
record=next(r for r in rows if r['tag']=='units100 1 stage1' and r['field']=='p0.param.val')
witness=next(r for r in record['numbers'] if r['parameter']=='Some(param_t0)' and r['index']==0)
initial=json.loads(gzip.decompress((root/'validation/preprocessing/initial.json.gz').read_bytes()))['state']
def decode(d):
 b=bytearray(d['size'])
 for offset,h in d['nonzero']:b[offset:offset+len(h)//2]=bytes.fromhex(h)
 return b
sizes=decode(initial['p0.param.aSize']);cell=sum(int.from_bytes(sizes[4*i:4*i+4],'little',signed=True) for i in range(16))
assert decode(initial['p0.param.shortlabel'])[cell*100:cell*100+3]==b'T0\0'
assert decode(initial['p0.param.val'])[cell*16:cell*16+16]==bytes(16)
source=out/'cancellation.rs'
source.write_text('''use rug::Float;
fn main() {
    let args:Vec<_>=std::env::args().collect();
    for bits in [113,128,256,512] {
        // units.C::xform_mjd for the P units100 T0=0 caller. Both constants
        // originate as binary64; retain source grouping through all three steps.
        let f=Float::with_val(bits,1)+1.55051979176e-8_f64;
        let shifted=Float::with_val(bits,0)-43144.0003725_f64;
        let scaled=shifted*f;
        let result=scaled+43144.0003725_f64;
        if bits==113 {assert_eq!(result,Float::with_val(bits,Float::parse(&args[1]).unwrap()));}
        if bits==128 {assert_eq!(result,Float::with_val(bits,Float::parse(&args[2]).unwrap()));}
        let (mantissa,exponent)=result.to_integer_exp().unwrap();
        println!("{}\\t{}\\t{}\\t{}",bits,result.to_string_radix(10,Some(170)),mantissa,exponent);
    }
}
''')
rustc=Path(subprocess.check_output(['rustup','which','rustc'],text=True).strip());deps=root/'rust/target/debug/deps';rugs=list(deps.glob('librug-*.rlib'));assert len(rugs)==1
exe=out/'cancellation';command=[str(rustc),'--edition=2024',str(source),'-L','dependency='+str(deps),'--extern','rug='+str(rugs[0]),'-o',str(exe)]
with (out/'compile.log').open('w') as log:subprocess.run(command,stdout=log,stderr=subprocess.STDOUT,check=True)
run=subprocess.run([str(exe),witness['native'],witness['rust']],text=True,capture_output=True,check=True);(out/'values.tsv').write_text(run.stdout)
exact=-Fraction.from_float(43144.0003725)*Fraction.from_float(1.55051979176e-8)
values={}
for line in run.stdout.splitlines():
 bits,text,mantissa,exponent=line.split('\t');e=int(exponent);v=Fraction(int(mantissa))*Fraction(2)**e;values[int(bits)]=v
assert values[256]==values[512]==exact
assert abs(values[128]-exact)<abs(values[113]-exact)
with localcontext() as context:
 context.prec=100
 def decimal(f):return str(Decimal(f.numerator)/Decimal(f.denominator))
 result={'qualification':'one source-expression diagnosis, no acceptance or production-policy change','case':'units100','field':'param_t0[0].val','initial_value':'0','initial_label_and_value_verified':True,'source':'units.C::xform_mjd; IFTE constants from ifteph.h','native_113_match':True,'production_128_match':True,'256_512_equal_exact_binary64_constant_rational':True,'absolute_error_days':{str(bits):decimal(abs(v-exact)) for bits,v in values.items()},'absolute_error_seconds':{str(bits):decimal(abs(v-exact)*86400) for bits,v in values.items()},'relative_difference_113_128':decimal(abs(values[113]-values[128])/max(abs(values[113]),abs(values[128]))),'review_source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'compiler_command':command,'compiler_version':subprocess.check_output([str(rustc),'-vV'],text=True),'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'executable_sha256':hashlib.sha256(exe.read_bytes()).hexdigest(),'initial_sha256':hashlib.sha256((root/'validation/preprocessing/initial.json.gz').read_bytes()).hexdigest(),'difference_input_sha256':hashlib.sha256((base/'128/corpus/differences.json').read_bytes()).hexdigest()}
(out/'receipt.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({k:result[k] for k in ['native_113_match','production_128_match','256_512_equal_exact_binary64_constant_rational','absolute_error_seconds']}))
print('receipt',out/'receipt.json')
