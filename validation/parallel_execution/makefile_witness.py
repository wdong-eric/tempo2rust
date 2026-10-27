"""Execute original/corrected Automake compile rules in fresh pinned source copies."""
from pathlib import Path
import difflib, hashlib, json, os, subprocess, sys
ROOT=Path(__file__).resolve().parents[2]
PIN='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
OUT=Path(sys.argv[1]).resolve();OUT.mkdir(parents=True,exist_ok=False)
TOOLS=ROOT/'validation/parallel_execution/build/tools/prefix/bin'
env=dict(os.environ,PATH=str(TOOLS)+':'+os.environ['PATH'],CC='/opt/homebrew/bin/gcc-16',CXX='/opt/homebrew/bin/g++-16',FC='/opt/homebrew/bin/gfortran-16',F77='/opt/homebrew/bin/gfortran-16')
records=[]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def run(label,cmd,cwd):
 r=subprocess.run(list(map(str,cmd)),cwd=cwd,env=env,capture_output=True,timeout=300)
 (OUT/(label+'.stdout')).write_bytes(r.stdout);(OUT/(label+'.stderr')).write_bytes(r.stderr)
 records.append(dict(label=label,command=list(map(str,cmd)),cwd=str(cwd),exit=r.returncode))
 (OUT/'commands.json').write_text(json.dumps(records,indent=2)+'\n')
 assert r.returncode==0,(label,r.stderr[-2000:]);return r.stdout.decode()
archive=OUT/'source.tar'
with archive.open('wb') as f: subprocess.run(['git','-C',str(ROOT/'tempo2'),'archive',PIN],stdout=f,check=True)
for mode in ['original','corrected']:
 source=OUT/mode;source.mkdir()
 run(mode+'-extract',['tar','-xf',archive,'--exclude=T2runtime','-C',source],OUT)
 if mode=='corrected':
  p=source/'Makefile.am';raw=p.read_text();text=raw.replace('$(headers) preProcess.C calculate_bclt.C', '$(headers) preProcess.C',1)
  text=text.replace('libtempo2_la_LIBADD = sofa/libsofa.la', 'libtempo2_la_LIBADD = sofa/libsofa.la libtempo2_bclt.la',1)
  text=text.replace('libtempo2_la_calculate_bclt_CXXFLAGS = @TEMPO2_OPENMP_CXXFLAGS@', '''# AK / 0053: libtempo2_la_calculate_bclt_CXXFLAGS is not a recognized
# Automake per-target flag variable, so calculate_bclt.C lost the OpenMP flags.
# Give that source a real convenience-library target: Automake applies its
# target CXXFLAGS and libtempo2 includes the resulting object through LIBADD.
noinst_LTLIBRARIES = libtempo2_bclt.la
libtempo2_bclt_la_SOURCES = calculate_bclt.C
libtempo2_bclt_la_CXXFLAGS = @TEMPO2_OPENMP_CXXFLAGS@''')
  p.write_text(text);(OUT/'correction.patch').write_text(''.join(difflib.unified_diff(raw.splitlines(True),text.splitlines(True),fromfile='pinned/Makefile.am',tofile='corrected/Makefile.am')))
 run(mode+'-bootstrap',['bash','bootstrap'],source)
 run(mode+'-configure',['./configure','--enable-openmp','--enable-float128','--disable-shared','--without-x'],source)
 # Generate exactly the headers needed by this real compilation rule.
 run(mode+'-headers',['make','TKlongdouble.h','TKcholesky.h','enum_str.h','t2help.h'],source)
 target='calculate_bclt.lo' if mode=='original' else 'libtempo2_bclt_la-calculate_bclt.lo'
 command=run(mode+'-dry-run',['make','-n',target],source)
 if mode=='original':assert '-fopenmp' not in command and '-DHAVE_OPENMP' not in command,command
 else:assert '-fopenmp' in command and '-DHAVE_OPENMP' in command,command
 run(mode+'-compile',['make','V=1',target],source)
 obj=source/(target[:-3]+'.o')
 symbols=run(mode+'-symbols',['nm','-u',obj],source)
 assert ('GOMP_parallel' in symbols)==(mode=='corrected'),symbols
run('versions',['sh','-c','autoconf --version; automake --version; libtool --version; g++-16 --version'],OUT)
(OUT/'receipt.json').write_text(json.dumps(dict(pin=PIN,commands=records,files={str(p.relative_to(OUT)):sha(p) for p in OUT.iterdir() if p.is_file()}),indent=2)+'\n')
print('Original rule omits OpenMP; corrected convenience target compiles GOMP_parallel.')
