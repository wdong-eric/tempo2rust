"""Generate explicit test-only field visitors from the pre-edit frozen map.
Production storage is independent and reviewed against the raw pinned header.
"""
import json
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1]
fields=json.loads((HERE/'fields.json').read_text())['fields']
rust='''// Generated test visitor for tempo2.h::observation; frozen source field order.
use crate::{tempo2::{SourceObservation, SourceClockCorrection}, PRECISION_BITS};
use rug::Float;
use serde_json::{json, Value};
fn long(seed:i32,id:usize,index:usize) -> Float {
    if seed==0 { return Float::with_val(128,0); }
    let mut v=Float::with_val(128,seed+id as i32*100+index as i32);
    v+=0.125;
    v+=Float::with_val(128,1)>>100;
    v
}
fn double(seed:i32,id:usize,index:usize)->f64 { if seed==0 {0.0} else { (seed+id as i32*100+index as i32) as f64 + 0.125 } }
fn integer(seed:i32,id:usize,index:usize)->i32 { if seed==0 {0} else { -(seed+id as i32*100+index as i32) } }
fn byte(seed:i32,id:usize,index:usize)->u8 { if seed==0 || index%32==5 {0} else {((seed as usize+id+index)%251) as u8} }
fn correction(seed:i32,id:usize,index:usize)->SourceClockCorrection {
    SourceClockCorrection { correction:Some(double(seed,id,index)), corrects_to:std::array::from_fn(|j|Some(byte(seed,id,index*32+j))) }
}
pub(super) fn seeded(seed:i32)->SourceObservation {
    SourceObservation {
'''
cpp='''// R supplemental native caller: unchanged pinned observation declaration and
// C++ whole-record assignment as tempo2Util.C:293. No unwritten memory is read.
#include "tempo2.h"
#include <cassert>
#include <cstdint>
#include <climits>
#include <type_traits>
#include <fstream>
#include <string>
#include <cstring>
static std::string q(const std::string&s){return "\\\""+s+"\\\"";}
static std::string hex(const void*p,size_t n){static const char*d="0123456789abcdef";std::string s;for(size_t i=0;i<n;i++){unsigned char c=((const unsigned char*)p)[i];s+=d[c>>4];s+=d[c&15];}return s;}
static std::string num(longdouble v){
 if(isnanq(v))return "{\\\"class\\\":\\\"NaN\\\"}";
 if(isinfq(v))return std::string("{\\\"class\\\":")+q(signbitq(v)?"-Infinity":"+Infinity")+"}";
 return "{\\\"class\\\":\\\"finite\\\",\\\"bytes_le\\\":"+q(hex(&v,sizeof v))+",\\\"bits\\\":128}";
}
static std::string num(double v){
 if(__builtin_isnan(v))return "{\\\"class\\\":\\\"NaN\\\"}";
 if(__builtin_isinf(v))return std::string("{\\\"class\\\":")+q(__builtin_signbit(v)?"-Infinity":"+Infinity")+"}";
 return "{\\\"class\\\":\\\"finite\\\",\\\"bytes_le\\\":"+q(hex(&v,sizeof v))+",\\\"bits\\\":64}";
}
static longdouble ld(int seed,int id,int index){return seed+id*100+index+0.125Q+ldexpq(1.0Q,-100);}
static double d(int seed,int id,int index){return seed+id*100+index+0.125;}
static int integer(int seed,int id,int index){return -(seed+id*100+index);}
static char byte(int seed,int id,int index){return index%32==5?0:(unsigned char)((seed+id+index)%251);}
static clock_correction correction(int seed,int id,int index){clock_correction c;c.correction=d(seed,id,index);for(int j=0;j<32;j++)c.corrects_to[j]=byte(seed,id,index*32+j);return c;}
static observation seeded(int seed){observation o;
'''

def expr(f,idx,seed='seed',lang='r'):
 id=fields.index(f)+1;t=f['source_type']
 fn={'longdouble':'long' if lang=='r' else 'ld','double':'double' if lang=='r' else 'd','int':'integer','long long':'integer','char':'byte','clock_correction':'correction'}[t]
 e=f'{fn}({seed},{id},{idx})'
 if lang=='r':
  if t=='long long':e+=' as i64'
  if t!='clock_correction':e='Some('+e+')'
 return e
for f in fields:
 name=f['name'];dims=f['extents'];idx='0' if not dims else 'i' if len(dims)==1 else f'i*{dims[1]}+j'
 e=expr(f,idx)
 if len(dims)==2:e='std::array::from_fn(|j|'+e+')'
 if dims:e='std::array::from_fn(|i|'+e+')'
 rust+=f'        {name}: {e},\n'
 loc='o.'+name;loop=''
 for k,dim in enumerate(dims):
  var='ij'[k];loop+=f'for(int {var}=0;{var}<{dim};{var}++)';loc+=f'[{var}]'
 cpp+=f' {loop}{loc}={expr(f,idx,lang="c")};\n'
rust+='    }\n}\n'
cpp+=' return o;\n}\n'
# Recursive test-only serialization, preserving scalar class/initialization.
rust+='''trait Cell { fn cell(&self)->Value; }
fn number(v:&Float)->Value {
    if v.is_nan(){json!({"class":"NaN"})}
    else if v.is_infinite(){json!({"class":if v.is_sign_negative(){"-Infinity"}else{"+Infinity"}})}
    else {let (n,e)=v.to_integer_exp().unwrap();json!({"class":"finite","integer":n.to_string(),"exponent":e,"negative":v.is_sign_negative()})}
}
impl Cell for Float {fn cell(&self)->Value {assert_eq!(self.prec(),PRECISION_BITS);number(self)}}
impl Cell for f64 {fn cell(&self)->Value {number(&Float::with_val(128,*self))}}
impl Cell for i32 {fn cell(&self)->Value {json!(self)}}
impl Cell for i64 {fn cell(&self)->Value {json!(self.to_string())}}
impl Cell for u8 {fn cell(&self)->Value {json!(self)}}
impl<T:Cell> Cell for Option<T> {fn cell(&self)->Value {self.as_ref().map(Cell::cell).unwrap_or(Value::Null)}}
impl<T:Cell,const N:usize> Cell for [T;N] {fn cell(&self)->Value {json!(self.iter().map(Cell::cell).collect::<Vec<_>>())}}
impl Cell for SourceClockCorrection {fn cell(&self)->Value {json!({"correction":self.correction.cell(),"corrects_to":self.corrects_to.cell()})}}
pub(super) fn state(o:&SourceObservation)->Value {
    let mut fields=Vec::new();
'''
cpp+='''template<class T> static std::string cell(T v){return std::to_string(v);}
static std::string cell(long long v){return q(std::to_string(v));}
static std::string cell(char v){return std::to_string((unsigned char)v);}
static std::string cell(double v){return num(v);}
static std::string cell(longdouble v){return num(v);}
static std::string cell(const clock_correction&c){std::string s="{\\\"correction\\\":"+num(c.correction)+",\\\"corrects_to\\\":[";for(int j=0;j<32;j++){if(j)s+=',';s+=cell(c.corrects_to[j]);}return s+"]}";}
template<class T,size_t N> static std::string array(const T(&v)[N]){std::string s="[";for(size_t i=0;i<N;i++){if(i)s+=',';s+=cell(v[i]);}return s+"]";}
static std::string state(const observation&o){std::string s="[";
'''
for id,f in enumerate(fields):
 name=f['name'];dims=f['extents'];ty=f['source_type'];rust+=f'    fields.push(json!({{"name":"{name}","type":"{ty}","dimensions":{dims},"value":o.{name}.cell()}}));\n'
 if len(dims)==0:e=f'cell(o.{name})'
 elif len(dims)==1:e=f'array(o.{name})'
 else:
  cpp+=f' std::string {name}="[";for(int i=0;i<{dims[0]};i++){{if(i){name}+=\',\';{name}+=array(o.{name}[i]);}}{name}+=\']\';\n';e=name
 header=json.dumps(dict(name=name,type=ty,dimensions=dims),separators=(',',':'))[:-1]+',"value":'
 cpp+=(' s+=\',\';\n' if id else '')+f' s+={json.dumps(header)}+{e}+"}}";\n'
rust+='    json!(fields)\n}\n';cpp+=' return s+"]";\n}\n'
# ABI measured from the actual C++ field declarations; signedness is separate.
cpp+='static std::string abi(){std::string s="[";\n'
for id,f in enumerate(fields):
 name=f['name'];dims=f['extents'];t=f['source_type'];idx='[0]'*len(dims)
 hdr=json.dumps(dict(name=name,type=t),separators=(',',':'))[:-1]+',"dimensions":['
 cpp+=(' s+=\',\';\n' if id else '')+f' s+={json.dumps(hdr)};\n'
 for j in range(len(dims)):
  if j:cpp+=" s+=',';\n"
  loc='((observation*)0)->'+name+'[0]'*j
  cpp+=f' s+=std::to_string(sizeof({loc})/sizeof({loc}[0]));\n'
 cpp+=f' s+="],\\\"cell_bytes\\\":"+std::to_string(sizeof(((observation*)0)->{name}{idx}))+"}}";\n'
cpp+=' return s+"]";\n}\n'
cpp+='''static void edges(observation&o,int which){
 o.sat=-0.0Q;o.origsat=HUGE_VALQ;o.sat_day=-HUGE_VALQ;o.sat_sec=nanq("");
 o.addedNoise=-0.0;o.TNRedSignal=__builtin_inf();o.TNRedErr=-__builtin_inf();o.TNDMSignal=__builtin_nan("");
 o.clockCorr=INT_MIN;o.delayCorr=INT_MAX;o.deleted=-1;o.nclock_correction=-7;o.nFlags=INT_MAX;o.obsNjump=INT_MIN;o.obsNfdjump=INT_MAX;
 o.jump[39]=INT_MIN;o.fdjump[39]=INT_MAX;o.pulseN=which==1?LLONG_MIN:LLONG_MAX;
 o.correctionsTT[29].correction=-0.0;
}
int main(int argc,char**argv){
 assert(argc==3);assert(sizeof(int)==4 && sizeof(long long)==8 && sizeof(double)==8 && sizeof(longdouble)==16);
 assert(std::is_signed<int>::value && std::is_signed<long long>::value);
 std::ofstream out(argv[1]);assert(out);out<<"{\\\"token\\\":"<<q(argv[2])<<",\\\"abi\\\":"<<abi()<<",\\\"signed_types\\\":{\\\"int\\\":true,\\\"long long\\\":true},\\\"cases\\\":[";
 for(int k=0;k<3;k++){
  observation src=seeded(10),dst=seeded(70);if(k)edges(src,k);
  auto sb=state(src),db=state(dst);dst=src;
  auto da=state(dst),sa=state(src);dst.fname[500]='!';dst.sat=2.0Q;
  if(k)out<<',';out<<"{\\\"case\\\":"<<k<<",\\\"source_before\\\":"<<sb<<",\\\"destination_before\\\":"<<db<<",\\\"source_after\\\":"<<sa<<",\\\"destination_after\\\":"<<da<<",\\\"independent\\\":"<<(state(src)==sb?"true":"false")<<"}";
 }
 out<<"]}";
}
'''
(ROOT/'rust/src/observation_probe_fields.rs').write_text(rust)
(HERE/'probe.cc').write_text(cpp)
