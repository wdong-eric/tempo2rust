"""Typed source transfer: arithmetic boundaries are emitted explicitly and reviewed.
This is development tooling; cargo uses committed Rust, not this generator.
"""
from translate_base import *
import json, difflib
T.update({'bool':'bool','unsigned int':'u32','uint64_t':'u64','unsigned long long':'u64','param_label':'i32','FILE':'File','observation':'SourceObservation','long double':'f64'})
def clean(s):
 s=re.sub(r'/\*.*?\*/|//[^\n]*',lambda m:'\n'*m[0].count('\n'),s,flags=re.S)
 s=re.sub(r'^[ \t]*#.*','',s,flags=re.M)
 s=re.sub(r'\blongdouble\s*\(', 'ldconst(',s)
 s=s.replace('const pulsar &psr','const pulsar *psr')
 s=s.replace('.c_str()','')
 return s
class Output(Translator):
 def __init__(self,source,functions):
  super().__init__(source,functions)
  self.fields={}
  self.defs={}
  for what in ['pulsar','observation']:
   txt=(ROOT/f'rust/src/{what}.rs').read_text()
   self.defs[what]=dict(re.findall(r'pub (\w+): ([^\n]+),',txt))
   self.fields[what]={k:(re.search(r'Option<(\w+)>',t).group(1) if re.search(r'Option<(\w+)>',t) else t) for k,t in re.findall(r'pub (\w+): ([^\n]+),',txt)}
 def pexpr(self,s):return self.parseexpr(s)
 def lval(self,n):
  def chain(n):
   if isinstance(n,c_ast.StructRef):return chain(n.name)+[('field',n.field.name)]
   if isinstance(n,c_ast.ArrayRef):return chain(n.name)+[('index',n.subscript)]
   if isinstance(n,c_ast.ID):return [('root',n.name)]
   return []
  path=chain(n)
  if path and path[0][1]=='psr' and len(path)>1:
   rest=path[1:];pi='0'
   if rest[0][0]=='index':pi=self.expr(rest.pop(0)[1])[0]
   owner=f'context.pulsars[({pi}) as usize]' if self.name=='textOutput' else 'psr'
   f=rest.pop(0)[1]
   if f=='param':
    l=self.expr(rest.pop(0)[1])[0];f=rest.pop(0)[1];slot=f'{owner}.param.slots[({l}) as usize]'
    if f in ['aSize','nLinkTo','nLinkFrom']:return slot+'.'+{'aSize':'a_size','nLinkTo':'n_link_to','nLinkFrom':'n_link_from'}[f],'i32',True
    k=self.expr(rest.pop(0)[1])[0]
    if f=='linkTo':return slot+f'.link_to[({k}) as usize]','i32',True
    f={'paramSet':'param_set','fitFlag':'fit_flag','prefitErr':'prefit_err'}.get(f,f)
    return slot+f'.cells[({k}) as usize].{f}',('bytes' if f in ['label','shortlabel'] else ('i32' if f in ['fit_flag','param_set'] else 'Float')),f in ['prefit','prefit_err']
   what='pulsar'
   if f=='obsn':
    i=self.expr(rest.pop(0)[1])[0];owner=(f'output_rows(context, ({pi}) as usize)?' if self.name=='textOutput' else 'rows')+f'[({i}) as usize]';f=rest.pop(0)[1];what='observation'
   while rest and rest[0][0]=='field':owner+='.'+f;f=rest.pop(0)[1]
   out=owner+'.'+f
   for kind,k in rest:out+=f'[({self.expr(k)[0]}) as usize]'
   t=self.fields[what][f]
   if t=='u8' and self.defs[what][f].count('[')>len(rest):t='string'
   return out,t,True
  if isinstance(n,c_ast.ID) and self.env.get(n.name)=='bytes':return n.name,'bytes',False
  if isinstance(n,c_ast.ArrayRef) and isinstance(n.name,c_ast.ID) and self.env.get(n.name.name)=='bytes':return n.name.name+'[('+self.expr(n.subscript)[0]+') as usize]','u8',False
  return super().lval(n)
 def expr(self,n,raw=False):
  s=CG.visit(n)
  if isinstance(n,c_ast.ID):
   constants={'MAX_PARAMS':'crate::tempo2::MAX_PARAMS as i32','START_FINISH_DELTA':'Float::with_val(PRECISION_BITS, Float::parse("1e-11").unwrap())','GM':'1.3271243999e20f64','BIG_G':'6.673e-11f64','PCM':'3.08568025e16f64','MASYR2RADS':'1.53628185004416045e-16f64','SI_UNITS':'1i32','TDB_UNITS':'2i32','FB90_TIMEEPH':'2i32','IF99_TIMEEPH':'1i32','T2C_IAU2000B':'1i32','REFPHS_TZR':'1i32','series_simple_pn':'1i32','series_taylor_pn':'2i32','T2_TNSUBPOLY_FLAG_NESTLIKE':'2u64','true':'true','false':'false'}
   if n.name in constants:
    x=constants[n.name];return x,('Float' if n.name=='START_FINISH_DELTA' else 'bool' if n.name in ['true','false'] else 'u64' if n.name.startswith('T2_TNSUB') else 'f64' if n.name in ['GM','BIG_G','PCM','MASYR2RADS'] else 'i32')
   if n.name.startswith('constraint_'):return f'(crate::pulsar::Constraint::{n.name} as i32)','i32'
   if n.name=='npsr':return 'context.pulsars.len() as i32','i32'
   if n.name=='fname':return 'fname.to_vec()','bytes'
   if self.env.get(n.name)=='bytes':return f'{n.name}.clone()','bytes'
   if self.env.get(n.name)=='file':return f'{n.name}.is_some()','bool'
  if isinstance(n,c_ast.Constant) and n.type=='char':return str(ord(n.value[1:-1]))+'i32','i32'
  if s.startswith('label_str['):return f'crate::core_output::LABEL_STR[({self.expr(n.subscript)[0]}) as usize].as_bytes().to_vec()','bytes'
  if isinstance(n,c_ast.ArrayRef) and s.startswith('psr[') and '.covar[' in s:
   m=re.match(r'psr\[(.*?)\]\.covar\[(.*?)\]\[(.*?)\]',s);return f'output_covar(context, ({self.parseexpr(m[1])}) as usize, ({self.parseexpr(m[2])}) as usize, ({self.parseexpr(m[3])}) as usize)?','f64'
  if isinstance(n,c_ast.ArrayRef) and s.startswith('psr[') and '.constraint_special[' in s:
   m=re.match(r'psr\[(.*?)\]\.constraint_special\[(.*?)\]',s);return f'output_constraint(context, ({self.parseexpr(m[1])}) as usize, ({self.parseexpr(m[2])}) as usize)?','bytes'
  if isinstance(n,(c_ast.StructRef,c_ast.ArrayRef)):
   try:x,t,o=self.lval(n)
   except:pass
   else:
    if t=='string':return f'read_string(&{x}[..], "{s}")?','bytes'
    if t=='u8':return (f'(*read(&{x}, "{s}")?) as i32' if o else f'{x} as i32'),'i32'
    if t=='bytes':return f'crate::clock_text::c_bytes(&{x}).to_vec()','bytes'
    if t=='SeriesType':return f'(*read(&{x}, "{s}")?) as i32','i32'
    if t=='Constraint':return f'(*read(&{x}, "{s}")?) as i32','i32'
  if isinstance(n,c_ast.TernaryOp):
   a,at=self.expr(n.iftrue);b,bt=self.expr(n.iffalse)
   if at==bt=='&str':c,ct=self.expr(n.cond);return f'if {cast(c,ct,"bool")} {{ {a} }} else {{ {b} }}','&str'
  if isinstance(n,c_ast.BinaryOp) and n.op=='&':
   a,at=self.expr(n.left);b,bt=self.expr(n.right)
   if 'u64' in [at,bt]:return f'({cast(a,at,"u64")} & {cast(b,bt,"u64")})','u64'
  if isinstance(n,c_ast.FuncCall):
   name=n.name.name;aa=n.args.exprs if n.args else []
   if name=='strlen':return f'({self.expr(aa[0])[0]}).len() as i32','i32'
   if name in ['strcmp','strcasecmp']:
    x,xt=self.expr(aa[0]);y,yt=self.expr(aa[1]);x=f'{x}.as_bytes()' if xt=='&str' else x;y=f'{y}.as_bytes()' if yt=='&str' else y
    return (f'i32::from(!({x}).eq_ignore_ascii_case(&{y}))' if name=='strcasecmp' else f'i32::from({x} != {y})'),'i32'
   if name=='strstr':
    x,xt=self.expr(aa[0]);y,yt=self.expr(aa[1]);return f'({x}).windows(({y}).len()).any(|w| w == ({y}).as_bytes())','bool'
   if name=='getParameterValue':
    owner=f'&context.pulsars[({self.expr(aa[0].expr.subscript)[0]}) as usize]' if self.name=='textOutput' else 'psr'
    return f'crate::tempo2_util::get_parameter_value_inner({owner}, ({self.expr(aa[1])[0]}) as usize, ({self.expr(aa[2])[0]}) as usize, d)?','Float'
   if name.startswith('t2Decode'):return f'crate::t2fit_nestlike::{name}({self.expr(aa[0])[0]})',('u64' if name.endswith('Flags') else 'i32')
   if name=='get_constraint_name':return f'crate::constraints::name(crate::pulsar::Constraint::from_index(({self.expr(aa[0])[0]}) as i32).ok_or_else(|| boundary("output_constraint_index"))?).as_bytes().to_vec()','bytes'
   if name=='t2Fit_getParamMatrixRow':return f'crate::general_fitting::get_param_matrix_row(&context.pulsars[p as usize].fitinfo,p,crate::tempo2::PARAMETER_LABELS[({self.expr(aa[2])[0]}) as usize],{self.expr(aa[3])[0]}).map(|i| i as i32).unwrap_or(-1)','i32'
   if name=='logerr_check':return 'd.check()','i32'
   if name=='fortran_nint':return f'source_nint({self.expr(aa[0])[0]})?','i32'
   if name=='fopen':
    x,t=self.expr(aa[0]);x=f'{x}.as_bytes().to_vec()' if t=='&str' else x
    return f'io.create(&{x})','file'
   if name=='isfinite':return f'({self.expr(aa[0])[0]}).is_finite()','bool'
   if name=='expm1':return f'({self.expr(aa[0])[0]}).exp_m1()','f64'
   if name=='fmax':return f'({self.expr(aa[0])[0]}).max({self.expr(aa[1])[0]})','f64'
   if name=='m2':return 'm2_inner('+', '.join(cast(*self.expr(a),'Float') for a in aa)+', io)?','f64'
   if name=='dglep':return f'{{ let value=dglep_inner(psr, {self.expr(aa[1])[0]}, {self.expr(aa[2])[0]}, d)?; io.drain(d)?; value }}','f64'
  return super().expr(n,raw)
 def formatted(self,args):
  fmt=args[0].value;vals=[]
  for a in args[1:]:
   x,t=self.expr(a);vals.append(f'Argument::from({x})')
  return f'crate::source_format::format({fmt}, &[{", ".join(vals)}])?'
 def stmt(self,n):
  if isinstance(n,c_ast.If) and any(CG.visit(n.cond).startswith('!psr'+prefix) for prefix in ['[p].param[param_tzrmjd]', '[p].param[param_tzrfrq]', '[p].param[param_tres]']):
   return '// fda6ad4 textOutput.C:127-133,1926 activates a report field without prefit.\n// Snapshot its defined previous value/error before activation (0053 poison witness).\n'+super().stmt(n)
  if isinstance(n,c_ast.Decl):
   if n.name=='CVS_verNum':return ''
   tp=typ(n)
   if tp=='&File':
    self.env[n.name]='file';return f'let mut {n.name} = '+(self.expr(n.init)[0] if n.init else 'None::<std::fs::File>')+';\n'
   if isinstance(tp,tuple):
    if tp[1]=='u8':self.env[n.name]='bytes';return f'let mut {n.name}:Vec<u8> = Vec::new();\n'
    if tp[1]=='Float':self.env[n.name]=tp;return f'let mut {n.name}: [Float; {tp[2]}] = std::array::from_fn(|_| Float::with_val(PRECISION_BITS, 0));\n'
  if isinstance(n,c_ast.ArrayRef) and isinstance(n.name,c_ast.ID) and self.env.get(n.name.name)=='bytes':return super().stmt(n)
  if isinstance(n,c_ast.Switch):
   c,ct=self.expr(n.cond);cases=n.stmt.block_items;return 'if '+c+' == 1 {\n'+''.join(self.stmt(x) for x in cases[0].stmts if not isinstance(x,c_ast.Break))+'} else {\n'+''.join(self.stmt(x) for case in cases[1:] for x in (case.stmts or []) if not isinstance(x,c_ast.Break))+'}\n'
  if isinstance(n,c_ast.Assignment) and isinstance(n.rvalue,c_ast.FuncCall) and n.rvalue.name.name=='fopen':return f'{CG.visit(n.lvalue)} = {self.expr(n.rvalue)[0]};\n'
  if isinstance(n,c_ast.FuncCall):
   name=n.name.name;aa=n.args.exprs if n.args else []
   if name in ['printf','ld_printf']:return f'{{ let bytes = {self.formatted(aa)}; io.write(&bytes)?; }}\n'
   if name in ['logdbg','logwarn','logerr','logmsg']:return f'd.log(b\'{dict(logdbg="d",logwarn="w",logerr="e",logmsg="m")[name]}\',"{self.src}",{self.source_lines.get(n.coord.line-1,n.coord.line-1)},&{self.formatted(aa)});\nio.drain(d)?;\n'
   if name in ['fprintf','ld_fprintf']:
    f=CG.visit(aa[0]);return (f'io.write(&{self.formatted(aa[1:])})?;\n' if f=='stdout' else f'write_file(&mut {f}, &{self.formatted(aa[1:])})?;\n')
   if name in ['sprintf','snprintf']:
    target=CG.visit(aa[0]);args=aa[2:] if name=='snprintf' else aa[1:];return f'{target} = {self.formatted(args)};\n'
   if name in ['strcpy','strncpy']:
    x,t,o=self.lval(aa[0]);v,vt=self.expr(aa[1]);v=f'{v}.as_bytes().to_vec()' if vt=='&str' else v
    if t=='bytes':return f'{x} = {v};\n'
    return f'{{ let bytes = {v}; crate::observation::write_string(&mut {x}[..], &bytes, "output_string")?; }}\n'
   if name=='fclose':return f'{CG.visit(aa[0])}.take();\n'
   if name=='scanf':return 'fname2 = io.input_token()?;\n'
   if name=='sscanf':return 'let tokens = crate::clock_text::tokens(&'+self.expr(aa[0])[0]+').map(|s|s.to_vec()).collect::<Vec<_>>();\n'+''.join(f'if let Some(v)=tokens.get({i}){{ {CG.visit(a)}=v.clone(); }}\n' for i,a in enumerate(aa[2:]))
   if name=='calcRMS':return 'calc_rms_inner(context, p as usize, d)?;\nio.drain(d)?;\n'
   if name=='printGlitch':return 'print_glitch_inner(&context.pulsars[p as usize], io, d)?;\n'
  return super().stmt(n)
 def func(self,n):
  self.name=n.decl.name;self.ret,params=self.functions[self.name];self.env={};self.optional=set();self.loopnext=[]
  def walk(x):
   if isinstance(x,c_ast.UnaryOp) and x.op=='&' and isinstance(x.expr,c_ast.ID):self.optional.add(x.expr.name)
   for _,v in x.children():walk(v)
  walk(n.body)
  # Source-witnessed unassigned branches must remain recoverable, not invented values.
  self.optional.update({'textOutput':['closestV','start','end','m2'],'dglep':['f0','df0','fleft','dfleft','fright','dfright','dph','ddph_dt','fnext','dfnext']}.get(self.name,[]))
  self.optional.discard('psr')
  for nm,tp in params:self.env[nm]=tp
  if self.name=='textOutput':self.env['fname']='bytes'
  body=self.stmt(n.body)
  if self.name in ['calcRMS','dglep']:body=body.replace('io.drain(d)?;','')
  signatures={
   'textOutput':'pub(crate) fn text_output_inner(context: &mut PulsarContext, globalParameter:f64,nGlobal:i32,outRes:i32,newpar:i32,fname:&[u8],io:&mut OutputIo<\'_>,d:&mut Diagnostics)->Result<(),TimingError>',
   'calcRMS':'pub(crate) fn calc_rms_values(psr:&mut SourcePulsar,rows:&[SourceObservation],d:&mut Diagnostics)->Result<f64,TimingError>',
   'm2':'pub(crate) fn m2_inner(mf:Float,sini:Float,m1:Float,io:&mut OutputIo<\'_>)->Result<f64,TimingError>',
   'printGlitch':'pub(crate) fn print_glitch_inner(psr:&SourcePulsar,io:&mut OutputIo<\'_>,d:&mut Diagnostics)->Result<(),TimingError>',
   'dglep_eval_phase':'fn dglep_eval_phase(t:f64,fph:f64,glf0:f64,glf0d:f64,glf1:f64,tds:f64,dph:&mut Option<f64>,ddph_dt:&mut Option<f64>,d:&mut Diagnostics)->Result<i32,TimingError>',
   'dglep':'pub(crate) fn dglep_inner(psr:&SourcePulsar,gn:i32,fph:f64,d:&mut Diagnostics)->Result<f64,TimingError>'}
  if self.ret=='()':body=body[:-2]+'Ok(())\n}\n'
  return f'/// {self.src}:{self.source_lines.get(n.decl.coord.line-1,n.decl.coord.line-1)}::{self.name}.\n'+signatures[self.name]+body
from native_build import output_corrected
original=(ROOT/'tempo2/textOutput.C').read_text()
corrected=output_corrected(original)
source_lines={}
for block in difflib.SequenceMatcher(a=original.splitlines(),b=corrected.splitlines(),autojunk=False).get_matching_blocks():
 for i in range(block.size):source_lines[block.b+i+1]=block.a+i+1
src=clean(corrected)
src=re.sub(r'\bfn\b','mass_function',src)
# Defined owner correction, with original native witness in check_helpers.py.
src=src.replace('psr->param[param_start]','psr[p].param[param_start]').replace('psr->param[param_finish]','psr[p].param[param_finish]')
src=src.replace('observation *o = psr[p].obsn+i;','')
src=src.replace('o->','psr[p].obsn[i].')
# C++ string search and assignment-in-condition rewrite, no branch change.
src=re.sub(r'strstr\((.*?)\)!=NULL',r'strstr(\1)',src)
src=src.replace('if (!(fout = fopen("aplus_t2.dat","w")))','fout = fopen("aplus_t2.dat","w"); if (!fout)').replace('if (!(fout = fopen("across_t2.dat","w")))','fout = fopen("across_t2.dat","w"); if (!fout)')
src=src.replace('if (!(fout2 = fopen(fname2,"w")))','fout2 = fopen(fname2,"w"); if (!fout2)')
ast=c_parser.CParser().parse('typedef double longdouble; typedef struct pulsar pulsar; typedef struct observation observation; typedef int bool; typedef unsigned long long uint64_t; typedef int param_label; typedef struct FILE FILE;\n'+src)
functions={n.decl.name:(typ(n.decl.type.type),[(a.name,typ(a)) for a in n.decl.type.args.params]) for n in ast.ext if isinstance(n,c_ast.FuncDef)}
tr=Output('textOutput.C',functions)
tr.source_lines=source_lines
header='''//! textOutput.C full report/PAR/helper transfer at fda6ad4; GPL-3.0-or-later.
//! Lint allowances preserve source grouping, constants, branches and first-knot loop.
//! Explicit source-order arithmetic and canonical state; no native ABI layout.
#![allow(non_snake_case,unused_mut,unused_variables,unused_assignments,unused_parens,unreachable_code,clippy::assign_op_pattern,clippy::double_parens,clippy::needless_borrow,clippy::unnecessary_to_owned,clippy::excessive_precision,clippy::needless_return,clippy::too_many_arguments,clippy::get_first,clippy::unnecessary_cast,clippy::if_same_then_else,clippy::never_loop)]
use super::*;
use crate::tempo2::Label::*;
use rug::ops::Pow;
'''
code=header+'\n'.join(tr.func(n) for n in ast.ext if isinstance(n,c_ast.FuncDef))
code=code.replace('chisqr = ', '// fda6ad4 textOutput.C:150,1441: unweighted PAR read had no producer; use fitChisq.\nchisqr = ',1)
(ROOT/'rust/src/core_output_source.rs').write_text(code)
