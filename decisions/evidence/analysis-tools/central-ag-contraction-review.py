"""Bounded native-only witness of the pinned Lomb compiler contraction profile."""
from pathlib import Path
import hashlib,json,subprocess,tempfile

root=Path('/Users/Eric.Dong/Downloads/tempo2rust')
pin='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
cc='/opt/homebrew/bin/g++-16'
ref=root/'validation/reference/build'
out=Path(tempfile.mkdtemp(prefix='tempo2-ag-contraction-review-'))
Path('/tmp/tempo2-ag-contraction-review-current').write_text(str(out)+'\n')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
raw=subprocess.check_output(['git','-C',str(root/'tempo2'),'show',pin+':TKspectrum.C'])
(out/'TKspectrum.original.C').write_bytes(raw)
source=raw.decode();start=source.index('void TKlomb_d(');end=source.index('// Fourier transform',start)
body=source[start:end].replace('void TKlomb_d(', 'void central_lomb(', 1)
(out/'lomb.cc').write_text('#include <cmath>\n#include "T2toolkit.h"\n'+body)
(out/'driver.cc').write_text(r'''#include <cstdio>
#include <cstdint>
#include <cstring>
#include <vector>
extern void central_lomb(double*,double*,int,double,double,double*,double*,int*,double*);
static void emit(int test,const char*kind,int i,double value){uint64_t bits;std::memcpy(&bits,&value,8);std::printf("%d\t%s\t%d\t%016llx\t%.17g\n",test,kind,i,(unsigned long long)bits,value);}
int main(){for(int test=0;test<2;test++){int n=test?17:16;std::vector<double>x(n),y(n),ox(n),oy(n);for(int i=0;i<n;i++){x[i]=test?(20*i+i%3)/16.0:i*1.25;y[i]=((i*i+3*i)%11)*1e-6;emit(test,"x",i,x[i]);emit(test,"y",i,y[i]);}int count=0;double variance=0;central_lomb(x.data(),y.data(),n,1,1,ox.data(),oy.data(),&count,&variance);emit(test,"count",0,count);emit(test,"variance",0,variance);for(int i=0;i<count;i++){emit(test,"frequency",i,ox[i]);emit(test,"power",i,oy[i]);}}}
''')
commands=[]
cmd=[cc,'-O2','-ffp-contract=off','-c',str(out/'driver.cc'),'-o',str(out/'driver.o')]
subprocess.run(cmd,check=True,capture_output=True);commands.append(cmd)
profiles={}
for name,flags in [('default',[]),('off',['-ffp-contract=off']),('fast',['-ffp-contract=fast'])]:
    obj=out/(name+'.o');exe=out/name
    cmd=[cc,'-O2',*flags,'-DHAVE_CONFIG_H','-I'+str(ref/'source'),'-c',str(out/'lomb.cc'),'-o',str(obj)]
    with (out/(name+'-compile.log')).open('w') as log:subprocess.run(cmd,check=True,stdout=log,stderr=subprocess.STDOUT)
    commands.append(cmd)
    cmd=[cc,str(out/'driver.o'),str(obj),str(ref/'libtempo2-reference.a'),'-lquadmath','-lgfortran','-o',str(exe)]
    with (out/(name+'-link.log')).open('w') as log:subprocess.run(cmd,check=True,stdout=log,stderr=subprocess.STDOUT)
    commands.append(cmd)
    with (out/(name+'.tsv')).open('w') as log:subprocess.run([str(exe)],check=True,stdout=log)
    profiles[name]={tuple(row[:3]):{'bits':row[3],'value':row[4]} for row in (line.split('\t') for line in (out/(name+'.tsv')).read_text().splitlines())}
assert profiles['default']==profiles['fast']
assert profiles['default'].keys()==profiles['off'].keys()
changes=[]
for key,a in profiles['default'].items():
    b=profiles['off'][key]
    if key[1]!='power':assert a==b,key
    elif a!=b:
        av,bv=float(a['value']),float(b['value'])
        changes.append({'case':int(key[0]),'bin':int(key[2]),'default':a,'off':b,'absolute_delta':abs(av-bv),'relative_to_default':abs(av-bv)/abs(av) if av else None})
compiler=subprocess.check_output([cc,'--version'],text=True)
optimizer=subprocess.check_output([cc,'-Q','--help=optimizers','-O2'],text=True)
(out/'compiler.txt').write_text(compiler)
(out/'optimizers.txt').write_text(optimizer)
shutil_copy=Path(__file__).read_bytes();(out/'central-ag-contraction-review.py').write_bytes(shutil_copy)
receipt={'scope':'Native-only pinned TKlomb_d body, symbol renamed without changing expressions; same driver object and reference archive across compiler contraction profiles. No Rust run, Linux execution, tolerance change or astrophysical validation.', 'pin':pin,'compiler':compiler.splitlines()[0],'target':subprocess.check_output([cc,'-dumpmachine'],text=True).strip(),'original_source_sha256':sha(out/'TKspectrum.original.C'),'archive_sha256':sha(ref/'libtempo2-reference.a'),'commands':commands,'default_equals_fast':True,'inputs_counts_variance_and_frequencies_exact':True,'changed_power_bins':changes,'files':{str(p.relative_to(out)):sha(p) for p in sorted(out.rglob('*')) if p.is_file()}}
(out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps({'evidence':str(out),'receipt_sha256':sha(out/'receipt.json'),'compiler':receipt['compiler'],'target':receipt['target'],'default_equals_fast':True,'changed_power_bins':changes},indent=2))
