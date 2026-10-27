"""Bounded native/current-AJ check of defined low-count C set reads."""
from pathlib import Path
import hashlib,json,os,shutil,subprocess,tempfile

ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust')
PIN='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
REF=ROOT/'validation/reference/build'
PREFIX=ROOT/'validation/library_interfaces/build/debug-prefix'
OUT=Path(tempfile.mkdtemp(prefix='tempo2-aj-low-count-review-'))
Path('/tmp/tempo2-aj-low-count-review-current').write_text(str(OUT)+'\n')
CXX='/opt/homebrew/bin/g++-16'
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
archive=REF/'libtempo2-reference.a'
assert sha(archive)=='313507c5063cb6a8156a0e5fd0f752e2baa59a77c78b5b32822d8efeddd2dbf5'
raw=subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',PIN+':t1polyco.c'])
assert raw==(REF/'source/t1polyco.c').read_bytes()
(OUT/'pinned-t1polyco.c').write_bytes(raw)
lib=OUT/'lib'
lib.mkdir()
shutil.copy2(PREFIX/'lib/libtempo2pred_rust.dylib',lib/'libtempo2pred_rust.dylib')
for name in ['rust/native/predictor.c','rust/src/predictor_ffi.rs','rust/LIBRARY_INTERFACES_API.md']:
    p=OUT/'current-source'/name
    p.parent.mkdir(parents=True,exist_ok=True)
    shutil.copy2(ROOT/name,p)
driver=r'''#include "tempo2pred_int.h"
#include <cstdio>
#include <cstdlib>
#include <string>
extern "C" { void T1PolycoSet_Init(T1PolycoSet *); void T1PolycoSet_Destroy(T1PolycoSet *); }
static std::string record(int count) {
    std::string x="TEMPO2:\nREAD DATE UTC 58000 0 0 -6 7 100 @ 60 "+std::to_string(count)+" 1400 0 0\n";
    if(count==1)x+="0.5\n";
    if(count==2)x+="0.5 60\n";
    return x;
}
static void run(const char *label, const std::string &text) {
    FILE *f=tmpfile(); if(!f)std::abort();
    if(fwrite(text.data(),1,text.size(),f)!=text.size())std::abort();
    rewind(f); T1PolycoSet set; T1PolycoSet_Init(&set);
    int ret=T1PolycoSet_Read(&set,f);
    std::printf("READ %s %d %d %ld %d %d\n",label,ret,set.nsegments,ftell(f),feof(f),ferror(f));
    for(int i=0;i<set.nsegments;i++) {
        auto &p=set.segments[i];
        // Reading a complete header/count is defined even if a later evaluation
        // would need an unwritten C1. Initialize the required exposed backing
        // before calling frequency; no uninitialized scalar is observed here.
        std::printf("META %s %d %d %La %s\n",label,i,p.ncoeff,p.mjd_mid,p.psrname);
        p.coeff[0]=0.25L; p.coeff[1]=30;
        std::printf("FREQUENCY %s %d %La\n",label,i,T1Polyco_GetFrequency(&p,58000,700));
    }
    T1PolycoSet_Destroy(&set); fclose(f);
}
int main() {
    run("count0",record(0));run("count1",record(1));run("count2",record(2));
    run("prefix2_then0",record(2)+record(0));run("prefix2_then1",record(2)+record(1));
}
'''
(OUT/'probe.cc').write_text(driver)
commands=[]
def run(label,cmd):
    cmd=list(map(str,cmd))
    r=subprocess.run(cmd,cwd=OUT,text=True,capture_output=True,timeout=60)
    (OUT/(label+'.stdout')).write_text(r.stdout);(OUT/(label+'.stderr')).write_text(r.stderr)
    commands.append(dict(label=label,command=cmd,exit=r.returncode,stdout_sha256=sha(OUT/(label+'.stdout')),stderr_sha256=sha(OUT/(label+'.stderr'))))
    assert r.returncode==0,(label,r.stderr)
    return r.stdout
run('native-build',[CXX,'-O2','-std=c++17','-I'+str(REF/'source'),OUT/'probe.cc',archive,'-lquadmath','-lgfortran','-o',OUT/'native'])
run('rust-build',[CXX,'-O2','-std=c++17','-I'+str(PREFIX/'include'),OUT/'probe.cc','-L'+str(lib),'-ltempo2pred_rust','-Wl,-rpath,'+str(lib),'-o',OUT/'rust'])
native=run('native-run',[OUT/'native']);rust=run('rust-run',[OUT/'rust'])
run('rust-dependencies',['/usr/bin/otool','-L',OUT/'rust'])
rn=[line.split() for line in native.splitlines() if line.startswith('READ ')]
rr=[line.split() for line in rust.splitlines() if line.startswith('READ ')]
assert len(rn)==len(rr)==5
assert [int(x[2]) for x in rn]==[0]*5
assert [int(x[3]) for x in rn]==[1,1,1,2,2]
differences=[{'case':a[1],'native':a[2:],'rust':b[2:]} for a,b in zip(rn,rr) if a!=b]
result=dict(native_pin=PIN,reference_archive_sha256=sha(archive),copied_rust_dylib_sha256=sha(lib/'libtempo2pred_rust.dylib'),source_snapshot={str(p.relative_to(OUT)):sha(p) for p in (OUT/'current-source').rglob('*') if p.is_file()},commands=commands,differences=differences,scope='Defined C set-reader status/count/cursor and caller initialization of exposed backing before frequency; no uninitialized coefficient is evaluated. Current AJ debug-prefix binary is provisional, not a delivered/accepted source identity.')
(OUT/'receipt.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'directory':str(OUT),'differences':differences,'receipt_sha256':sha(OUT/'receipt.json')},indent=2))
print('NATIVE\n'+native+'RUST\n'+rust)
