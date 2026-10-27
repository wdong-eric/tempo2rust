"""Check generic predictor FRead on a defined seekable and pipe Cheby stream."""
from pathlib import Path
import hashlib,json,subprocess
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust')
OUT=Path(Path('/tmp/tempo2-aj-low-count-review-current').read_text().strip())
REF=ROOT/'validation/reference/build'
CXX='/opt/homebrew/bin/g++-16'
driver=r'''#include "tempo2pred.h"
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <unistd.h>
static const char text[]=
"ChebyModelSet 1 segments\nChebyModel BEGIN\nPSRNAME PIPE\nSITENAME PKS\nTIME_RANGE 58000 58001\nFREQ_RANGE 1000 2000\nDISPERSION_CONSTANT 0\nNCOEFF_TIME 2\nNCOEFF_FREQ 2\nCOEFFS 2 0\nCOEFFS 4 0\nChebyModel END\nTAIL\n";
static void run(bool nonseekable) {
    FILE *f;
    if(nonseekable) {
        int fd[2];if(pipe(fd))std::abort();
        if(write(fd[1],text,sizeof(text)-1)!=(ssize_t)(sizeof(text)-1))std::abort();
        close(fd[1]); f=fdopen(fd[0],"r");
    }else{
        f=tmpfile();if(!f)std::abort();
        if(fwrite(text,1,sizeof(text)-1,f)!=sizeof(text)-1)std::abort();rewind(f);
    }
    if(!f)std::abort();
    T2Predictor p;T2Predictor_Init(&p);
    int status=T2Predictor_FRead(&p,f);
    std::printf("READ %s %d %d %d %s\n",nonseekable?"pipe":"seekable",status,(int)p.kind,
        status==0?p.modelset.cheby.nsegments:-1,status==0?T2Predictor_GetPSRName(&p):"none");
    int next=fgetc(f);
    std::printf("CURSOR %s %d %d\n",nonseekable?"pipe":"seekable",next,ferror(f));
    T2Predictor_Destroy(&p);fclose(f);
}
int main(){run(false);run(true);}
'''
(OUT/'pipe-probe.cc').write_text(driver)
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
commands=[]
def run(label,args):
    args=list(map(str,args));r=subprocess.run(args,cwd=OUT,capture_output=True,text=True,timeout=60)
    (OUT/(label+'.stdout')).write_text(r.stdout);(OUT/(label+'.stderr')).write_text(r.stderr)
    commands.append(dict(label=label,command=args,exit=r.returncode,stdout_sha256=sha(OUT/(label+'.stdout')),stderr_sha256=sha(OUT/(label+'.stderr'))))
    assert r.returncode==0,(label,r.stderr)
    return r.stdout
common=[CXX,'-O2','-std=c++17','-I'+str(REF/'source'),OUT/'pipe-probe.cc']
run('pipe-native-build',[*common,REF/'libtempo2-reference.a','-lquadmath','-lgfortran','-o',OUT/'pipe-native'])
run('pipe-rust-build',[*common,'-L'+str(OUT/'lib'),'-ltempo2pred_rust','-Wl,-rpath,'+str(OUT/'lib'),'-o',OUT/'pipe-rust'])
native=run('pipe-native-run',[OUT/'pipe-native']);rust=run('pipe-rust-run',[OUT/'pipe-rust'])
assert 'READ seekable 0 1 1 PIPE' in native and 'READ pipe 0 1 1 PIPE' in native
result=dict(commands=commands,reference_archive_sha256=sha(REF/'libtempo2-reference.a'),rust_dylib_sha256=sha(OUT/'lib/libtempo2pred_rust.dylib'),native=native,rust=rust,matching=native==rust,scope='Borrowed generic FILE reader with complete defined Cheby content. Source uses ftell but needs rewind only on failed first trial. Pipe success and next unread byte are observed; no source failure-state or uninitialized storage is used.')
(OUT/'pipe-receipt.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'directory':str(OUT),'receipt_sha256':sha(OUT/'pipe-receipt.json'),**result},indent=2))
