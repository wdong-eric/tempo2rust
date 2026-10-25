"""Natural PAR/TIM covariance selectors, debug files and failure recovery."""
from pathlib import Path
import argparse, hashlib, json, math, os, subprocess
from fitting_corrections import corrected
from check_kernels import compare, read_native
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
BUILD=HERE/'build/covariance-models';REF=ROOT/'validation/reference/build'
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()

def run(profile):
    BUILD.mkdir(parents=True,exist_ok=True)
    identity=json.loads((HERE/'build/native/identity.json').read_text());objects=[];commands=[]
    for name in ['cholesky.C','choleskyRoutines.C']:
        source=corrected(name,subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',identity['pin']+':'+name]).decode())
        if name=='cholesky.C':
            token='    int ret = cholesky_formL(cholesky_L,m,np);';assert source.count(token)==1
            source=source.replace(token,'    extern void ae_covariance(double**,int);ae_covariance(m,np);\n'+token)
        path=BUILD/name;path.write_text(source);obj=BUILD/(name+'.o');objects.append(obj)
        cmd=identity['commands'][0].copy();cmd[cmd.index('-c')+1]=str(path);cmd[cmd.index('-o')+1]=str(obj)
        p=subprocess.run(cmd,capture_output=True);assert p.returncode==0,p.stderr.decode();commands.append(cmd)
    assemblies={}
    for obj in objects:
        assembly=BUILD/(obj.stem+'-assembly.txt')
        assembly.write_bytes(subprocess.check_output(['xcrun','llvm-objdump','--disassemble','--demangle',str(obj)]))
        assemblies[str(assembly)]=sha(assembly)
    native=BUILD/'native';cmd=identity['commands'][-1].copy();cmd[cmd.index(str(HERE/'native_fit.cc'))]=str(HERE/'covariance_models_probe.cc')
    cmd=[arg for arg in cmd if Path(arg).name not in {p.name for p in objects}]
    at=cmd.index(str(REF/'libtempo2-reference.a'));cmd[at:at]=list(map(str,objects));cmd[cmd.index('-o')+1]=str(native)
    p=subprocess.run(cmd,capture_output=True);assert p.returncode==0,p.stderr.decode();commands.append(cmd)
    rust=ROOT/'rust/target'/profile/'examples/fitting_covariance_models';runtime=REF/'source/T2runtime'
    par=ROOT/'validation/fitting/inputs/f0.par';tim=HERE/'inputs/multi-frequency.tim'
    inputs=BUILD/'inputs';inputs.mkdir(exist_ok=True)
    dcf=inputs/'daily.dcf';dcf.write_text('2\n'+''.join('%.17g\n'%(1e-12*math.exp(-i*i/1800))for i in range(242))+'ignored trailing text\n')
    ecm=inputs/'extra.ecm';ecm.write_text(''.join('%.17g\n'%(1e-13 if i==j else 0.)for i in range(16)for j in range(16))+'ignored trailing text\n')
    include=inputs/'include.model';include.write_text('MJD 53980 54020\nMODEL T2Matern1.5 1e-12 30\nEND\nMODEL T2SqExp 1e-13 15\n')
    selector=inputs/'selection.model';selector.write_text('MODEL T2\nPSR OTHER\nMODEL Unknown\nEND\nPSR J0000-0000\nMJD 53950 54050 # source ignores this tail\nINCLUDE '+str(include)+'\nMODEL T2SqExp 2e-13 20\nCLEAR\nMODEL T2SqExp 1e-13 10\n')
    legacy1=inputs/'model1';legacy1.write_text('MODEL 1\nALPHA 4\nFC 3\nAMP 1e-27\n')
    legacy2=inputs/'model2';legacy2.write_text('MODEL 2\nALPHA 4\nBETA .5\nFC 3\nAMP 1e-27\n')
    legacy_numbers=inputs/'legacy-numbers';legacy_numbers.write_text('MODEL 2\nALPHA 0x1p2 trailing\nBETA .5tail\nFC 3\nAMP 1e-27\n')
    cases={
        'power':'MODEL T2PowerLaw 4 1e-27 3', 'power-dm':'MODEL T2PowerLawDM 4 1e-21 3',
        'year':'MODEL T2YrPowerLaw 4 1e-27 3', 'year-dm':'MODEL T2YrPowerLawDM 4 1e-21 3',
        'year-x':'MODEL T2YrPowerLaw_X 4 1e-32',
        'qp':'MODEL T2PowerLaw_QPc 4 1e-27 3 2 .1 .03 1',
        'redpink':'MODEL T2PowerLaw_RedPink 4 1e-27 3 1.5 15',
        'sqexp':'MODEL T2SqExp 1e-12 30', 'sqexp-dm':'MODEL T2SqExpDM 1e-6 30',
        'matern15':'MODEL T2Matern1.5 1e-12 30','matern15-dm':'MODEL T2Matern1.5DM 1e-6 30',
        'matern25':'MODEL T2Matern2.5 1e-12 30','matern25-dm':'MODEL T2Matern2.5DM 1e-6 30',
        'dm':'MODEL DM 4 2 1400','dmparam':'MODEL DMCovarParam 1.5 1e-12 30',
        'selection':str(selector),'legacy1':str(legacy1),'legacy2':str(legacy2),'legacy-numbers':str(legacy_numbers),
        'dcf':str(dcf),'dcf-directive':'MODEL T2\nDCF '+str(dcf),
        'ecm':'MODEL T2\nECM '+str(ecm),'psrj':'PSRJ',
        'comment':'MODEL T2SqExp 1e-12 30 # source ignores this tail',
        'numeric-prefix':'MODEL T2SqExp 1e-12 30ignored',
        'hexadecimal':'MODEL T2SqExp 0x1p-40 0x1ep0','recovery':'recovery',
    }
    records=[]
    for name,model in cases.items():
        out=BUILD/profile/name;out.mkdir(parents=True,exist_ok=True);dirs={engine:out/engine for engine in ['native','rust']}
        for directory in dirs.values():
            directory.mkdir(exist_ok=True)
            (directory/'covarFunc.dat_J0000-0000').write_bytes(dcf.read_bytes())
            (directory/'cycle.model').write_text('INCLUDE cycle.model\n')
        trace=dirs['native']/'values.txt';nm=cases['sqexp'] if name=='recovery' else model
        cmds=[[str(native),str(trace),str(par),str(tim),nm,'1'],[str(rust),str(runtime),str(par),str(tim),model,str(dirs['rust'])]]
        for (engine,directory),command in zip(dirs.items(),cmds):
            p=subprocess.run(command,cwd=directory,env={**os.environ,'TEMPO2':str(runtime)},capture_output=True,timeout=60)
            (directory/'stdout').write_bytes(p.stdout);(directory/'stderr').write_bytes(p.stderr)
            assert p.returncode==0,(name,engine,p.returncode,p.stderr.decode())
        got=json.loads((dirs['rust']/'stdout').read_text());reference=read_native(trace.read_text())
        failures=compare(reference,{k:got[k]for k in ['covariance','lower','determinant']})
        assert got['indices']==[int(r.split()[2])for r in trace.read_text().splitlines()if r.startswith('INDEX ')]
        assert len(got['errors'])==(4 if name=='recovery' else 0)
        for item in got['inputs']:
            path=Path(item['path']);path=path if path.is_absolute() else dirs['rust']/path
            assert item['sha256']==sha(path) and bytes(item['bytes'])==path.read_bytes()
        file_names=['chol.covarMatrix','chol.L']+(['newDCF']if name not in ['dmparam','dcf','dcf-directive','ecm','psrj'] else [])
        assert set(Path(f).name for f in got['files'])==set(file_names),(name,got['files'])
        files={}
        for f in file_names:
            files[f]={engine:sha(directory/f) for engine,directory in dirs.items()}
            if files[f]['native']!=files[f]['rust']:failures.append({'file':f,'kind':'bytes'})
        records.append({'case':name,'commands':cmds,'selector':model,'numeric_fields':len(reference),'failures':failures,
            'files':files,'input_files':[{'path':i['path'],'sha256':i['sha256'],'role':i['role']}for i in got['inputs']],
            'native_sha256':sha(trace),'rust_sha256':sha(dirs['rust']/'stdout'),'recovery_errors':got['errors']})
    receipt={'profile':profile,'pin':identity['pin'],'base_identity_sha256':sha(HERE/'build/native/identity.json'),
        'builder_sha256':sha(__file__),'probe_sha256':sha(HERE/'covariance_models_probe.cc'),'commands':commands,'assembly_sha256':assemblies,
        'native_executable_sha256':sha(native),'rust_executable_sha256':sha(rust),'inputs':{str(p):sha(p)for p in [par,tim,*inputs.iterdir()]},'records':records}
    (BUILD/(profile+'.json')).write_text(json.dumps(receipt,indent=2)+'\n')
    print('covariance model failures',sum(len(r['failures'])for r in records))
    for r in records:
        if r['failures']:print(r['case'],r['failures'][:3])

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--profile',choices=['debug','release'],default='debug');run(p.parse_args().profile)
