"""AG isolated native witness build. Never writes accepted source or evidence."""
from pathlib import Path
import subprocess,json,hashlib,difflib,sys
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
PIN='fda6ad4a9753ad7dc5d291406f2f310a9161423f';REF=ROOT/'validation/reference/build'
CXX='/opt/homebrew/bin/g++-16'
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def build(out,corrected=True):
 out=Path(out).resolve();out.mkdir(parents=True,exist_ok=False);src=out/'source';src.mkdir();objects=[];commands=[];patch='';sources={}
 for name in ['T2toolkit.C','TKspectrum.C','GWsim.C','TKrobust.C']:
  old=subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',PIN+':'+name]).decode();new=old
  if name=='TKrobust.C':
   old=(ROOT/'validation/general_fitting/build/native/source/TKrobust.C').read_text();new=old
   if corrected:new=new.replace('Ocvm[i][j] = cvm_ls[i][j];','if(Ocvm)Ocvm[i][j] = cvm_ls[i][j];').replace('e[i] = sqrt(cvm[i][i]* ecorrect);','if(e)e[i] = sqrt(cvm[i][i]* ecorrect);')
  if name=='T2toolkit.C':
   new=new.replace('long TKsetSeed()', 'static time_t ag_time(time_t*){return 1793000000;}\nlong TKsetSeed()').replace('time(NULL)','ag_time(NULL)')
   new=new.replace('static int rand_mti=RAND_N+1;','unsigned long long ag_draws=0;\nstatic int rand_mti=RAND_N+1;')
   new=new.replace('y = rand_mt[rand_mti++];','ag_draws++;\n    y = rand_mt[rand_mti++];')
   new+='\nint ag_index(){return rand_mti;}\nunsigned long ag_word(int i){return rand_mt[i];}\n'
  if corrected and name=='TKspectrum.C':
   new=new.replace('TKsortit(x,y,n);','TKsort_3d(x,y,e,n);',1)
   for var in ['interpX','interpY','smoothX','smoothY']:
    new=new.replace(var+'[n]',var+'[n>4096?n:4096]')
   new=new.replace('double yd[MAX_OBSN][4];','double yd[MAX_OBSN>4096?MAX_OBSN:4096][4];')
   new=new.replace('(s4-s1/s2)/(s5-s3/s2)','(s4-s1*s3/s2)/(s5-s3*s3/s2)')
   new=new.replace('double** weights=malloc_blas(1,n);','double** weights=malloc_blas(n,n);\n    for(int i=0;i<n;i++)for(int j=0;j<n;j++)weights[i][j]=0;')
   new=new.replace('weights[0][k] = 1.0 / sig[k];','weights[k][k] = sig[k];')
   new=new.replace('calcSpectraErr(weights,x,y,n,outX,outY,outE,*outN);','calcSpectraErr(weights,x,y,n,outX,outY,outE,*outN);\n    free_blas(weights);')
   new=new.replace('TKleastSquares(data,white_data,','TKleastSquares(resy,white_data,')
   new=new.replace('int nalv,nread,n;','int nalv,nread=0,n;')
   # Source undefined no-selected-pivot case becomes its existing nbad error.
   new=new.replace('amax=0.0;','amax=0.0; irow=icolum=0;',1)
   new=new.replace('        ipivot[icolum]=ipivot[icolum]+1;','        if(!icolum){*nbad=1;return;}\n        ipivot[icolum]=ipivot[icolum]+1;')
  if corrected and name=='GWsim.C':
   new=new.replace('gw[k].across_g==0.','gw[k].across_g=0.')
   new=new.replace('else psrVal1 = longdouble(0.0);','else {psrVal1=longdouble(0.0);psrVal1_im=longdouble(0.0);}')
  p=src/name;p.write_text(new);sources[name]={'original':hashlib.sha256(old.encode()).hexdigest(),'executed':sha(p)}
  patch+=''.join(difflib.unified_diff(old.splitlines(True),new.splitlines(True),fromfile='pinned/'+name,tofile='executed/'+name))
  obj=out/(name+'.o');cmd=[CXX,'-O2','-DHAVE_CONFIG_H','-I'+str(REF/'source'),'-c',str(p),'-o',str(obj)];subprocess.run(cmd,check=True,stdout=(out/(name+'.log')).open('w'),stderr=subprocess.STDOUT);commands.append(cmd);objects.append(obj)
 # Reuse exactly the reviewed AE/AD/AC objects with their observer stubs.
 ae=ROOT/'validation/general_fitting/build/native';identity=json.loads((ae/'identity.json').read_text())
 for name,digest in identity['objects'].items():
  p=Path(name);assert sha(p)==digest,(p,'accepted dependency drift')
  if p.name!='TKrobust.C.o':objects.append(p)
 archive=REF/'libtempo2-reference.a';assert sha(archive)==identity['archive_sha256']
 exe=out/'analysis';cmd=[CXX,'-O2','-ffp-contract=off','-DHAVE_CONFIG_H','-I'+str(REF/'source'),'-I'+str(ROOT/'validation/general_fitting'),str(HERE/'native.cc'),*map(str,objects),str(archive),'-lquadmath','-lgfortran','-o',str(exe)]
 subprocess.run(cmd,check=True,stdout=(out/'link.log').open('w'),stderr=subprocess.STDOUT);commands.append(cmd);(out/'corrections.patch').write_text(patch)
 (out/'identity.json').write_text(json.dumps({'pin':PIN,'corrected':corrected,'sources':sources,'objects':{str(p):sha(p) for p in objects},'archive':sha(archive),'commands':commands,'executable':sha(exe),'harness':{p.name:sha(p) for p in [HERE/'native.cc',HERE/'components.h',HERE/'exchange.h']},'builder':sha(__file__),'patch':sha(out/'corrections.patch')},indent=2)+'\n');return exe
if __name__=='__main__':print(build(sys.argv[1],'--original' not in sys.argv))
