"""Natural-state standalone sinc/GWB callbacks and the non-default glitch update."""
from pathlib import Path
import argparse, json, os, subprocess
from check_kernels import compare, read_native, sha

ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
BUILD=HERE/'build/standalone-callbacks';AD=ROOT/'validation/timing_models/fixtures'


def run(profile):
    BUILD.mkdir(parents=True,exist_ok=True)
    inputs=[AD/(name+'.par')for name in ['glitches','ifunc-sinc','quad-interpolation','gw-burst']]
    inputs.append(HERE/'inputs/dm-grid.par')
    par=BUILD/'combined.par';par.write_text(inputs[0].read_text()+''.join(p.read_text().split('UNITS TCB\n',1)[1]for p in inputs[1:-1])+'DMMODEL'+inputs[-1].read_text().split('DMMODEL',1)[1])
    tim=AD/'glitches.tim';runtime=ROOT/'validation/reference/build/source/T2runtime'
    identity=json.loads((HERE/'build/native/identity.json').read_text())
    native=BUILD/'native';cmd=identity['commands'][-1].copy();cmd[cmd.index(str(HERE/'native_fit.cc'))]=str(HERE/'standalone_callbacks_probe.cc');cmd[cmd.index('-o')+1]=str(native)
    p=subprocess.run(cmd,capture_output=True);assert p.returncode==0,p.stderr.decode();commands=[cmd]
    directory=BUILD/profile;directory.mkdir(exist_ok=True);trace=directory/'native.txt'
    rust=ROOT/'rust/target'/profile/'examples/fitting_standalone_callbacks'
    for name,cmd in [('native',[str(native),str(trace),str(par),str(tim)]),('rust',[str(rust),str(runtime),str(par),str(tim)])]:
        p=subprocess.run(cmd,cwd=directory,env={**os.environ,'TEMPO2':str(runtime)},capture_output=True,timeout=60);commands.append(cmd)
        (directory/(name+'.stdout')).write_bytes(p.stdout);(directory/(name+'.stderr')).write_bytes(p.stderr)
        assert p.returncode==0,(name,p.returncode,p.stderr.decode())
    reference=read_native(trace.read_text());actual=json.loads((directory/'rust.stdout').read_text())
    failures=compare(reference,actual)
    (directory/'receipt.json').write_text(json.dumps({'profile':profile,'pin':identity['pin'],'base_identity_sha256':sha(HERE/'build/native/identity.json'),'builder_sha256':sha(__file__),'probe_sha256':sha(HERE/'standalone_callbacks_probe.cc'),'source_inputs':{str(p):sha(p)for p in inputs},'inputs':{str(p):sha(p)for p in [par,tim]},'executables':{str(p):sha(p)for p in [native,rust]},'commands':commands,'fields':len(reference),'failures':failures},indent=2)+'\n')
    print('standalone callback fields',len(reference),'failures',failures)


if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--profile',choices=['debug','release'],default='debug');run(p.parse_args().profile)
