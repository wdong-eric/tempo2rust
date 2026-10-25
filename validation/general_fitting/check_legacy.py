"""Natural single/global legacy APIs, multiplication weights and full lower L."""
from pathlib import Path
from decimal import Decimal as D
import argparse, hashlib, json, os, subprocess
from fitting_corrections import corrected
from check_solver_contract import compare
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
BUILD=HERE/'build/legacy';REF=ROOT/'validation/reference/build';sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()


def run(profile):
 BUILD.mkdir(parents=True,exist_ok=True);identity=json.loads((HERE/'build/native/identity.json').read_text());objects=[];commands=[]
 for name in ['TKfit.C','t2fit.C']:
  source=corrected(name,subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',identity['pin']+':'+name]).decode())
  if name=='TKfit.C':
   start=source.index('double TKrobustDefConstrainedLeastSquares(');pos=source.index('{',start)+1
   source=source[:pos]+'\nextern void ae_solver_input(double*,double*,double**,double**,double**,int,int,int);ae_solver_input(data,white_data,designMatrix,white_designMatrix,constraintsMatrix,ndata,nparams,nconstraints);\n'+source[pos:]
   token='TKsingularValueDecomposition_lsq(augmented_DM,ndata+nconstraints,nparams,v,w,u);'
   source=source.replace(token,token+'\nextern void ae_solver_svd(int,const longdouble*);ae_solver_svd(nparams,w);')
  src=BUILD/name;src.write_text(source);obj=BUILD/(name+'.o');objects.append(obj)
  cmd=identity['commands'][0].copy();cmd[cmd.index('-c')+1]=str(src);cmd[cmd.index('-o')+1]=str(obj)
  p=subprocess.run(cmd,capture_output=True);assert p.returncode==0,p.stderr.decode();commands.append(cmd)
 cmd=identity['commands'][-1].copy();cmd[cmd.index(str(HERE/'native_fit.cc'))]=str(HERE/'native_legacy.cc')
 for name in ['TKfit.C','t2fit.C']:cmd.remove(str(HERE/'build/native'/(name+'.o')))
 at=cmd.index(str(REF/'libtempo2-reference.a'));cmd[at:at]=list(map(str,objects));native=BUILD/'native';cmd[cmd.index('-o')+1]=str(native)
 p=subprocess.run(cmd,capture_output=True);assert p.returncode==0,p.stderr.decode();commands.append(cmd)
 rust=ROOT/'rust/target'/profile/'examples/fitting_legacy';runtime=REF/'source/T2runtime';pars=[HERE/'inputs/global0.par',HERE/'inputs/global1.par'];tim=ROOT/'validation/fitting/inputs/bary.tim';records=[]
 for mode,lower in [('single','diagonal'),('single','full'),('dcm','diagonal'),('dcm','full'),('sigma','diagonal'),('global','diagonal'),('global','full')]:
  dest=BUILD/profile/(mode+'-'+lower);dest.mkdir(parents=True,exist_ok=True);trace=dest/'native.txt'
  cmds=[[str(native),str(trace),mode,lower,*map(str,pars),str(tim)],[str(rust),mode,lower,str(runtime),*map(str,pars),str(tim)]]
  for engine,cmd in zip(['native','rust'],cmds):
   p=subprocess.run(cmd,capture_output=True,env={**os.environ,'TEMPO2':str(runtime)},timeout=120);(dest/(engine+'.stdout')).write_bytes(p.stdout);(dest/(engine+'.stderr')).write_bytes(p.stderr);assert p.returncode==0,(mode,lower,engine,p.stderr.decode())
  result=json.loads((dest/'rust.stdout').read_text());rows=[r.split()for r in trace.read_text().splitlines()];shape=next(r for r in rows if r[0]=='SHAPE');n,nf,nc=map(int,shape[1:]);assert nc==0
  matrix=[[None]*nf for _ in range(n)];data=[None]*n;failures=[]
  for r in rows:
   if r[0]=='A':matrix[int(r[1])][int(r[2])]=r[3]
   if r[0]=='D':data[int(r[1])]=r[2]
  final='\n'.join(' '.join(r)for r in rows if r[0]in ['S','CHI','P','E','C'])
  failures+=compare(final,result['solver'],matrix,data,7,True)
  for r in rows:
   if r[0]=='A':
    i,j=map(int,r[1:3])
    for name,value in [('matrix',r[3]),('white_matrix',r[4])]:
     scale=max(abs(D(a[3 if name=='matrix'else 4]))for a in rows if a[0]=='A'and int(a[2])==j);actual=D(result['design'][name][i][j]);expected=D(value)
     if abs(actual-expected)>D('5e-14')*scale or(expected==0 and actual!=0):failures.append({'quantity':f'{name}.{i}.{j}','native':value,'rust':str(actual)})
   elif r[0]=='D':
    i=int(r[1])
    for name,value in [('data',r[2]),('white_data',r[3])]:
     scale=max(abs(D(a[2 if name=='data'else 3]))for a in rows if a[0]=='D');actual=D(result['design'][name][i]);expected=D(value)
     if abs(actual-expected)>D('5e-14')*scale:failures.append({'quantity':f'{name}.{i}','native':value,'rust':str(actual)})
  records.append({'mode':mode,'lower':lower,'commands':cmds,'failures':failures,'native_output_sha256':sha(trace),'rust_output_sha256':sha(dest/'rust.stdout')});print(mode,lower,len(failures),failures[:2])
 out=BUILD/profile/'receipt.json';out.write_text(json.dumps({'pin':identity['pin'],'profile':profile,'commands':commands,'builder_sha256':sha(__file__),'probe_sha256':sha(HERE/'native_legacy.cc'),'inputs':{str(p):sha(p)for p in pars+[tim]},'native_sha256':sha(native),'rust_sha256':sha(rust),'records':records},indent=2)+'\n')


if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--profile',choices=['debug','release'],default='debug');run(p.parse_args().profile)
