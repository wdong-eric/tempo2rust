"""Assignment K: canonical source staging, identity, and native build (no A2 library)."""
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import shutil
import subprocess

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
BUILD = HERE/'build'
PIN = 'fda6ad4a9753ad7dc5d291406f2f310a9161423f'
ENV = {**os.environ, 'LC_ALL': 'C', 'LANG': 'C'}
ENV.pop('TEMPO2_ALIAS', None)
SOURCES = ['initialise.C', 'readParfile.C', 'TKlongdouble.C', 'tempo2Util.C',
           'TKmatrix.C', 'TKlog.C', 'clkcorr.C', 'tabulatedfunction.C', 'observatory.C']
HEADERS = ['tempo2.h', 'TKlongdouble.float128.h', 'TKlongdouble.ld.h', 'TKlog.h',
           'TKmatrix.h', 'T2accel.h', 'tempo2Util.h', 't2fit_nestlike.h',
           'tabulatedfunction.h', 'global.C', 'TKfit.h']


def sha(data):
    return hashlib.sha256(data).hexdigest()


def save(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True, allow_nan=False)+'\n')


def runner_identity():
    return {name:sha((HERE/name).read_bytes()) for name in
            ['run.py','profile.py','fixtures.py','inventory.py','verify.py','coverage.py','probe.cc']}


def blob(name):
    return subprocess.check_output(['git', '-C', str(ROOT/'tempo2'), 'show', PIN+':'+name])


def uncomment(s):
    # Preserve newlines and strings, including braces within strings.
    return re.sub(r'"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\'|//[^\n]*|/\*[\s\S]*?\*/',
                  lambda m: re.sub(r'[^\n]', ' ', m[0]) if m[0].startswith(('/',)) else m[0], s)


def enum_names():
    s = uncomment(blob('tempo2.h').decode())
    return re.findall(r'\bparam_\w+', re.search(r'enum label\s*\{(.*?)\}', s, re.S)[1])


def prepare():
    src = BUILD/'source'
    src.mkdir(parents=True, exist_ok=True)
    for name in SOURCES + HEADERS:
        (src/name).write_bytes(blob(name))
    (src/'TKlongdouble.h').write_bytes(blob('TKlongdouble.float128.h'))
    (src/'config.h').write_bytes((ROOT/'validation/reference/build/source/config.h').read_bytes())
    global_data = blob('global.C')
    # Source data declarations and CVS routine, verbatim. No timing engine dependency.
    (src/'global_data.C').write_bytes(global_data[:global_data.index(b'void extra_delays')]+
                                    global_data[global_data.index(b'void CVSdisplayVersion'):])
    (src/'names.h').write_text('static const char *names[] = {'+', '.join(json.dumps(n) for n in enum_names())+'};\n')
    s = uncomment(blob('initialise.C').decode()).split('void initialiseOne')[1].split('void allocateMemory')[0]
    fields = list(dict.fromkeys(re.findall(r'psr->(\w+)\s*=',s)+re.findall(r'strcpy\(psr->(\w+)\s*,',s)))
    fields = [f for f in fields if f not in ['obsn','covar','ToAextraCovar']]
    (src/'controls.h').write_text(''.join('FIELD('+f+');\n' for f in fields))
    return src


def compile_probe(src, name='probe', mutation=None, coverage=False):
    runner_sources = runner_identity()
    if mutation:
        dest = BUILD/(name+'_source')
        shutil.copytree(src, dest, dirs_exist_ok=True)
        src = dest
        filename, old, new = mutation
        p = src/filename
        s = p.read_text()
        assert s.count(old) == 1, (filename, old, s.count(old))
        p.write_text(s.replace(old, new))
    cc = shutil.which('g++-16')
    assert cc, 'GCC 16 binary128 backend required'
    flags = ['-O2', '-std=gnu++11', '-DHAVE_CONFIG_H', '-I'+str(src)]
    if coverage:
        flags += ['--coverage']
    exe = BUILD/name
    exe.unlink(missing_ok=True)
    command = [cc, *flags, str(HERE/'probe.cc'), *[str(src/n) for n in SOURCES],
               str(src/'global_data.C'), '-lquadmath', '-Wl,-map,'+str(BUILD/(name+'.map')), '-o', str(exe)]
    p = subprocess.run(command, capture_output=True, text=True, env=ENV)
    (BUILD/(name+'.compile.log')).write_text(p.stdout+p.stderr)
    if p.returncode:
        raise RuntimeError(p.stderr)
    profile = dict(source_revision=PIN, compiler=subprocess.check_output([cc,'--version'],text=True).splitlines()[0],
                   compiler_sha256=sha(Path(cc).read_bytes()), command=command,
                   platform=platform.platform(), machine=platform.machine(), locale='C',
                   sources={n:sha((src/n).read_bytes()) for n in SOURCES+HEADERS},
                   generated={n:dict(sha256=sha((src/n).read_bytes()),text=(src/n).read_text()) for n in ['config.h','names.h','controls.h']},
                   numeric_header_sha256=sha((src/'TKlongdouble.h').read_bytes()),
                   globals_data_sha256=sha((src/'global_data.C').read_bytes()),
                   globals_slices=['global.C:1-64', 'global.C:176-200'],
                   executable_sha256=sha(exe.read_bytes()), probe_sha256=sha((HERE/'probe.cc').read_bytes()),
                   link_map_sha256=sha((BUILD/(name+'.map')).read_bytes()),
                   dependencies=subprocess.check_output(['otool','-L',str(exe)],text=True).splitlines()[1:],
                   runtime_hashes={n:sha(Path(subprocess.check_output([cc,'-print-file-name='+n],text=True).strip()).read_bytes()) for n in ['libquadmath.0.dylib','libstdc++.6.dylib','libgcc_s.1.1.dylib']},
                   support_policy='All primary and support translation units from raw pinned Git bytes. Only global.C data prefix and CVSdisplayVersion are sliced. No A2 archive or initializer correction. Header/config generated as declared. Real clock/table/observatory support is linked.',
                   mutation=mutation, coverage_instrumentation=coverage,
                   runner_sources=runner_sources)
    assert runner_identity() == runner_sources, 'Runner/harness changed during compilation'
    return exe, profile
