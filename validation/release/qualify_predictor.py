#!/usr/bin/env python3
"""Fresh standalone native and private-prefix C11/C++ qualification; no oracle writes."""
from pathlib import Path
import argparse, difflib, hashlib, json, os, platform, shutil, subprocess, sys, math
from predictor_compare import compare_values, compare_storage, mutation_controls
ROOT=Path(__file__).resolve().parents[2]; HERE=ROOT/'validation/library_interfaces'
PIN='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
p=argparse.ArgumentParser();p.add_argument('--prefix',type=Path,required=True);p.add_argument('--out',type=Path,required=True);p.add_argument('--native-repository',type=Path,required=True);a=p.parse_args()
prefix=a.prefix.resolve();out=a.out.resolve();out.mkdir(parents=True,exist_ok=False)
commands=[]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def run(cmd,stem,cwd=None,expected=0):
 cmd=list(map(str,cmd));env=os.environ.copy();env.pop('TEMPO2',None);env.pop('TEMPO2_CLOCK_PATH',None);r=subprocess.run(cmd,cwd=cwd,capture_output=True,env=env);commands.append({'command':cmd,'cwd':str(cwd),'status':r.returncode})
 (out/(stem+'.stdout')).write_bytes(r.stdout);(out/(stem+'.stderr')).write_bytes(r.stderr)
 assert r.returncode==expected,(stem,r.returncode,r.stderr.decode(errors='replace'))
 return r
src=out/'native';src.mkdir();sources={};patch=''
for name in ['tempo2pred.h','tempo2pred.c','cheby2d.c','t1polyco.c','TKlongdouble.float128.h']:
 original=subprocess.check_output(['git','-C',a.native_repository.resolve(),'show',PIN+':'+name]).decode();corrected=original
 if name=='cheby2d.c':
  corrected=corrected.replace('ld_fprintf(', 'fprintf(').replace('ld_printf(', 'printf(')
  corrected=corrected.replace('ChebyModel *from)', 'const ChebyModel *from)').replace('Cheby2D_Evaluate(Cheby2D *cheby','Cheby2D_Evaluate(const Cheby2D *cheby')
 if name=='tempo2pred.c':corrected=corrected.replace('    T2Predictor pred;','    T2Predictor pred; T2Predictor_Init(&pred);')
 (src/name).write_text(corrected);sources[name]={'raw':hashlib.sha256(original.encode()).hexdigest(),'executed':sha(src/name)}
 patch+=''.join(difflib.unified_diff(original.splitlines(True),corrected.splitlines(True),fromfile='raw/'+name,tofile='executed/'+name))
# Correct declaration-only mismatches; no core or native timing library is linked.
shutil.copy2(ROOT/'rust/include/tempo2pred_int.h',src/'tempo2pred_int.h')
(out/'native-corrections.patch').write_text(patch)
cc=os.environ.get('NATIVE_CC','gcc');cxx=os.environ.get('NATIVE_CXX','g++')
darwin=platform.system()=='Darwin';suffix='.dylib' if darwin else '.so'
# C clients call libm directly; ELF does not inherit a DSO's dependencies.
client_libs=['-lm']+([] if darwin else ['-ldl'])
shared=prefix/('lib/libtempo2pred_rust'+suffix)
native_library=src/('libnative'+suffix)
run([cc,'-std=c11','-O2',ROOT/'validation/release/platform_semantics.c',*client_libs,'-o',out/'platform-semantics'],'platform-semantics-build')
run([out/'platform-semantics'],'platform-semantics')
objects=[]
for name in ['tempo2pred.c','cheby2d.c','t1polyco.c']:
 obj=src/(name+'.o');run([cc,'-std=c11','-D_GNU_SOURCE','-O2','-fPIC','-include','stdio.h','-include',src/'TKlongdouble.float128.h','-I'+str(src),'-c',src/name,'-o',obj],name);objects.append(obj)
run([cc,'-dynamiclib' if darwin else '-shared',*objects,'-lquadmath','-lm','-o',native_library],'native-link')
run([cc,'-std=c11','-D_GNU_SOURCE','-O2','-DNATIVE','-I'+str(src),HERE/'consumer.c','-L'+str(src),'-lnative','-Wl,-rpath,'+str(src),*client_libs,'-o',out/'native-client'],'native-client-build')
run([cc,'-std=c11','-D_GNU_SOURCE','-O2','-I'+str(src),HERE/'low_count.c','-L'+str(src),'-lnative','-Wl,-rpath,'+str(src),*client_libs,'-o',out/'native-low-count'],'native-low-count-build')
native_low=run([out/'native-low-count'],'native-low-count')
run([cc,'-std=c11','-D_GNU_SOURCE','-O2','-I'+str(src),HERE/'boundary.c','-L'+str(src),'-lnative','-Wl,-rpath,'+str(src),*client_libs,'-o',out/'native-boundary'],'native-boundary-build')
native_boundary=run([out/'native-boundary'],'native-boundary')
run([cc,'-std=c11','-D_GNU_SOURCE','-O2','-I'+str(src),HERE/'storage.c','-L'+str(src),'-lnative','-Wl,-rpath,'+str(src),*client_libs,'-o',out/'native-storage'],'native-storage-build')
work=out/'native-run';work.mkdir();reference=run([out/'native-client'],'native-client',cwd=work)
ref_storage=run([out/'native-storage',out/'native-storage.tsv'],'native-storage',cwd=work)
controls=mutation_controls(reference.stdout.splitlines(),(out/'native-storage.tsv').read_text().splitlines())
layout=ROOT/'decisions/evidence/predictor-interface/layout.c'
run([cc,'-std=c11','-I'+str(src),layout,'-o',out/'native-layout'],'native-layout-build')
ref_layout=run([out/'native-layout'],'native-layout')
# Static requirements are rustc's actual profile-specific native requirements.
install=json.loads((prefix.parent/(prefix.name+'-build')/'install.json').read_text())
results=[]
for language,compiler,standard in [('c',cc,'c11'),('cpp',cxx,'c++17')]:
 for linkage in ['shared','static']:
  tag=language+'-'+linkage
  link=['-L'+str(prefix/'lib'),'-ltempo2pred_rust','-Wl,-rpath,'+str(prefix/'lib')] if linkage=='shared' else [prefix/'lib/libtempo2pred_rust.a',*install['native_static_libs']]
  link+=client_libs
  exe=out/tag
  run([compiler,'-x','c' if language=='c' else 'c++','-std='+standard,'-D_GNU_SOURCE','-O2','-I'+str(prefix/'include'),'-c',HERE/'consumer.c','-o',out/(tag+'.o')],tag+'-compile')
  run([compiler,out/(tag+'.o'),*link,'-o',exe],tag+'-link')
  work=out/(tag+'-run');work.mkdir();r=run([exe],tag,cwd=work)
  # The only extra output is the deliberately corrected alias regression.
  actual=[line for line in r.stdout.splitlines() if not line.startswith(b'V self-copy ')]
  expected=reference.stdout.splitlines()
  compare_values(actual,expected)
  loaded=r.stderr.decode().split('LOADED ',1)[1].splitlines()[0]
  assert Path(loaded).resolve()==(shared if linkage=='shared' else exe).resolve(),loaded
  for mode,status in [('unsupported-copy',255),('unsupported-insert',255),('unsupported-keep',255),('write-failure',1)]:
   fail=run([exe,mode],tag+'-'+mode,cwd=work,expected=status)
   native=run([out/'native-client',mode],'native-'+tag+'-'+mode,cwd=work,expected=status)
   assert fail.stderr==native.stderr and fail.stdout==native.stdout
  run([compiler,'-x','c' if language=='c' else 'c++','-std='+standard,'-D_GNU_SOURCE','-O2','-I'+str(prefix/'include'),'-c',HERE/'low_count.c','-o',out/(tag+'-low-count.o')],tag+'-low-count-compile')
  run([compiler,out/(tag+'-low-count.o'),*link,'-o',out/(tag+'-low-count')],tag+'-low-count-link')
  low=run([out/(tag+'-low-count')],tag+'-low-count');assert low.stdout==native_low.stdout and low.stderr==native_low.stderr
  run([compiler,'-x','c' if language=='c' else 'c++','-std='+standard,'-D_GNU_SOURCE','-O2','-I'+str(prefix/'include'),'-c',HERE/'boundary.c','-o',out/(tag+'-boundary.o')],tag+'-boundary-compile')
  run([compiler,out/(tag+'-boundary.o'),*link,'-o',out/(tag+'-boundary')],tag+'-boundary-link')
  boundary=run([out/(tag+'-boundary')],tag+'-boundary');assert boundary.stdout==native_boundary.stdout and boundary.stderr==native_boundary.stderr
  run([compiler,'-x','c' if language=='c' else 'c++','-std='+standard,'-D_GNU_SOURCE','-O2','-I'+str(prefix/'include'),'-c',HERE/'storage.c','-o',out/(tag+'-storage.o')],tag+'-storage-compile')
  run([compiler,out/(tag+'-storage.o'),*link,'-o',out/(tag+'-storage')],tag+'-storage-link')
  storage=run([out/(tag+'-storage'),out/(tag+'-storage.tsv')],tag+'-storage',cwd=work)
  sr=(out/(tag+'-storage.tsv')).read_text().splitlines(); nr=(out/'native-storage.tsv').read_text().splitlines()
  compare_storage(sr,nr)
  assert storage.stdout==ref_storage.stdout and storage.stderr==ref_storage.stderr
  run([compiler,'-x','c' if language=='c' else 'c++','-std='+standard,'-I'+str(prefix/'include'),layout,'-o',out/(tag+'-layout')],tag+'-layout-build')
  measured=run([out/(tag+'-layout')],tag+'-layout');assert measured.stdout==ref_layout.stdout
  # Each side reads the other side's actual file; serialization is exact on
  # identical input backing even when constructed coefficients differ in rounding.
  for owner in ['native-run',tag+'-run']:
   for filename in ['cheby.dat','t1.dat']:
    source=out/owner/filename
    native=run([out/'native-client','exchange',source],tag+'-'+owner+'-'+filename+'-native-exchange',cwd=work)
    native_bytes=(work/'exchanged.dat').read_bytes()
    foreign=run([exe,'exchange',source],tag+'-'+owner+'-'+filename+'-rust-exchange',cwd=work)
    assert native.stdout==foreign.stdout and native_bytes==(work/'exchanged.dat').read_bytes()
  compiled=run([exe,'compiled-test'],tag+'-compiled-test',cwd=work)
  assert b'RMS=' in compiled.stdout and not compiled.stderr
  results.append({'profile':tag,'layout_rows':len(measured.stdout.splitlines()),'value_state_rows':len(actual),'storage_rows':len(sr),'phase_diagnostic_lines':len(storage.stdout.splitlines()),'low_count_rows':len(low.stdout.splitlines()),'boundary_rows':len(boundary.stdout.splitlines()),'loaded':loaded})
# Source client with only the 0084 approved initialization/destruction fixes.
getperiod=subprocess.check_output(['git','-C',a.native_repository.resolve(),'show',PIN+':getPeriod.C']).decode()
getperiod=getperiod.replace('    T2Predictor pred;','    T2Predictor pred; T2Predictor_Init(&pred);').replace('    return 0;','    T2Predictor_Destroy(&pred); return 0;')
(src/'getPeriod.C').write_text(getperiod)
run([cxx,'-I'+str(src),src/'getPeriod.C','-L'+str(src),'-lnative','-Wl,-rpath,'+str(src),*client_libs,'-o',out/'native-getPeriod'],'native-getPeriod-build')
# Installed getPeriod uses its real shared dependency; preserve source atof syntax.
for filename in ['cheby.dat','t1.dat']:
 for mjd in ['58000.001','58000.001junk','nonnumeric']:
  r=run([prefix/'bin/getPeriod-rust',out/'native-run'/filename,mjd,'extra'],filename+'-'+mjd)
  native=run([out/'native-getPeriod',out/'native-run'/filename,mjd,'extra'],'native-'+filename+'-'+mjd)
  assert len(r.stdout.splitlines())==1 and r.stdout==native.stdout
run([prefix/'bin/getPeriod-rust'],'client-missing-args',expected=1)
run([prefix/'bin/getPeriod-rust',out/'missing','58000'],'client-missing-file',expected=1)
invalid=out/'invalid.dat';invalid.write_text('invalid predictor\n')
run([prefix/'bin/getPeriod-rust',invalid,'58000'],'client-malformed',expected=1)
run([prefix/'bin/getPeriod-rust',out/'native-run/t1.dat','58000'],'client-recovery')
for count in [0,1]:
 low=out/('low-count-'+str(count)+'.dat');low.write_text('TEMPO2:\nREAD DATE UTC 58000 0 0 -6 7 100 @ 60 '+str(count)+' 1400 0 0\n'+('0.5\n' if count else ''))
 run([prefix/'bin/getPeriod-rust',low,'58000'],'client-low-count-'+str(count),expected=1)
run((['otool','-L',shared] if darwin else ['ldd',shared]),'shared-dependencies')
run((['otool','-L',prefix/'bin/getPeriod-rust'] if darwin else ['ldd',prefix/'bin/getPeriod-rust']),'client-dependencies')
# Apple's LLVM21 nm cannot decode LLVM22 bitcode in Rust standard-library
# archive members. Inspect the real native ABI member and execute static clients.
member=subprocess.check_output(['ar','p',prefix/'lib/libtempo2pred_rust.a','predictor.o'])
(out/'predictor-archive.o').write_bytes(member)
run(['ar','t',prefix/'lib/libtempo2pred_rust.a'],'archive-members')
static_symbols=run(['nm',*(['-gU'] if darwin else ['-g','--defined-only']),out/'predictor-archive.o'],'static-symbols')
shared_symbols=run(['nm',*(['-gU'] if darwin else ['-g','--defined-only']),shared],'shared-symbols')
native_symbols=run(['nm',*(['-gU'] if darwin else ['-g','--defined-only']),native_library],'native-symbols')
def public_symbols(data):
 return {line.split()[-1].decode().lstrip('_') for line in data.splitlines() if line.split() and line.split()[-1].decode().lstrip('_').startswith(('T2Predictor_','T1Polyco','T1P_','Cheby','testFunc','testCheby2D','parse_ld_cheby','tempo2_verbose'))}
assert public_symbols(static_symbols.stdout)==public_symbols(shared_symbols.stdout)==public_symbols(native_symbols.stdout)

for name in ['LICENSE','MT19937_LICENSE.txt']:
 assert (prefix/'share/tempo2-rust/rust'/name).read_bytes()==(ROOT/'rust'/name).read_bytes()
receipt={'pin':PIN,'platform':platform.platform(),'long_double_profile':ref_layout.stdout.decode().splitlines()[0],'gate_epsilon':'2**-52 unchanged AF','source':sources,'profiles':results,'commands':commands,
 'rejected_controls':controls,
 'observer_sources':{str(p):sha(p) for p in [HERE/'consumer.c',HERE/'storage.c',HERE/'low_count.c',HERE/'boundary.c',Path(__file__).with_name('predictor_compare.py'),ROOT/'validation/release/platform_semantics.c',layout]},
 'symbols':sorted(public_symbols(shared_symbols.stdout)),
 'prefix_artifacts':{str(p):sha(p) for p in [prefix/'lib/libtempo2pred_rust.a',shared,prefix/'bin/getPeriod-rust',prefix/'include/tempo2pred.h',prefix/'include/tempo2pred_int.h']},
 'evidence':{str(p.relative_to(out)):sha(p) for p in sorted(out.rglob('*')) if p.is_file()},'script':sha(Path(__file__))}
(out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');print(json.dumps(results,indent=2))
