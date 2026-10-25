"""Native interactive/automatic T2fitSpectra choices and repeated-call histories."""
from pathlib import Path
from decimal import Decimal as D, getcontext
import argparse, hashlib, json, subprocess
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
BUILD=HERE/'build/spectral-session';REF=ROOT/'validation/reference/build'
PIN='fda6ad4a9753ad7dc5d291406f2f310a9161423f';sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
getcontext().prec=70


def run(profile):
    BUILD.mkdir(parents=True,exist_ok=True)
    source=subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',PIN+':choleskyRoutines.C']).decode()
    # This diagnostic reads uninitialized v1/m when amplitude is supplied. Remove
    # only that unused diagnostic from the observer; never use its bytes as truth.
    token='    printf("Model scale = %g %g %g %d\\n",*modelScale,v1,m,*modelNfit);'
    assert source.count(token)==1
    source=source.replace(token,'    // AE observer omits undefined diagnostic-only v1/m reads.')
    src=BUILD/'choleskyRoutines.C';src.write_text(source);obj=BUILD/'choleskyRoutines.o'
    base=['/opt/homebrew/bin/g++-16','-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(REF/'source')]
    commands=[base+['-c',str(src),'-o',str(obj)],base+[str(HERE/'spectrum_probe.cc'),str(obj),str(REF/'libtempo2-reference.a'),'-lquadmath','-lgfortran','-o',str(BUILD/'native')]]
    for cmd in commands:subprocess.run(cmd,capture_output=True,check=True)
    rust=ROOT/'rust/target'/profile/'examples/fitting_spectrum';records=[]
    for scenario in range(5):
        answers=b'0\n1\n0.5\n0.25\n4\n5\n' if scenario==0 else b''
        a=subprocess.run([str(BUILD/'native'),str(scenario)],input=answers,capture_output=True,timeout=30);assert a.returncode==0,a.stderr.decode()
        b=subprocess.check_output([str(rust),str(scenario)],timeout=30);got=json.loads(b)
        (BUILD/f'{scenario}-native.txt').write_bytes(a.stdout+a.stderr);(BUILD/f'{scenario}-{profile}.json').write_bytes(b)
        rows=[r.split() for r in a.stdout.decode().splitlines() if r.startswith('STATE ')]
        assert len(rows)==len(got);failures=[]
        for row,actual in zip(rows,got):
            assert int(row[1])==actual['call'] and bool(int(row[2]))==actual['result'] and int(row[5])==actual['nfit']
            for name,ref,val in zip(['alpha','corner','scale','variance','beta'],row[3:5]+row[6:9],actual['values']):
                gate=D('5e-14')*abs(D(ref))
                if abs(D(val)-D(ref))>gate:failures.append({'call':actual['call'],'quantity':name,'reference':ref,'rust':val,'gate':str(gate)})
        records.append({'scenario':scenario,'stdin':answers.decode(),'native_sha256':sha(BUILD/f'{scenario}-native.txt'),'rust_sha256':sha(BUILD/f'{scenario}-{profile}.json'),'failures':failures})
    (BUILD/(profile+'.json')).write_text(json.dumps({'pin':PIN,'profile':profile,'commands':commands,'builder_sha256':sha(__file__),'probe_sha256':sha(HERE/'spectrum_probe.cc'),'observed_source_sha256':sha(src),'native_executable_sha256':sha(BUILD/'native'),'rust_executable_sha256':sha(rust),'records':records},indent=2)+'\n')
    print('spectral session failures',sum(len(r['failures'])for r in records))
    for r in records:
        if r['failures']:print(r['scenario'],r['failures'])


if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--profile',choices=['debug','release'],default='debug');run(p.parse_args().profile)
