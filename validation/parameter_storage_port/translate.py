#!/usr/bin/env python3
"""Bounded source transcription for O; production has no source/JSON dependency.
Re-running writes only the enum prefix and initializer; hand-maintained storage
and copy logic are not generated. The pin and complete grammar are asserted.
"""
from pathlib import Path
import re, subprocess
ROOT=Path(__file__).resolve().parents[2]
PIN='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
def blob(name):return subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',PIN+':'+name],text=True)
def uncomment(s):return re.sub(r'/\*.*?\*/|//[^\n]*','',s,flags=re.S)
h=uncomment(blob('tempo2.h'))
names=[s.strip() for s in h.split('enum label {')[1].split('};')[0].split(',') if s.strip()]
assert len(names)==166
text='''//! TEMPO2 tempo2.h::label, param_label and parameter (not C ABI layout).
//! Source array-valued storage; lib.rs::Parameter is a derived scalar output view.
#![allow(non_camel_case_types)]
use crate::PRECISION_BITS;
use rug::Float;

pub type param_label = i32;

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Label {
'''+''.join(f'    {n} = {i},\n' for i,n in enumerate(names))+'}\n\npub const PARAMETER_NAMES: [&str; 166] = [\n'+''.join(f'    "{n}",\n' for n in names)+'];\npub const PARAMETER_LABELS: [Label; 166] = [\n'+''.join(f'    Label::{n},\n' for n in names)+'];\n'
p=ROOT/'rust/src/tempo2.rs'
if p.exists():text+='// Owned source storage.\n'+p.read_text().split('// Owned source storage.\n',1)[1]
p.write_text(text)
s=uncomment(blob('initialise.C').split('/* Spin-frequency parameters */')[1].split('for (k=0; k < MAX_PARAMS; ++k)')[0])
s=s.replace('psr->dm_series_type=series_undefined;','')
s=re.sub(r'for\s*\(\s*([jk])\s*=\s*(\d+);\s*\1\s*<\s*psr->param\[(\w+)\].aSize;\s*\1\+\+\)',r'for \1 in \2..psr.slots[\3 as usize].cells.len()',s)
s=s.replace('if (k>0)','if k > 0')
s=re.sub(r'psr->param\[(\w+)\].val\[([jk0-9]+)\] = 0.0;',r'psr.slots[\1 as usize].cells[\2].val = Float::with_val(PRECISION_BITS, 0);',s)
def fmt(m):
 dest,form,args=m.groups();parts=args.split(',') if args else []
 form=form.replace('%04d','{:04}').replace('%d','{}')
 value='format!("'+form+'"'+(' ,'+','.join(parts) if parts else '')+')'
 if dest.strip()=='temp':return 'temp = '+value+';'
 n,f,i=re.fullmatch(r'psr->param\[(\w+)\].(label|shortlabel)\[(\w+)\]',dest.strip()).groups()
 return f'write_label(&mut psr.slots[{n} as usize].cells[{i}].{f}, {value}.as_bytes());'
s=re.sub(r'sprintf\(\s*(.*?),\s*"([^"]*)"\s*(?:,\s*([^;]*?))?\);',fmt,s)
def cp(m):
 n,f,i,value=m.groups()
 b='temp.as_bytes()' if value.strip()=='temp' else 'b'+value.strip()
 return f'write_label(&mut psr.slots[{n} as usize].cells[{i}].{f}, {b});'
s=re.sub(r'strcpy\(\s*psr->param\[(\w+)\].(label|shortlabel)\[(\w+)\],\s*(temp|"[^"]*")\s*\);',cp,s)
assert not any(x in s for x in ['psr->','sprintf','strcpy',';k',';j']),s
# The duplicate daop assignment is source behavior, including retained bytes after NUL.
s=s.replace('write_label(&mut psr.slots[param_daop as usize].cells[0].label, b"IPERHARM");','// Source defect: D_AOP is overwritten; param_iperharm is also labeled later.\n    write_label(&mut psr.slots[param_daop as usize].cells[0].label, b"IPERHARM");')
out='''//! Parameter-only initialise.C::allocateMemory / initialiseOne / destroyMemory.
//! All other pulsar/observation/covariance initialization is deferred.
// GPL-3.0-or-later; TEMPO2 contributors. Source pin in O's projection.
use crate::tempo2::{Label::*, ParameterStore, SourceParameter, MAX_PARAMS, MAX_DM_DERIVATIVES, MAX_FREQ_DERIVATIVES, MAX_COMPANIONS, MAX_BPJ_JUMPS, MAX_DMX, MAX_SX, MAX_TEL_DX, MAX_TEL_DY, MAX_TEL_DZ};
use crate::PRECISION_BITS;
use rug::Float;

/// realloc is unused by the source; Rust replacement drops the previous owner.
pub(crate) fn allocate_memory(_realloc: i32) -> ParameterStore {
    let slots = (0..MAX_PARAMS).map(|i| {
        let capacity = match i as i32 {
'''
# Extract exactly the allocation selection chain, retaining source ordering/grouping.
a=uncomment(blob('initialise.C').split('void allocateMemory')[1]).split('if (i==param_dm)')[1].split('psr->param[i].val')[0]
a='if (i==param_dm)'+a
pat=r'(?:else\s+)?if\s*\((.*?)\)\s*psr->param\[i\].aSize\s*=\s*(\w+)\s*;'
constants={n:int(v) for n,v in re.findall(r'#define\s+(MAX_\w+)\s+(\d+)\b',h)}
for m in re.finditer(pat,a,re.S):
 ns=re.findall(r'i\s*==\s*(\w+)',m[1]); assert ns
 cap=m[2]
 out+='            x if '+ ' || '.join('x == '+n+' as i32' for n in ns)+' => '+str(cap)+',\n'
rest=re.sub(pat,'',a,flags=re.S).strip();assert rest=='else psr->param[i].aSize = 1;',rest
out+='''            _ => 1,
        };
        SourceParameter::allocated(capacity, 100)
    }).collect();
    ParameterStore { slots }
}

// Known source literals/formats fit the allocated buffers; retain bytes after NUL.
fn write_label(destination: &mut [u8], value: &[u8]) {
    destination[..value.len()].copy_from_slice(value);
    destination[value.len()] = 0;
}

pub(crate) fn initialise_parameters() -> ParameterStore {
    let mut psr = allocate_memory(0);
    let mut temp: String;
'''+s+'\n    psr\n}\n'
(ROOT/'rust/src/initialise.rs').write_text(out)
