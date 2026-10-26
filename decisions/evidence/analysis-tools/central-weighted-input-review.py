from pathlib import Path
import hashlib,json,subprocess,tempfile
root=Path('/Users/Eric.Dong/Downloads/tempo2rust');out=Path(tempfile.mkdtemp(prefix='tempo2-ag-weight-policy-'));Path('/tmp/tempo2-ag-weight-policy-current').write_text(str(out)+'\n')
pin='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
original=subprocess.check_output(['git','-C',str(root/'tempo2'),'show',pin+':TKspectrum.C']).decode();(out/'TKspectrum.original.C').write_text(original)
s=original.replace('#include "TKspectrum.h"','#include "TKspectrum.h"\nvoid central_weighted_input(int,double*,double*,double*);',1)
for call in ['TK_weightLS(smoothX,smoothY,e,nSmooth,specX,specY,&nSpec,outY_re,outY_im);','TK_fitSinusoids(smoothX,smoothY,e,nSmooth,specX,specY,&nSpec);']:
 assert s.count(call)==1
 s=s.replace(call,'central_weighted_input(nSmooth,smoothX,smoothY,e); *nout=0; return 0.0;\n            '+call)
(out/'TKspectrum.C').write_text(s)
(out/'probe.cc').write_text(r'''#include "tempo2.h"
#include "TKspectrum.h"
#include <cstdio>
static const char* current;
void central_weighted_input(int n,double*x,double*y,double*e){
 printf("count\t%s\t%d\n",current,n);
 for(int i=0;i<n;i++)printf("row\t%s\t%d\t%.17g\t%.17g\t%.17g\n",current,i,x[i],y[i],e[i]);
}
int main(){
 MAX_OBSN=32;
 for(int k=0;k<5;k++){
  double x[]={0,.25,2,2.25,4,4.25,6,6.25};
  double y[]={1,5,2,6,3,7,4,8};
  double e[]={1,3,2,4,3,5,4,6};
  double ox[64]={},oy[64]={},re[64]={},im[64]={};int nout=0;
  current=k==0?"direct":k==1?"average":k==2?"hann":k==3?"difference":"average_sinusoids";
  TKspectrum(x,y,e,8,(k==1||k==4)?1:0,3,k==2?1:0,0,k==3?1:0,k==4?5:4,1,1,3,ox,oy,&nout,0,0,re,im);
 }
}
''')
cc='/opt/homebrew/bin/g++-16';headers=root/'validation/reference/build/source';archive=root/'validation/reference/build/libtempo2-reference.a'
cmd=[cc,'-O2','-DHAVE_CONFIG_H','-I'+str(headers),str(out/'probe.cc'),str(out/'TKspectrum.C'),str(archive),'-lquadmath','-lgfortran','-o',str(out/'probe')]
with (out/'build.log').open('w') as f:p=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
if p.returncode:print((out/'build.log').read_text()[-4000:]);raise SystemExit(p.returncode)
with (out/'observed.tsv').open('w') as f:subprocess.run([str(out/'probe')],stdout=f,check=True)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
r={'scope':'Instrumented pinned driver with actual source transforms, stopping immediately at weighted estimator dispatch. No spectrum computation or scientific validation is claimed; no spline/OOB path executed.', 'pin':pin,'command':cmd,'original_source_sha256':sha(out/'TKspectrum.original.C'),'instrumented_sha256':sha(out/'TKspectrum.C'),'archive_sha256':sha(archive),'executable_sha256':sha(out/'probe'),'observed_sha256':sha(out/'observed.tsv')}
(out/'receipt.json').write_text(json.dumps(r,indent=2)+'\n');print(out);print((out/'observed.tsv').read_text());print('receipt',sha(out/'receipt.json'))
