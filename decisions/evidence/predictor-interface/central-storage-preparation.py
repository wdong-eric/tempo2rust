"""Native T1 fixed-storage/count and partial-reader witnesses for AJ planning."""
from pathlib import Path
import hashlib,json,subprocess,tempfile
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust')
PIN='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
REV='897041dd528cfb96db19dadb0844023e56814aa6'
OUT=Path(tempfile.mkdtemp(prefix='tempo2-predictor-storage-'))
Path('/tmp/tempo2-predictor-storage-current').write_text(str(OUT)+'\n')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def blob(name):return subprocess.check_output(['git','show',PIN+':'+name],cwd=ROOT/'tempo2')
def body(s,signature):
 start=s.index(signature);begin=s.index('{',start);stop=begin+1;depth=1
 while depth:
  depth+=(s[stop]=='{')-(s[stop]=='}');stop+=1
 return s[start:stop]+'\n'
sources={name:blob(name) for name in ['tempo2pred.h','t1polyco.c']}
for name,data in sources.items():(OUT/name).write_bytes(data)
functions=[body(sources['t1polyco.c'].decode(),sig) for sig in ['long double T1Polyco_GetPhase(', 'long double T1Polyco_GetFrequency(', 'int T1Polyco_Read_NewFormat(']]
program='''#include <assert.h>
#include <float.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tempo2pred.h"
'''+ '\n'.join(functions)+r'''
static void initial(T1Polyco *p, int count, long double c1, double dm) {
    memset(p, 0, sizeof(*p));
    strcpy(p->psrname, "INITIAL"); strcpy(p->date_string, "DATE");
    strcpy(p->utc_string, "UTC"); strcpy(p->sitename, "@");
    p->mjd_mid=58000; p->reference_phase=7; p->frequency_psr_0=100;
    p->frequency_obs=1400; p->span=60; p->ncoeff=count; p->dm=dm;
    for(int i=0;i<32;i++)p->coeff[i]=(i+1)/16.0L;
    p->coeff[0]=.25L; p->coeff[1]=c1; p->coeff[2]=.5L;
}
static void read_case(FILE *out, const char *name, T1Polyco *p, const char *text) {
    FILE *input=tmpfile(); assert(input);
    assert(fwrite(text,1,strlen(text),input)==strlen(text)); rewind(input);
    long double tail[29]; memcpy(tail,p->coeff+3,sizeof(tail));
    int status=T1Polyco_Read_NewFormat(p,input);
    long position=ftell(input); assert(position>=0);
    int eof=feof(input), error=ferror(input);
    fprintf(out,"READ\t%s\t%d\t%ld\t%zu\t%d\t%La\t%La\t%La\t%a\t%a\t%La\t%La\t%La\t%d\t%d\t%d\t%s\n",
        name,status,position,strlen(text),p->ncoeff,p->mjd_mid,
        p->reference_phase,p->frequency_psr_0,p->dm,p->doppler,
        p->coeff[0],p->coeff[1],p->coeff[2],eof,error,
        memcmp(tail,p->coeff+3,sizeof(tail))==0,p->psrname);
    /* The caller can still inspect the borrowed stream and owns its close. */
    fprintf(out,"TAIL\t%s\t",name);
    int c;while((c=fgetc(input))!=EOF)fprintf(out,"%02x",(unsigned)c);
    fprintf(out,"\n");assert(!ferror(input));assert(fclose(input)==0);
}
int main(int argc,char **argv) {
    assert(argc==2);FILE *out=fopen(argv[1],"wb");assert(out);
    fprintf(out,"TYPE\t%d\t%zu\t%zu\n",LDBL_MANT_DIG,sizeof(long double),sizeof(T1Polyco));
    int counts[]={0,1,2,3,32};long double coeff1[]={0,15,30};
    for(int n=0;n<5;n++)for(int c=0;c<3;c++)for(int d=0;d<2;d++)for(int offset=-1;offset<=1;offset++) {
        T1Polyco p;initial(&p,counts[n],coeff1[c],d?8:0);
        unsigned char before[sizeof(p)];memcpy(before,&p,sizeof(p));
        long double mjd=p.mjd_mid+offset/1024.0L;
        long double phase=T1Polyco_GetPhase(&p,mjd,700);
        long double frequency=T1Polyco_GetFrequency(&p,mjd,700);
        fprintf(out,"EVAL\t%d\t%d\t%d\t%d\t%La\t%La\t%d\n",counts[n],c,d,offset,
            phase,frequency,memcmp(before,&p,sizeof(p))==0);
    }
    /* Frequency is a separate source operation: these fields are not read. */
    for(int n=0;n<5;n++) {
        T1Polyco p;initial(&p,counts[n],15,8);
        long double mjd=p.mjd_mid+1/1024.0L;
        long double original=T1Polyco_GetFrequency(&p,mjd,700);
        p.coeff[0]=NAN; p.reference_phase=NAN;
        p.dm=NAN; p.doppler=NAN; p.frequency_obs=NAN;
        long double after=T1Polyco_GetFrequency(&p,mjd,NAN);
        fprintf(out,"LEAF\t%d\t%La\t%La\n",counts[n],original,after);
    }
    for(int count=0;count<3;count++)for(int seed=15;seed<=30;seed+=15) {
        T1Polyco p;initial(&p,7,seed,8);char text[512],name[64];
        snprintf(name,sizeof(name),"count-%d-seed-%d",count,seed);
        const char *coeff=count==0?"":count==1?"0.5\n":"0.5 60\n";
        snprintf(text,sizeof(text),"READ DATE UTC 58000 0 0 -6 7 100 @ 60 %d 1400 0 0\n%sTAIL\n",count,coeff);
        read_case(out,name,&p,text);
        fprintf(out,"READFREQ\t%s\t%La\n",name,T1Polyco_GetFrequency(&p,58000,700));
    }
    T1Polyco p;initial(&p,7,15,8);
    read_case(out,"header-failure",&p,"BROKEN DATE UTC 59000 16 bad\nTAIL\n");
    read_case(out,"coefficient-failure",&p,"READ DATE UTC 59000 16 2 -6 11 200 @ 90 3 1400 0 0\n0.5 30 bad\nTAIL\n");
    read_case(out,"recovery",&p,"RECOVER DATE UTC 58001 4 0 -4 9 50 @ 60 2 1000 0 0\n0.125 60\nTAIL\n");
    fprintf(out,"READFREQ\trecovery\t%La\n",T1Polyco_GetFrequency(&p,58001,700));
    assert(fclose(out)==0);return 0;
}
'''
(OUT/'probe.c').write_text(program)
command=['/opt/homebrew/bin/gcc-16','-std=c11','-O2',str(OUT/'probe.c'),'-lm','-o',str(OUT/'probe')]
r=subprocess.run(command,capture_output=True)
(OUT/'compile.stdout').write_bytes(r.stdout);(OUT/'compile.stderr').write_bytes(r.stderr)
assert r.returncode==0,r.stderr.decode()
run_command=[str(OUT/'probe'),str(OUT/'results.tsv')]
r=subprocess.run(run_command,capture_output=True,timeout=30)
(OUT/'stdout').write_bytes(r.stdout);(OUT/'stderr').write_bytes(r.stderr)
assert r.returncode==0 and not r.stderr
lines=(OUT/'results.tsv').read_text().splitlines()
rows=[s.split('\t') for s in lines]
assert rows[0][:3]==['TYPE','53','8'],rows[0]
evals={tuple(map(int,x[1:5])): {'phase':x[5],'frequency':x[6],'unchanged':x[7]} for x in rows if x[0]=='EVAL'}
assert len(evals)==90 and all(v['unchanged']=='1' for v in evals.values())
checks={}
for c in range(3):
 for d in range(2):
  for offset in [-1,0,1]:
   zero,one,two=[evals[(n,c,d,offset)] for n in [0,1,2]]
   checks[f'count-0-and-1-{c}-{d}-{offset}']=zero==one
   checks[f'frequency-reads-c1-{c}-{d}-{offset}']=all(float.fromhex(evals[(n,c,d,offset)]['frequency'])==100+[0,.25,.5][c] for n in [0,1,2])
   if offset==0 or c==0:checks[f'padding-neighbor-{c}-{d}-{offset}']=zero==two
   else:checks[f'forcing-count-2-changes-phase-{c}-{d}-{offset}']=zero['phase']!=two['phase']
leaves=[x for x in rows if x[0]=='LEAF']
assert len(leaves)==5
for x in leaves:checks['frequency-ignores-unused-fields-'+x[1]]=x[2]==x[3]
reads={x[1]:x[2:] for x in rows if x[0]=='READ'}
tails={x[1]:bytes.fromhex(x[2]) for x in rows if x[0]=='TAIL'}
freqs={x[1]:x[2] for x in rows if x[0]=='READFREQ'}
assert len(reads)==9 and len(tails)==9
for count in range(3):
 for seed in [15,30]:
  name=f'count-{count}-seed-{seed}';x=reads[name]
  checks[name+'-success-and-cursor']=x[0]=='0' and int(x[1])==int(x[2])-5 and tails[name]==b'TAIL\n'
  checks[name+'-count-and-storage']=(int(x[3])==count and float.fromhex(x[9])==(.25 if count==0 else .5)
       and float.fromhex(x[10])==(seed if count<2 else 60) and float.fromhex(x[11])==.5 and x[12:15]==['0','0','1'])
  checks[name+'-frequency']=float.fromhex(freqs[name])==100+(seed/60 if count<2 else 1)
h=reads['header-failure'];c=reads['coefficient-failure'];g=reads['recovery']
checks['header-failure-partial-fields']=(h[0]=='-1' and h[3]=='7' and h[15]=='BROKEN' and float.fromhex(h[4])==58000 and float.fromhex(h[7])==16 and tails['header-failure']==b'bad\nTAIL\n')
checks['coefficient-failure-partial-fields']=(c[0]=='-1' and c[3]=='3' and c[15]=='READ' and float.fromhex(c[4])==59000 and float.fromhex(c[5])==11 and float.fromhex(c[6])==200 and float.fromhex(c[9])==.5 and float.fromhex(c[10])==30 and float.fromhex(c[11])==.5 and tails['coefficient-failure']==b'bad\nTAIL\n')
checks['same-object-recovers']=(g[0]=='0' and g[3]=='2' and g[15]=='RECOVER' and float.fromhex(g[4])==58001 and float.fromhex(g[9])==.125 and float.fromhex(g[10])==60 and float.fromhex(freqs['recovery'])==51 and tails['recovery']==b'TAIL\n')
checks['read-leaves-unwritten-tail-intact']=all(x[12:15]==['0','0','1'] for x in reads.values())
checks['source-phase-diagnostics']=len(r.stdout.splitlines())==270 and r.stdout.count(b'WARNIGN WARNGING: Silly update in polyco read\n')==90
assert all(checks.values()),[k for k,v in checks.items() if not v]
integration={}
for name in ['rust/src/t1polyco.rs','rust/src/predictor_number.rs','rust/PREDICTOR_API.md','rust/PREDICTOR_SOURCE_MAP.md']:
 data=subprocess.check_output(['git','show',REV+':'+name],cwd=ROOT)
 dest=OUT/'integration'/name;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(data)
 integration[name]=hashlib.sha256(data).hexdigest()
receipt={'scope':'Three exact pinned native source bodies with fully initialized fixed T1 storage. Tests declared count independently of backing, source partial mutations and borrowed FILE cursor. No Rust ABI, production correction, uninitialized native state or Linux qualification.',
 'pin':PIN,'source_hashes':{n:sha(OUT/n) for n in sources},'extracted_body_hashes':[hashlib.sha256(s.encode()).hexdigest() for s in functions],
 'compile_command':command,'compile_exit':0,'run_command':run_command,'run_exit':0,'compiler':subprocess.check_output(['/opt/homebrew/bin/gcc-16','--version'],text=True),
 'program_sha256':sha(OUT/'probe.c'),'executable_sha256':sha(OUT/'probe'),'results_sha256':sha(OUT/'results.tsv'),'stdout_sha256':sha(OUT/'stdout'),'stderr_sha256':sha(OUT/'stderr'),
 'source_profile':rows[0],'evaluation_rows':90,'frequency_only_rows':5,'reader_rows':9,'checks':checks,'readers':reads,'tails_hex':{k:v.hex() for k,v in tails.items()},'integration_revision':REV,'integration_hashes':integration}
(OUT/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps({'out':str(OUT),'receipt_sha256':sha(OUT/'receipt.json'),'evaluation_rows':90,'frequency_only_rows':5,'reader_rows':9,'checks':len(checks),'passed':all(checks.values())},indent=2))
