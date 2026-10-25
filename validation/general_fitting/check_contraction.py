"""GCC16 arm64 TN contraction witness; the production numerical policy is unchanged."""
from pathlib import Path
from decimal import Decimal
import argparse, hashlib, json, os, re, subprocess
from compare import compare
from fitting_corrections import corrected

ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
BUILD=HERE/'build/contraction';REF=ROOT/'validation/reference/build'
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()


def run(profile):
    BUILD.mkdir(parents=True,exist_ok=True)
    identity=json.loads((HERE/'build/native/identity.json').read_text())
    source=corrected('t2fit_nestlike.C',subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',identity['pin']+':t2fit_nestlike.C']).decode())
    commands=[];versions={};records=[]
    for name,flags in [('fused',[]),('unfused',['-ffp-contract=off'])]:
        directory=BUILD/name;directory.mkdir(exist_ok=True)
        path=directory/'t2fit_nestlike.C';path.write_text(source);obj=directory/'t2fit_nestlike.C.o'
        cmd=identity['commands'][0].copy();cmd[cmd.index('-c')+1]=str(path);cmd[cmd.index('-o')+1]=str(obj);cmd+=flags
        p=subprocess.run(cmd,capture_output=True);assert p.returncode==0,p.stderr.decode();commands.append(cmd)
        asm=directory/'assembly.txt';asm.write_bytes(subprocess.check_output(['xcrun','llvm-objdump','--disassemble','--demangle',str(obj)]))
        native=directory/'native';cmd=identity['commands'][-1].copy()
        idx=next(i for i,arg in enumerate(cmd)if Path(arg).name==obj.name);cmd[idx]=str(obj);cmd[cmd.index('-o')+1]=str(native)
        p=subprocess.run(cmd,capture_output=True);assert p.returncode==0,p.stderr.decode();commands.append(cmd)
        versions[name]={'executable':str(native),'executable_sha256':sha(native),'object_sha256':sha(obj),'assembly_sha256':sha(asm),'contractions':len(re.findall(r'\b(?:fmadd|fmsub|fnmadd|fnmsub)\b',asm.read_text()))}
    assert versions['fused']['contractions']>0 and versions['unfused']['contractions']==0
    runtime=REF/'source/T2runtime';rust=ROOT/'rust/target'/profile/'tempo2-rust'
    for name in ['red','dm-noise','chromatic-noise']:
        par=HERE/'inputs'/('red-noise.par'if name=='red'else name+'.par');tim=ROOT/'validation/fitting/inputs/bary.tim'
        directory=BUILD/profile/name;directory.mkdir(parents=True,exist_ok=True)
        command=[str(rust),'timing','--fit','--runtime',str(runtime),'--par',str(par),'--tim',str(tim)]
        p=subprocess.run(command,cwd=directory,capture_output=True,timeout=60);assert p.returncode==0,p.stderr.decode();commands.append(command)
        candidate=directory/'rust.json';candidate.write_bytes(p.stdout)
        traces={};comparisons={}
        for version,details in versions.items():
            trace=directory/(version+'.tsv');command=[details['executable'],str(trace),'1',str(par),str(tim)]
            p=subprocess.run(command,cwd=directory,env={**os.environ,'TEMPO2':str(runtime)},capture_output=True,timeout=60);assert p.returncode==0,p.stderr.decode();commands.append(command)
            (directory/(version+'.stderr')).write_bytes(p.stderr)
            traces[version]={tuple(line.split('\t')[:-1]):Decimal(line.split('\t')[-1])for line in trace.read_text().splitlines()}
            comparisons[version]=compare(trace,candidate,directory/(version+'.comparison.json'))
        a,b=traces['fused'],traces['unfused'];assert a.keys()==b.keys()
        changed=[k for k in a if k[0]=='1' and k[1]=='prepared' and k[4].startswith('matrix.') and float(a[k])!=float(b[k])]
        residuals=[abs(a[k]-b[k])for k in a if k[1]=='postfit' and k[4]=='residual']
        if name=='red':assert changed, 'red case must exercise the polynomial contraction sites'
        records.append({'case':name,'inputs':{str(p):sha(p)for p in [par,tim]},'first_matrix_changed_entries':len(changed),'max_postfit_difference_s':str(max(residuals)),
                        'rust_comparison_failures':{v:len(r['failures'])for v,r in comparisons.items()},'output_sha256':{str(p):sha(p)for p in [candidate,directory/'fused.tsv',directory/'unfused.tsv']}})
    (BUILD/(profile+'.json')).write_text(json.dumps({'profile':profile,'pin':identity['pin'],'base_identity_sha256':sha(HERE/'build/native/identity.json'),'builder_sha256':sha(__file__),'rust_sha256':sha(rust),'versions':versions,'commands':commands,'records':records},indent=2)+'\n')
    print(json.dumps(records,indent=2))


if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--profile',choices=['debug','release'],default='debug');run(p.parse_args().profile)
