"""Repeat new DDH/chromatic-log fits with identical native or Rust solver inputs."""
from pathlib import Path
from decimal import Decimal as D
import argparse, hashlib, json, subprocess
import check_solver_contract as solver
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
BUILD=HERE/'build/pipeline-solvers';sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()

def run(profile):
    BUILD.mkdir(parents=True,exist_ok=True);native,commands=solver.build()
    rust=ROOT/'rust/target'/profile/'examples/fitting_solver';records=[]
    cases={name:HERE/'build/ddh'/profile/name for name in ['current','historical']}
    cases['chromatic-log-noise']=HERE/'build/public'/profile/'chromatic-log-noise'
    for name,directory in cases.items():
        assert (directory/'rust.json').exists(), ('required pipeline export missing',directory)
        candidate=json.loads((directory/'rust.json').read_text())
        ref={}
        for line in (directory/'native.tsv').read_text().splitlines():
            cycle,stage,p,i,key,value=line.split('\t');ref[int(cycle),stage,int(p),int(i),key]=D(value)
        system=candidate['cycles'][0]['systems'][0]
        at=lambda stage,i,key:ref[1,stage,0,i,key]
        n=int(at('prepared',-1,'ndata'));nf=int(at('prepared',-1,'nparams'));nc=int(at('prepared',-1,'nconstraints'))
        mode=int(at('prepared',-1,'fit_mode'));rescale=int(mode==0 or (mode==1 and at('prepared',-1,'rescale_err_chisq')==1))
        for owner in ['native','rust']:
            out=BUILD/profile/name/owner;out.mkdir(parents=True,exist_ok=True)
            if owner=='native':
                data=[at('prepared',i,'y')for i in range(n)];white=[at('prepared',i,'white_y')for i in range(n)]
                matrix=[[at('prepared',i,'matrix.'+str(j))for j in range(nf)]for i in range(n)]
                wm=[[at('prepared',i,'white_matrix.'+str(j))for j in range(nf)]for i in range(n)]
                constraints=[[at('constraint',i,'matrix.'+str(j))for j in range(nf)]for i in range(nc)]
                targets=[at('constraint',i,'target')for i in range(nc)]
            else:
                data=system['data'];white=system['white_data'];matrix=system['matrix'];wm=system['white_matrix'];constraints=system['constraints'];targets=system['constraint_targets']
            fmt=lambda values:' '.join(format(float(v),'.17g')for v in values)+'\n'
            text=f'{n} {nf} {nc} {rescale}\n'+''.join(fmt([data[i],white[i],*[v for j in range(nf)for v in [matrix[i][j],wm[i][j]]]])for i in range(n))
            text+=''.join(fmt([targets[i],*constraints[i]])for i in range(nc))
            source=out/'input.txt';source.write_text(text);trace=out/'native.txt'
            cmds=[[str(native),str(source),str(trace),'7','1','0','0','0'],[str(rust),str(source),'7','1','0']]
            for engine,command in zip(['native','rust'],cmds):
                proc=subprocess.run(command,cwd=out,capture_output=True,timeout=120)
                (out/(engine+'.stderr')).write_bytes(proc.stderr);(out/(engine+'.stdout')).write_bytes(proc.stdout)
                assert proc.returncode==0,(name,owner,engine,proc.stderr.decode())
            got=json.loads((out/'rust.stdout').read_text())
            failures=solver.compare(trace.read_text(),got,matrix,data,7,True,mode)
            lines=[r.split()for r in trace.read_text().splitlines()];values={tuple(r[:-1]):r[-1]for r in lines if r[0]!='S'}
            reproduction=[]
            if owner=='native':
                pairs=[('CHI',values['CHI',],at('fit',-1,'chi'))]
                pairs += [(f'P.{j}',values['P',str(j)],at('fit',j,'increment'))for j in range(nf)]
                pairs += [(f'E.{j}',values['E',str(j)],at('fit',j,'error'))for j in range(nf)]
                pairs += [(f'C.{j}.{k}',values['C',str(j),str(k)],at('fit',j,'covariance.'+str(k)))for j in range(nf)for k in range(nf)]
            else:
                fit=system['solver'];pairs=[('CHI',got['chi_square'],fit['chi_square'])]
                pairs += [(f'P.{j}',got['increments'][j],fit['increments'][j])for j in range(nf)]
                pairs += [(f'E.{j}',got['errors'][j],fit['errors'][j])for j in range(nf)]
                pairs += [(f'C.{j}.{k}',got['covariance'][j][k],fit['covariance'][j][k])for j in range(nf)for k in range(nf)]
            for key,a,b in pairs:
                if float(a).hex()!=float(b).hex():reproduction.append({'quantity':key,'standalone':str(a),'pipeline':str(b)})
            record={'case':name,'input_owner':owner,'commands':cmds,'input_sha256':sha(source),'pipeline_export_sha256':sha(directory/('native.tsv'if owner=='native'else'rust.json')),
                'native_output_sha256':sha(trace),'rust_output_sha256':sha(out/'rust.stdout'),'failures':failures,'same_engine_reproduction_failures':reproduction,
                'native_chi':values['CHI',],'rust_chi':got['chi_square'],'raw_native_singular_values':[r[2]for r in lines if r[0]=='S']}
            records.append(record);print(name,owner,'shared failures',len(failures),'reproduction failures',len(reproduction),flush=True)
    receipt={'profile':profile,'pin':solver.PIN,'builder_sha256':sha(__file__),'comparator_sha256':sha(HERE/'check_solver_contract.py'),
        'native_sha256':sha(native),'rust_sha256':sha(rust),'commands':commands,'records':records}
    (BUILD/(profile+'.json')).write_text(json.dumps(receipt,indent=2)+'\n')

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--profile',choices=['debug','release'],default='debug');run(p.parse_args().profile)
