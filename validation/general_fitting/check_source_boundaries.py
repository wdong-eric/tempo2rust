"""0053 extent/initialization witnesses; never evaluate undefined native values."""
from pathlib import Path
import difflib, hashlib, json, os, subprocess
from fitting_corrections import corrected
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
BUILD=HERE/'build/source-boundaries';BUILD.mkdir(parents=True,exist_ok=True)
identity=json.loads((HERE/'build/native/identity.json').read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
records=[]
for version in ['original','corrected']:
    directory=BUILD/version;directory.mkdir(exist_ok=True);objects=[];commands=[]
    for name in ['t2fit.C','choleskyAutomatic.C','choleskyRoutines.C']:
        old=subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',identity['pin']+':'+name]).decode()
        source=corrected(name,old) if version=='corrected' else old
        (directory/(name+'.patch')).write_text(''.join(difflib.unified_diff(old.splitlines(True),source.splitlines(True),fromfile='original/'+name,tofile=version+'/'+name)))
        if name=='choleskyAutomatic.C':
            source=source.replace('    resx =', '    extern void* ae_allocate(size_t);\n    resx =',1)
            for var in ['resx','resy','rese','specX','specY','specY_err']:
                source=source.replace(var+' = (double*) malloc(',var+' = (double*) ae_allocate(')
            token='    nres = T2obtainTimingResiduals(psr,resx,resy,rese);'
            source=source.replace(token,token+'\nextern void ae_observe_residuals(double*,double*,double*,int);ae_observe_residuals(resx,resy,rese,nres);return;')
        elif name=='choleskyRoutines.C':
            token='            resx[nres] ='
            source=source.replace(token,'            extern void ae_check_extent(double*,int);ae_check_extent(resx,nres);ae_check_extent(resy,nres);ae_check_extent(rese,nres);\n'+token)
        else:
            start=source.rindex('void subtractTNPoly(');end=source.index('void subtractSWGPMeanSW(',start);block=source[start:end]
            block=block.replace('        int nParams = 0;','        int nParams = 0; int ae_written=0;')
            token='                        white_y[ifit] ='
            block=block.replace(token,'                        if(iparam==0)++ae_written;\n'+token)
            token='            TKleastSquares(psr_y, white_y,'
            block=block.replace(token,'            extern void ae_initialized(int,int);ae_initialized(ae_written,psr_ndata);\n'+token)
            source=source[:start]+block+source[end:]
        path=directory/name;path.write_text(source);obj=directory/(name+'.o');objects.append(obj)
        cmd=identity['commands'][0].copy();cmd[cmd.index('-c')+1]=str(path);cmd[cmd.index('-o')+1]=str(obj)
        result=subprocess.run(cmd,capture_output=True);(directory/"compile.log").write_bytes(result.stdout+result.stderr);assert result.returncode==0,result.stderr.decode();commands.append(cmd)
    native=directory/'native';cmd=identity['commands'][-1].copy();cmd[cmd.index(str(HERE/'native_fit.cc'))]=str(HERE/'source_boundary_probe.cc')
    cmd.remove(str(HERE/'build/native/t2fit.C.o'));at=cmd.index(str(ROOT/'validation/reference/build/libtempo2-reference.a'));cmd[at:at]=list(map(str,objects));cmd[cmd.index('-o')+1]=str(native)
    result=subprocess.run(cmd,capture_output=True);(directory/"compile.log").write_bytes(result.stdout+result.stderr);assert result.returncode==0,result.stderr.decode();commands.append(cmd)
    for case in ['allocation','constant','fitted']:
        out=directory/case;out.mkdir(exist_ok=True)
        command=[str(native),case,str(ROOT/'validation/fitting/inputs/f0.par'),str(ROOT/'validation/fitting/inputs/bary.tim')]
        p=subprocess.run(command,cwd=out,env={**os.environ,'TEMPO2':str(ROOT/'validation/reference/build/source/T2runtime')},capture_output=True,timeout=120)
        (out/'stdout').write_bytes(p.stdout);(out/'stderr').write_bytes(p.stderr)
        values=[s for s in p.stdout.decode().splitlines() if s.startswith(('BOUNDARY','EXTENT','RES ','F0 ','MASK ','TN '))]
        assert p.returncode==(86 if version=='original' and case!='fitted' else 0),(version,case,p.returncode,p.stderr.decode())
        records.append({'version':version,'case':case,'commands':commands+[command],'executable_sha256':sha(native),'stdout_sha256':sha(out/'stdout'),'exit':p.returncode,'values':values})
original=next(r['values'] for r in records if r['version']=='original' and r['case']=='fitted')
new=next(r['values'] for r in records if r['version']=='corrected' and r['case']=='fitted')
assert original==new
assert 'MASK 7' in next(r['values'] for r in records if r['version']=='corrected' and r['case']=='constant')
(BUILD/'receipt.json').write_text(json.dumps({'pin':identity['pin'],'builder_sha256':sha(__file__),'probe_sha256':sha(HERE/'source_boundary_probe.cc'),'records':records},indent=2)+'\n')
for r in records:print(r['version'],r['case'],r['exit'],r['values'][:2])
