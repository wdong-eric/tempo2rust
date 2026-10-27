"""Bounded native calculate_bclt worker witness for AK preparation; no Rust edits."""
from pathlib import Path
import difflib
import hashlib
import json
import os
import subprocess
import tempfile

ROOT = Path('/Users/Eric.Dong/Downloads/tempo2rust')
PIN = 'fda6ad4a9753ad7dc5d291406f2f310a9161423f'
REV = '20e368fe6e7d3e05565ae7f0132f4498dfa9069e'
REF = ROOT / 'validation/reference/build'
CXX = Path('/opt/homebrew/bin/g++-16')
OUT = Path(tempfile.mkdtemp(prefix='tempo2-central-openmp-'))
Path('/tmp/tempo2-central-openmp-current').write_text(str(OUT)+'\n')
records = []

def sha(p):
    return hashlib.sha256(Path(p).read_bytes()).hexdigest()

def run(label, command, env=None):
    command=list(map(str,command))
    result=subprocess.run(command,cwd=OUT,env=env,text=True,capture_output=True,timeout=120)
    stdout=OUT/(label+'.stdout')
    stderr=OUT/(label+'.stderr')
    stdout.write_text(result.stdout)
    stderr.write_text(result.stderr)
    records.append(dict(label=label,command=command,exit=result.returncode,
                        stdout_sha256=sha(stdout),stderr_sha256=sha(stderr)))
    (OUT/'commands.json').write_text(json.dumps(records,indent=2)+'\n')
    assert result.returncode==0,(label,result.returncode,result.stderr[-3000:])
    return result.stdout

archive=REF/'libtempo2-reference.a'
assert sha(archive)=='313507c5063cb6a8156a0e5fd0f752e2baa59a77c78b5b32822d8efeddd2dbf5'
assert subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'rev-parse','HEAD'],text=True).strip()==PIN
raw=subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',PIN+':calculate_bclt.C'],text=True)
assert hashlib.sha256(raw.encode()).hexdigest()=='899c58042bc053efeae5b25eacc46de9b4ca1bb0943c995a846e0affb8d597e7'
(OUT/'original-calculate_bclt.C').write_text(raw)
declaration='''extern void central_observation_event(int, int);
int central_openmp_compilation() {
#ifdef _OPENMP
    return _OPENMP;
#else
    return 0;
#endif
}
'''
text=raw.replace('void calculate_bclt(pulsar *psr,int npsr)',declaration+'\nvoid calculate_bclt(pulsar *psr,int npsr)',1)
anchor='        for (int i=0;i<psr[p].nobs;i++){'
assert text.count(anchor)==1
text=text.replace(anchor,anchor+'\n            central_observation_event(p, i);',1)
(OUT/'calculate_bclt.C').write_text(text)
(OUT/'patch.diff').write_text(''.join(difflib.unified_diff(raw.splitlines(True),text.splitlines(True),fromfile='pinned/calculate_bclt.C',tofile='instrumented/calculate_bclt.C')))

include=OUT/'include'
include.mkdir()
headers={}
for source in sorted((REF/'source').glob('*.h')):
    (include/source.name).write_bytes(source.read_bytes())
    headers[source.name]=sha(source)

probe=r'''#include "tempo2.h"
#include <omp.h>
#include <quadmath.h>
#include <cstdio>
#include <cstdlib>
#include <vector>
#include <set>

static std::vector<int> workers, visits;
extern int central_openmp_compilation();
void central_observation_event(int p, int i) {
    if (p != 0 || i < 0 || i >= static_cast<int>(workers.size())) std::abort();
    // The source worksharing loop assigns each observation exactly once.
    // Each element belongs to that iteration, so this observer adds no shared counter.
    workers[i] = omp_get_thread_num();
    visits[i] += 1;
}
int main() {
    constexpr int n=259;
    auto *p=static_cast<pulsar *>(std::calloc(1,sizeof(pulsar)));
    if (!p) return 2;
    p->nobs=n;
    p->correctTroposphere=0;
    p->obsn=static_cast<observation *>(std::calloc(n,sizeof(observation)));
    p->param[param_posepoch].val=static_cast<longdouble *>(std::calloc(1,sizeof(longdouble)));
    if (!p->obsn || !p->param[param_posepoch].val) return 3;
    p->param[param_posepoch].val[0]=58000;
    p->posPulsar[0]=0.8; p->posPulsar[1]=-0.4; p->posPulsar[2]=0.3;
    p->velPulsar[0]=1e-5; p->velPulsar[1]=-2e-5; p->velPulsar[2]=3e-5;
    debugFlag=0; quietFlag=1; displayCVSversion=0; tcheck=0;
    workers.assign(n,-1); visits.assign(n,0);
    for (int i=0;i<n;++i) {
        p->obsn[i].delayCorr=0;
        p->obsn[i].sat=static_cast<longdouble>(58000)+static_cast<longdouble>(i)/86400;
        p->obsn[i].freq=700+13*i;
        p->obsn[i].troposphericDelay=-777;
    }
    calculate_bclt(p,1);
    std::set<int> unique;
    for (int i=0;i<n;++i) {
        if (visits[i]!=1 || workers[i]<0) return 4;
        unique.insert(workers[i]);
        auto &o=p->obsn[i];
        if (o.freqSSB!=o.freq || o.roemer!=0 || o.troposphericDelay!=0) return 5;
        char roemer[128];
        quadmath_snprintf(roemer,sizeof(roemer),"%Qa",o.roemer);
        std::printf("value %d %a %s %a %a %a %a\n",i,o.freqSSB,roemer,o.psrPos[0],o.psrPos[1],o.psrPos[2],o.troposphericDelay);
        std::fprintf(stderr,"worker %d %d %d\n",i,workers[i],visits[i]);
    }
    std::fprintf(stderr,"summary %d %zu %d\n",central_openmp_compilation(),unique.size(),n);
    std::free(p->param[param_posepoch].val);
    std::free(p->obsn);
    std::free(p);
}
'''
(OUT/'probe.cc').write_text(probe)
flags=['-O2','-std=gnu++17','-DHAVE_CONFIG_H','-I'+str(include)]
run('probe-compile',[CXX,*flags,'-fopenmp','-c',OUT/'probe.cc','-o',OUT/'probe.o'])
for label,extra in [('serial',[]),('parallel',['-DHAVE_OPENMP','-fopenmp'])]:
    run(label+'-compile',[CXX,*flags,*extra,'-c',OUT/'calculate_bclt.C','-o',OUT/(label+'.o')])
    run(label+'-link',[CXX,'-fopenmp',OUT/'probe.o',OUT/(label+'.o'),archive,'-lquadmath','-lgfortran','-Wl,-map,'+str(OUT/(label+'.map')),'-o',OUT/label])
    run(label+'-dependencies',['/usr/bin/otool','-L',OUT/label])

profiles={}
for name,binary,count in [('serial','serial',4),('parallel-1','parallel',1),('parallel-2','parallel',2),('parallel-4','parallel',4)]:
    env=dict(os.environ,OMP_NUM_THREADS=str(count),OMP_DYNAMIC='FALSE',OMP_THREAD_LIMIT='4',OMP_MAX_ACTIVE_LEVELS='1')
    values=run(name+'-run',[OUT/binary],env)
    lines=(OUT/(name+'-run.stderr')).read_text().splitlines()
    summary=lines[-1].split()
    assert summary[0]=='summary' and int(summary[3])==259
    observed=sorted({int(line.split()[2]) for line in lines[:-1]})
    assert len(lines)==260 and len(observed)==(1 if binary=='serial' else count)
    assert (int(summary[1])==0) if binary=='serial' else (int(summary[1])>0)
    assert len(values.splitlines())==259
    profiles[name]=dict(requested_workers=count,observed_workers=observed,openmp_macro=int(summary[1]),observations=259,scientific_sha256=sha(OUT/(name+'-run.stdout')),executable_sha256=sha(OUT/binary))
    if name=='serial': baseline=values
    else: assert values==baseline,name

snapshot=OUT/'rust-snapshot'
sources={}
for name in ['clkcorr.rs','clock_diagnostics.rs','source_state.rs','propagation.rs','solar_wind.rs','global.rs','bats.rs','runtime.rs','calceph.rs','t2accel.rs','general_fitting.rs','fitting_tests.rs']:
    path='rust/src/'+name
    data=subprocess.check_output(['git','show',REV+':'+path],cwd=ROOT)
    snapshot.mkdir(exist_ok=True)
    (snapshot/name).write_bytes(data)
    sources[path]=sha(snapshot/name)
result=dict(native_pin=PIN,rust_snapshot_revision=REV,source_sha256=sha(OUT/'original-calculate_bclt.C'),instrumented_source_sha256=sha(OUT/'calculate_bclt.C'),patch_sha256=sha(OUT/'patch.diff'),archive_sha256=sha(archive),headers=headers,rust_sources=sources,compiler=run('compiler-version',[CXX,'--version']),profiles=profiles,commands=records,comparisons=259*6*3,failures=[],scope='Actual pinned calculate_bclt loop with an observation-ID worker observer; one pulsar, 259 defined delayCorr=0 observations, diagnostics/version disabled. Exact serial/parallel equality in six observed quantities. This is not full timing, Rust parallelism, shared-state/race, OpenMP Makefile-wiring or performance qualification.')
(OUT/'receipt.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'directory':str(OUT),'comparisons':result['comparisons'],'profiles':profiles,'receipt_sha256':sha(OUT/'receipt.json'),'scope':result['scope']},indent=2))
