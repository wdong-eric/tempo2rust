"""Check that foreign scalar marshalling preserves values under caller locale."""
from pathlib import Path
import hashlib,json,subprocess
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust')
OUT=Path(Path('/tmp/tempo2-aj-low-count-review-current').read_text().strip())
REF=ROOT/'validation/reference/build'
CXX='/opt/homebrew/bin/g++-16'
driver=r'''#include "tempo2pred_int.h"
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <clocale>
int main() {
    T1Polyco p;std::memset(&p,0,sizeof p);
    p.ncoeff=2;p.mjd_mid=58000;p.frequency_psr_0=100.125L;p.coeff[1]=30;
    const char *names[]={"C","de_DE.UTF-8","fr_FR.UTF-8","C"};
    for (int i=0;i<4;++i) {
        if (!std::setlocale(LC_NUMERIC,names[i])) return 2;
        long double result=T1Polyco_GetFrequency(&p,58000,700);
        // Print under C locale so the comparison observes the scalar value,
        // not the caller's independently legitimate presentation convention.
        if (!std::setlocale(LC_NUMERIC,"C")) return 3;
        std::printf("FREQUENCY %d %s %La\n",i,names[i],result);
    }
}
'''
(OUT/'locale-probe.cc').write_text(driver)
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
commands=[]
def run(label,args):
    args=list(map(str,args));r=subprocess.run(args,cwd=OUT,capture_output=True,text=True,timeout=60)
    (OUT/(label+'.stdout')).write_text(r.stdout);(OUT/(label+'.stderr')).write_text(r.stderr)
    commands.append(dict(label=label,command=args,exit=r.returncode,stdout_sha256=sha(OUT/(label+'.stdout')),stderr_sha256=sha(OUT/(label+'.stderr'))))
    assert r.returncode==0,(label,r.stderr)
    return r.stdout
common=[CXX,'-O2','-std=c++17','-I'+str(REF/'source'),OUT/'locale-probe.cc']
run('locale-native-build',[*common,REF/'libtempo2-reference.a','-lquadmath','-lgfortran','-o',OUT/'locale-native'])
run('locale-rust-build',[*common,'-L'+str(OUT/'lib'),'-ltempo2pred_rust','-Wl,-rpath,'+str(OUT/'lib'),'-o',OUT/'locale-rust'])
native=run('locale-native-run',[OUT/'locale-native']);rust=run('locale-rust-run',[OUT/'locale-rust'])
values=[line.split()[-1] for line in native.splitlines()]
assert len(values)==4 and len(set(values))==1
result=dict(commands=commands,reference_archive_sha256=sha(REF/'libtempo2-reference.a'),rust_dylib_sha256=sha(OUT/'lib/libtempo2pred_rust.dylib'),native=native,rust=rust,matching=native==rust,scope='Defined T1 frequency on identical fully initialized native scalar backing. Only caller LC_NUMERIC changes; output formatting resets to C to expose actual value differences. No file parsing, phase, new rounding policy or uninitialized state is involved.')
(OUT/'locale-receipt.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'directory':str(OUT),'receipt_sha256':sha(OUT/'locale-receipt.json'),'native':native,'rust':rust,'matching':native==rust},indent=2))
