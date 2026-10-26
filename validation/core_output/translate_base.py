# Authored translation tooling adapted from AD; no production build dependency.
import sys,re,pathlib
from pycparser import c_parser,c_ast,c_generator
ROOT=pathlib.Path(__file__).resolve().parents[2]
CG=c_generator.CGenerator()
T={'double':'f64','float':'f32','int':'i32','long':'i64','long int':'i64','long long':'i64','longdouble':'Float','void':'()','char':'u8','pulsar':'SourcePulsar'}
def typ(n):
 if isinstance(n,c_ast.Decl): return typ(n.type)
 if isinstance(n,(c_ast.TypeDecl,c_ast.Typename)): return typ(n.type)
 if isinstance(n,c_ast.IdentifierType):return T[' '.join(n.names)]
 if isinstance(n,c_ast.PtrDecl): return '&'+typ(n.type)
 if isinstance(n,c_ast.ArrayDecl):return ('array',typ(n.type),CG.visit(n.dim))
 raise Exception(type(n))
def cast(s,a,b):
 if a==b:return s
 if b=='Float':return f'Float::with_val(PRECISION_BITS, {s})'
 if a=='Float':
  if b=='i64':return f'crate::form_residuals::pulse_integer(&({s})).ok_or_else(|| boundary("source_pulse_integer"))?'
  if b=='f64':return f'({s}).to_f64()'
  if b=='f32':return f'({s}).to_f32()'
  if b=='i32':return f'crate::ifteph::extended_i32(&({s}), "binary_integer")?'
 if b=='bool':return f'({s} != '+('0.0' if a.startswith('f') else '0')+')'
 if a=='i64' and b=='i32':return f'i32::try_from({s}).map_err(|_| boundary("source_integer_narrowing"))?'
 if a=='bool':return f'({s}) as {b}'
 if a=='f64' and b=='i32':return f'source_i32({s})?'
 return f'({s}) as {b}'
def common(a,b):
 for t in ['Float','f64','f32','u64','i64','u32','i32','u8']:
  if t in [a,b]:return t
 raise Exception((a,b))
class Translator:
 def __init__(self,src,functions):self.src=src;self.functions=functions;self.env={};self.ret='()';self.loopnext=[]
 def lval(self,n):
  s=CG.visit(n)
  m=re.fullmatch(r'psr(?:\[p\]|\[0\])?\.?->?param.*',s)
  # canonical parameter arrays, accessed by source-indexed labels
  m=re.fullmatch(r'psr(?:\[p\]|\[0\])?(?:\.|->)param\[(.*?)\]\.(val|err|paramSet|fitFlag)\[(.*?)\]',s)
  if m:
   label,field,index=m.groups();l=self.expr(c_parser.CParser().parse('void x(){'+label+';}').ext[0].body.block_items[0])[0];i=self.expr(c_parser.CParser().parse('void x(){'+index+';}').ext[0].body.block_items[0])[0]
   field={'paramSet':'param_set','fitFlag':'fit_flag'}.get(field,field)
   return f'psr.param.slots[({l}) as usize].cells[({i}) as usize].{field}', 'Float' if field in ['val','err'] else 'i32',False
  m=re.fullmatch(r'psr(?:\[p\]|\[0\])?(?:\.|->)param\[(.*?)\]\.(aSize|nLinkTo|nLinkFrom)',s)
  if m:
   l=self.parseexpr(m[1]);return f'psr.param.slots[({l}) as usize].'+{'aSize':'a_size','nLinkTo':'n_link_to','nLinkFrom':'n_link_from'}[m[2]],'i32',True
  m=re.fullmatch(r'psr(?:\[p\]|\[0\])?(?:\.|->)obsn\[(.*?)\]\.([A-Za-z0-9_]+)(?:\[(.*?)\])?',s)
  if m:
   i,f,k=m.groups();ty='Float' if f in ['bbat','bat','sat','pet','residual'] else 'f64'
   return f'rows[({self.parseexpr(i)}) as usize].{f}'+(f'[({self.parseexpr(k)}) as usize]' if k else ''),ty,True
  m=re.fullmatch(r'psr(?:\[p\]|\[0\])?(?:\.|->)(\w+)(?:\[(.*?)\])?',s)
  if m:
   f,k=m.groups();ty='i32' if f in ['noWarnings','orbifuncN','ifuncN','nCompanion','eclCoord'] else 'f64'
   return f'psr.{f}'+(f'[({self.parseexpr(k)}) as usize]' if k else ''),ty,True
  if isinstance(n,c_ast.ID):return n.name,self.env[n.name],n.name in self.optional
  if isinstance(n,c_ast.UnaryOp) and n.op=='*':
   s,t=self.expr(n.expr,raw=True);return f'*{s}',t[1:],True
  if isinstance(n,c_ast.ArrayRef):
   s,t=self.expr(n.name,raw=True);i,_=self.expr(n.subscript);return f'{s}[({i}) as usize]',t[1],False
  raise Exception(('lval',s))
 def parseexpr(self,s):return self.expr(c_parser.CParser().parse('void x(){'+s+';}').ext[0].body.block_items[0])[0]
 def expr(self,n,raw=False):
  if isinstance(n,c_ast.Constant):
   if n.type=='string':return n.value,'&str'
   t='f64' if ('double' in n.type or n.type=='float') else 'i32';return re.sub(r'\.(?=[eEf]|$)', '.0', n.value.rstrip('LlUu'))+('f64' if t=='f64' else 'i32'),t
  if isinstance(n,c_ast.ID):
   if n.name.startswith('param_'):return '('+n.name+' as i32)','i32'
   if n.name in {'M_PI','SECDAY','AULTSC','SPEED_LIGHT','SOLAR_MASS'}:
    return {'M_PI':'std::f64::consts::PI','SECDAY':'86400.0f64','AULTSC':'499.00478364f64','SPEED_LIGHT':'299792458.0f64','SOLAR_MASS':'1.98892e30f64'}[n.name],'f64'
   if n.name in ['MAX_COMPANIONS','MAX_BPJ_JUMPS']:return f'(crate::tempo2::{n.name} as i32)','i32'
   if n.name=='displayCVSversion':return '(d.display_version as i32)','i32'
   if n.name=='TSUN':return 'Float::with_val(PRECISION_BITS, 4.925490947e-6f64)','Float'
   if n.name=='LD_PI':return 'Float::with_val(PRECISION_BITS, rug::float::Constant::Pi)','Float'
   s,t,o=self.lval(n)
  elif isinstance(n,(c_ast.ArrayRef,c_ast.StructRef)) or (isinstance(n,c_ast.UnaryOp) and n.op=='*'):
   s,t,o=self.lval(n)
  else:s=t=o=None
  if s is not None:
   if s=='psr.name':return 'String::from_utf8_lossy(&read_string(&psr.name[..], "name")?).into_owned()', '&str'
   if o:s=f'read(&{s}, "{CG.visit(n)}")?'
   if not raw and t=='Float':s=f'({s}).clone()'
   elif o:s=f'*{s}'
   return s,t
  if isinstance(n,c_ast.Cast):
   s,t=self.expr(n.expr);b=typ(n.to_type);return cast(s,t,b),b
  if isinstance(n,c_ast.UnaryOp):
   if n.op=='&':
    s,t,_=self.lval(n.expr);return '&mut '+s,'&'+t
   s,t=self.expr(n.expr)
   if n.op=='+':return s,t
   if n.op=='!':return '!('+cast(s,t,'bool')+')','bool'
   return '('+n.op+s+')',t
  if isinstance(n,c_ast.BinaryOp):
   a,at=self.expr(n.left);b,bt=self.expr(n.right);op=n.op
   if op in ['&&','||']:return f'({cast(a,at,"bool")} {op} {cast(b,bt,"bool")})','bool'
   t=common(at,bt);a=cast(a,at,t);b=cast(b,bt,t)
   return f'({a} {op} {b})','bool' if op in ['<','<=','>','>=','==','!='] else t
  if isinstance(n,c_ast.TernaryOp):
   a,at=self.expr(n.iftrue);b,bt=self.expr(n.iffalse);t=common(at,bt);c,ct=self.expr(n.cond)
   return f'if {cast(c,ct,"bool")} {{ {cast(a,at,t)} }} else {{ {cast(b,bt,t)} }}',t
  if isinstance(n,c_ast.FuncCall):
   name=n.name.name;args=n.args.exprs if n.args else []
   if name=='ldconst':return f'Float::with_val(PRECISION_BITS, Float::parse("{CG.visit(args[0])}").unwrap())','Float'
   if name=='ifunc':
    count=self.expr(args[3])[0];t=cast(*self.expr(args[2]),'f64');a=CG.visit(args[0]).split('.')[-1];b=CG.visit(args[1]).split('.')[-1]
    return f'crate::ifunc::interpolate(&psr.{a}.iter().take(({count}) as usize).map(|v| read(v, "{a}").copied()).collect::<Result<Vec<_>, _>>()?, &psr.{b}.iter().take(({count}) as usize).map(|v| read(v, "{b}").copied()).collect::<Result<Vec<_>, _>>()?, {t}, {count}, d)?','f64'
   if name=='getParameterValue':return f'crate::tempo2_util::get_parameter_value_inner(psr, ({self.expr(args[1])[0]}) as usize, ({self.expr(args[2])[0]}) as usize, d)?','Float'
   maths={'sin':'sin','cos':'cos','tan':'tan','asin':'asin','acos':'acos','atan':'atan','sqrt':'sqrt','fabs':'abs','exp':'exp','log':'ln','log10':'log10','floor':'floor','atan2':'atan2','pow':'pow'}
   base=name[:-1] if name.endswith('l') else name
   if base in maths:
    ss=[self.expr(a) for a in args];t=ss[0][1]
    if name in ['sinl','cosl','floorl','fabsl','powl']:t='Float'
    elif name.endswith('l'):t='f64'
    elif t not in ['Float','f64','f32']:t='f64'
    a=cast(*ss[0],t);method=maths[base]
    if len(ss)==2:
     b=cast(*ss[1],t)
     if t=='Float':return f'({a}).{method}(&({b}))',t
     return f'({a}).'+('powf' if base=='pow' else method)+f'({b})',t
    return f'({a}).{method}()',t
   if name in self.functions:
    ret,params=self.functions[name];out=[]
    for a,(nm,tp) in zip(args,params):
     if tp=='&SourcePulsar':out+=['psr','rows'];continue
     if nm=='p' and name.endswith('model'):continue
     ss,tt=self.expr(a);out.append(cast(ss,tt,tp))
    out.append('d');return f'{name}('+', '.join(out)+')?',ret
   raise Exception(('call',name))
  raise Exception(('expr',type(n),CG.visit(n)))
 def stmt(self,n):
  if n is None:return ''
  if isinstance(n,c_ast.Compound):
   scope=self.env.copy();body=''.join(self.stmt(x) for x in n.block_items or []);self.env=scope
   return '{\n'+body+'}\n'
  if isinstance(n,c_ast.DeclList):return ''.join(self.stmt(x) for x in n.decls)
  if isinstance(n,c_ast.Decl):
   if n.name=='CVS_verNum':return ''
   t=typ(n);self.env[n.name]=t
   if isinstance(t,tuple):return f'let mut {n.name}: [{t[1]}; {t[2]}] = [0.0; {t[2]}];\n'
   if n.name in self.optional:
    v=cast(*self.expr(n.init),t) if n.init else None
    return f'let mut {n.name}: Option<{t}> = '+(f'Some({v})' if v else 'None')+';\n'
   if n.init:
    s,a=self.expr(n.init);return f'let mut {n.name}: {t} = {cast(s,a,t)};\n'
   return f'let mut {n.name}: {t};\n'
  if isinstance(n,c_ast.Assignment):
   if self.name=='DDKmodel' and CG.visit(n.lvalue)=='dtdpx':return '// DDKmodel.C:267 reads dtdpx before initialization; it has no subsequent use.\n// 0053: omit this dead invalid calculation; no parallax value is invented.\n'
   s,t,o=self.lval(n.lvalue);v,a=self.expr(n.rvalue)
   if n.op!='=':
    old,ot=self.expr(n.lvalue);ct=common(t,a);v=f'({cast(old,ot,ct)} {n.op[0]} {cast(v,a,ct)})';a=ct
   v=cast(v,a,t)
   return f'{s} = '+(f'Some({v})' if o else v)+';\n'
  if isinstance(n,c_ast.If):
   if isinstance(n.iftrue,c_ast.FuncCall) and n.iftrue.name.name=='CVSdisplayVersion':return f'd.version("{self.src}", "{self.name}()");\n'
   s,t=self.expr(n.cond);return f'if {cast(s,t,"bool")} {{\n{self.stmt(n.iftrue)}}}'+(f' else {{\n{self.stmt(n.iffalse)}}}' if n.iffalse else '')+'\n'
  if isinstance(n,c_ast.For):
   init=self.stmt(n.init);cond,ct=self.expr(n.cond);nxt=self.stmt(n.next);self.loopnext.append(nxt);body=self.stmt(n.stmt);self.loopnext.pop()
   return '{\n'+init+f'while {cast(cond,ct,"bool")} {{\n'+body+nxt+'}\n}\n'
  if isinstance(n,c_ast.While):
   s,t=self.expr(n.cond);return f'while {cast(s,t,"bool")} '+self.stmt(n.stmt)
  if isinstance(n,c_ast.DoWhile):
   s,t=self.expr(n.cond);return 'loop {\n'+self.stmt(n.stmt)+f'if !({cast(s,t,"bool")}) {{ break; }}\n}}\n'
  if isinstance(n,c_ast.Return):
   s,t=self.expr(n.expr) if n.expr else ('()','()');return f'return Ok({cast(s,t,self.ret)});\n'
  if isinstance(n,c_ast.UnaryOp) and n.op in ['p++','++','p--','--']:
   s,t,o=self.lval(n.expr);v=f'({self.expr(n.expr)[0]} '+('+' if '+' in n.op else '-')+' '+('1.0' if t in ['f64','f32'] else '1')+')'
   return f'{s} = '+(f'Some({v})' if o else v)+';\n'
  if isinstance(n,c_ast.FuncCall):
   name=n.name.name;args=n.args.exprs if n.args else []
   if name=='exit':return f'return Err(boundary("{self.src}::{self.name}: source exit"));\n'
   if name=='CVSdisplayVersion':return f'd.version("{self.src}", "{self.name}()");\n'
   if name in ['printf','ld_printf','logerr','logwarn','logmsg']:
    fmt=args[0].value
    vals=[]
    for a in args[1:]:
     x,t=self.expr(a);vals.append(f'String::from_utf8_lossy(&{x})' if t=='bytes' else x)
    # diagnostic formatting is refined against the native message comparisons
    fm=re.sub(r'%[-+0-9.*]*[LQhl]*[a-zA-Z]','{}',fmt)
    body=f'format!({fm}'+(' ,'+','.join(vals) if vals else '')+').as_bytes()'
    if name in ['printf','ld_printf']:return f'd.stdout({body});\n'
    return f'd.log(b\'{"e" if name=="logerr" else ("w" if name=="logwarn" else "m")}\', "{self.src}", {n.coord.line-1}, {body});\n'
   if name=='displayMsg':
    es=[self.expr(a)[0] for a in args];return f'd.display({es[0]}, {es[1]}.as_bytes(), {es[2]}.as_bytes(), {es[3]}.as_bytes(), {es[4]})?;\n'
   s,t=self.expr(n);return s+';\n'
  if isinstance(n,c_ast.Break):return 'break;\n'
  if isinstance(n,c_ast.Continue):return (self.loopnext[-1] if self.loopnext else '')+'continue;\n'
  if isinstance(n,c_ast.EmptyStatement):return ''
  raise Exception(('stmt',type(n),CG.visit(n)))
 def func(self,n):
  self.name=n.decl.name;self.ret,params=self.functions[self.name];self.env={};outs=[];self.optional=set()
  def walk(x):
   if isinstance(x,c_ast.UnaryOp) and x.op=='&' and isinstance(x.expr,c_ast.ID):self.optional.add(x.expr.name)
   for _,c in x.children():walk(c)
  walk(n.body)
  for nm,tp in params:
   if tp=='&SourcePulsar':outs+=['psr: &mut SourcePulsar','rows: &[SourceObservation]'];continue
   if nm=='p' and self.name.endswith('model'):continue
   self.env[nm]=tp;outs.append(f'mut {nm}: '+('&mut Option<'+tp[1:]+'>' if tp.startswith('&') else tp))
  outs+=['d: &mut Diagnostics'];body=self.stmt(n.body)
  if self.ret=='()':body=body[:-2]+'Ok(())\n}\n'
  return f'/// {self.src}::{self.name} (pinned source line {n.decl.coord.line-1}).\npub(super) fn {self.name}('+', '.join(outs)+f') -> Result<{self.ret}, TimingError> '+body
