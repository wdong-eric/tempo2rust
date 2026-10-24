"""Bounded source-expression O2 contraction witnesses; no production precision switch."""
import hashlib,json,os,random,struct,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent;OUT=HERE/'build/arithmetic';OUT.mkdir(parents=True,exist_ok=True)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
cc='/opt/homebrew/bin/g++-16'
source=r'''
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <cstdint>
// fda6ad4 ifteph.C:433,459,442/471,250 and tempo2Util.C::dotproduct.
__attribute__((noinline)) double recurrence(double a,double b,double c){return a*b-c;}
__attribute__((noinline)) double derivative(double a,double b,double c,double d){return a*b+c+c-d;}
__attribute__((noinline)) double sum(double a,double b,double c){return c+a*b;}
__attribute__((noinline)) double record(double a,double b,double c){return a+b*c;}
__attribute__((noinline)) double dot(double a,double b,double c,double d,double e,double f){return a*b+c*d+e*f;}
int main(int argc,char**argv){double x[6];for(int i=0;i<6;i++){uint64_t n=strtoull(argv[i+1],0,16);memcpy(x+i,&n,8);}double y[]={recurrence(x[0],x[1],x[2]),derivative(x[0],x[1],x[2],x[3]),sum(x[0],x[1],x[2]),record(x[0],x[1],x[2]),dot(x[0],x[1],x[2],x[3],x[4],x[5])};for(auto v:y){uint64_t n;memcpy(&n,&v,8);printf("%016llx\n",(unsigned long long)n);}}
'''
rust=r'''
fn main(){let x:Vec<f64>=std::env::args().skip(1).map(|s|f64::from_bits(u64::from_str_radix(&s,16).unwrap())).collect();let y=[x[0].mul_add(x[1],-x[2]),x[0].mul_add(x[1],x[2])+x[2]-x[3],x[0].mul_add(x[1],x[2]),x[1].mul_add(x[2],x[0]),x[4].mul_add(x[5],x[0].mul_add(x[1],x[2]*x[3]))];for v in y{println!("{:016x}",v.to_bits());}}
'''
(OUT/'witness.cc').write_text(source);(OUT/'witness.rs').write_text(rust)
commands=[]
for name,extra in [('o2',[]),('unfused',['-ffp-contract=off'])]:
 cmd=[cc,'-O2',*extra,str(OUT/'witness.cc'),'-o',str(OUT/name)];subprocess.run(cmd,check=True);commands.append(cmd)
 cmd=[cc,'-O2',*extra,'-S',str(OUT/'witness.cc'),'-o',str(OUT/(name+'.s'))];subprocess.run(cmd,check=True);commands.append(cmd)
cmd=['rustc','-O',str(OUT/'witness.rs'),'-o',str(OUT/'rust')];subprocess.run(cmd,check=True);commands.append(cmd)
rng=random.Random(58);witnesses={};checks=[]
for i in range(100):
 xs=[rng.uniform(-2,2) for _ in range(6)];args=[struct.pack('>d',v).hex() for v in xs]
 output={name:subprocess.check_output([str(OUT/name),*args],text=True).splitlines() for name in ['o2','unfused','rust']}
 assert output['o2']==output['rust'],(i,args,output)
 checks.append(dict(inputs=args,**output))
 for j,name in enumerate(['recurrence','derivative','sum','record','dot']):
  if output['o2'][j]!=output['unfused'][j]:witnesses.setdefault(name,i)
assert len(witnesses)==5,witnesses
(OUT/'receipt.json').write_text(json.dumps(dict(commands=commands,compiler_sha256=sha(Path(cc)),files={p.name:sha(p) for p in OUT.iterdir() if p.is_file() and p.name!='receipt.json'},witnesses=witnesses,checks=checks),indent=2)+'\n')
print('PASS 100 expression inputs; five actual contraction witnesses; native O2 equals explicit Rust mul_add')
