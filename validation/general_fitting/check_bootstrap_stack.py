"""Natural IBOOT=2 call: original frame limit, corrected heap and four fits."""
from pathlib import Path
import hashlib, json, os, resource, subprocess
from fitting_corrections import corrected

ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
BUILD=HERE/'build/bootstrap-stack';BUILD.mkdir(parents=True,exist_ok=True)
REF=ROOT/'validation/reference/build';runtime=REF/'source/T2runtime'
identity=json.loads((HERE/'build/native/identity.json').read_text())
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
par=HERE/'inputs/bootstrap.par';tim=ROOT/'validation/fitting/inputs/bary.tim'
commands=[];records=[]
util=BUILD/'tempo2Util.C'
util.write_text(corrected('tempo2Util.C',subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',identity['pin']+':tempo2Util.C']).decode()))
cmd=identity['commands'][0].copy();cmd[cmd.index('-c')+1]=str(util);cmd[cmd.index('-o')+1]=str(util.with_suffix('.o'))
p=subprocess.run(cmd,capture_output=True);assert p.returncode==0,p.stderr.decode();commands.append(cmd)
original=subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',identity['pin']+':bootstrap.C']).decode()
for version in ['original','corrected']:
    directory=BUILD/version;directory.mkdir(exist_ok=True)
    source=original if version=='original' else corrected('bootstrap.C',original)
    token='    if (displayCVSversion == 1)';assert source.count(token)==1
    source=source.replace(token,'    fprintf(stderr,"ENTER bootstrap\\n");\n'+token)
    source=source.replace('        t2Fit(&psr[p],1,NULL);','        t2Fit(&psr[p],1,NULL);\n        fprintf(stderr,"BOOTSTRAP FIT %d\\n",iter);')
    src=directory/'bootstrap.C';src.write_text(source);obj=directory/'bootstrap.o'
    cmd=identity['commands'][0].copy();cmd[cmd.index('-c')+1]=str(src);cmd[cmd.index('-o')+1]=str(obj);cmd.append('-fstack-usage')
    p=subprocess.run(cmd,capture_output=True);assert p.returncode==0,p.stderr.decode();commands.append(cmd)
    native=directory/'native';cmd=identity['commands'][-1].copy();cmd[cmd.index(str(HERE/'native_fit.cc'))]=str(HERE/'native_bootstrap.cc')
    at=cmd.index(str(REF/'libtempo2-reference.a'));cmd[at:at]=[str(obj),str(util.with_suffix('.o'))];cmd[cmd.index('-o')+1]=str(native)
    p=subprocess.run(cmd,capture_output=True);assert p.returncode==0,p.stderr.decode();commands.append(cmd)
    command=[str(native),str(directory/'history.tsv'),str(par),str(tim)]
    p=subprocess.run(command,cwd=directory,env={**os.environ,'TEMPO2':str(runtime)},capture_output=True,timeout=60)
    (directory/'stdout').write_bytes(p.stdout);(directory/'stderr').write_bytes(p.stderr)
    entered=b'ENTER bootstrap' in p.stderr
    fits=[l for l in p.stderr.decode().splitlines()if l.startswith('BOOTSTRAP FIT ')]
    if version=='original':assert p.returncode<0 and not entered and not fits,(p.returncode,p.stderr[-500:])
    else:assert p.returncode==0 and entered and fits==['BOOTSTRAP FIT '+str(i)for i in range(4)],(p.returncode,p.stderr[-500:])
    stack=next(l for l in (directory/'bootstrap.su').read_text().splitlines()if 'int bootstrap(' in l)
    records.append({'version':version,'command':command,'exit':p.returncode,'entered':entered,'fits':fits,'stack_usage':stack,'source_sha256':sha(src),'executable_sha256':sha(native),'stderr_sha256':sha(directory/'stderr'),'history_sha256':sha(directory/'history.tsv')})
(BUILD/'receipt.json').write_text(json.dumps({'pin':identity['pin'],'builder_sha256':sha(__file__),'probe_sha256':sha(HERE/'native_bootstrap.cc'),'base_identity_sha256':sha(HERE/'build/native/identity.json'),'stack_limit_bytes':resource.getrlimit(resource.RLIMIT_STACK),'inputs':{str(p):sha(p)for p in [par,tim]},'commands':commands,'records':records},indent=2)+'\n')
print('original frame fails before entry; heap correction completes four natural bootstrap fits')
