"""Pinned source/configuration preparation for Assignment J; writes only here."""
import hashlib
import json
import os
import platform
import re
import shutil
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
BUILD = HERE / 'build'
SOURCE = 'fda6ad4a9753ad7dc5d291406f2f310a9161423f'
LIBRARY = ROOT / 'validation/reference/build/libtempo2-reference.a'
LIBRARY_SHA = '313507c5063cb6a8156a0e5fd0f752e2baa59a77c78b5b32822d8efeddd2dbf5'
FILES = ['tabulatedfunction.C', 'tabulatedfunction.h', 'clkcorr.C', 'toa2utc.C',
         'tempo2.h', 'observatory.C', 'TKlog.C', 'TKlog.h', 'TKlongdouble.C',
         'TKlongdouble.float128.h', 'TKlongdouble.ld.h', 'tempo2Util.C', 'global.C',
         'tai2tt.C', 'utc2tai.C', 'readJBO_bat.C', 'read_fortran2.h']
ENV = {**os.environ, 'LC_ALL': 'C', 'LANG': 'C'}
ENV.pop('TEMPO2_ALIAS', None)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def save(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True, allow_nan=False) + '\n')


def blob(name):
    return subprocess.check_output(['git', '-C', str(ROOT/'tempo2'), 'show', SOURCE+':'+name])


def prepare():
    src = BUILD / 'source'
    src.mkdir(parents=True, exist_ok=True)
    manifest = {}
    for name in FILES:
        data = blob(name)
        (src/name).write_bytes(data)
        manifest[name] = dict(sha256=sha(data), size=len(data), origin='pinned Git blob')
    numeric = blob('TKlongdouble.float128.h')
    (src/'TKlongdouble.h').write_bytes(numeric)
    config = (ROOT/'validation/reference/build/source/config.h').read_bytes()
    (src/'config.h').write_bytes(config)
    manifest['TKlongdouble.h'] = dict(sha256=sha(numeric), origin='TKlongdouble.float128.h verbatim')
    manifest['config.h'] = dict(sha256=sha(config), text=config.decode(), origin='A2 generated minimal config')
    assert sha(LIBRARY.read_bytes()) == LIBRARY_SHA
    archive_symbols = subprocess.check_output(['nm', '-g', str(LIBRARY)], text=True, stderr=subprocess.DEVNULL)
    selected = {}
    for name in ['toa2utc', 'defineClockCorrectionSequence', 'getClockCorrections', 'getCorrectionTT',
                 'getCorrection', 'readJBO_bat', 'tai2tt', 'utc2tai', 'convertTOA', 'toa2utc_nist', 'linearInterpolate']:
        selected[name] = re.findall(r'^.*\bT _'+name+r'$', archive_symbols, re.M)
    cc = shutil.which('g++-16')
    assert cc, 'g++-16 is required for binary128 reference'
    flags = ['-O2', '-std=gnu++11', '-DHAVE_CONFIG_H', '-I'+str(src)]
    preprocessed = {}
    for name in ['toa2utc.C','tai2tt.C','utc2tai.C','readJBO_bat.C']:
        command = [cc, *flags, '-E', '-P', str(src/name)]
        result = subprocess.run(command, check=True, text=True, capture_output=True, env=ENV)
        (BUILD/(name+'.ii')).write_text(result.stdout)
        # Definitions, not surviving declarations, determine the active profile.
        definitions = re.findall(r'\b(?:void|double)\s+(toa2utc|tai2tt|utc2tai|convertTOA|toa2utc_nist|linearInterpolate|readJBO_bat)\s*\([^;{}]*\)\s*\{', result.stdout)
        preprocessed[name] = dict(command=command, sha256=sha(result.stdout.encode()), definitions=definitions)
    assert preprocessed['toa2utc.C']['definitions'] == ['toa2utc']
    assert preprocessed['tai2tt.C']['definitions'] == preprocessed['utc2tai.C']['definitions'] == []
    assert preprocessed['readJBO_bat.C']['definitions'] == ['readJBO_bat']
    expected = {'toa2utc','defineClockCorrectionSequence','getClockCorrections','getCorrectionTT','getCorrection','readJBO_bat'}
    assert {k for k,v in selected.items() if v} == expected
    out = dict(source_revision=SOURCE, central_release='67aa77c', sources=manifest,
               compiler=subprocess.check_output([cc,'--version'],text=True).splitlines()[0],
               platform=platform.platform(), machine=platform.machine(), locale='C', flags=flags,
               support_archive_sha256=LIBRARY_SHA, archive_selected_defined_symbols=selected,
               preprocessed=preprocessed,
               support_policy='Raw primary, observatory, TKlog, TKlongdouble and tempo2Util compiled first; archive supplies canonical global variables and uncalled dependencies. initialiseOne is never called; no A2 resource tracing on exercised paths.')
    save(BUILD/'profile.json',out)
    return src, cc, flags, out


if __name__ == '__main__':
    _,_,_,profile = prepare()
    save(HERE/'profile.json',profile)
    print('PASS: pinned source, configuration, archive and active/disabled symbols')
