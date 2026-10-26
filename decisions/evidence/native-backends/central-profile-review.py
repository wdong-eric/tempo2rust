"""Compile-only witness for T2accel's BLAS declaration guard; no provider executes."""
from pathlib import Path
import difflib
import hashlib
import json
import subprocess
import tempfile

ROOT = Path('/Users/Eric.Dong/Downloads/tempo2rust')
PIN = 'fda6ad4a9753ad7dc5d291406f2f310a9161423f'
OUT = Path(tempfile.mkdtemp(prefix='tempo2-backend-compile-profile-'))
Path('/tmp/tempo2-backend-compile-profile-current').write_text(str(OUT) + '\n')
REF = ROOT / 'validation/reference/build/source'
sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
raw = subprocess.check_output(['git', '-C', str(ROOT / 'tempo2'), 'show', PIN + ':T2accel.C'], text=True)
header = subprocess.check_output(['git', '-C', str(ROOT / 'tempo2'), 'show', PIN + ':T2accel.h'])

# Confirm the selected Fortran compiler's ordinary external spelling. This
# establishes only symbol naming, not any installed BLAS/LAPACK provider ABI.
fortran = OUT / 'symbol.f90'
fortran.write_text('subroutine centralprofileprobe()\nend subroutine\n')
subprocess.run(['/opt/homebrew/bin/gfortran-16', '-c', str(fortran), '-o', str(OUT / 'symbol.o')], check=True)
symbols = subprocess.check_output(['/usr/bin/nm', str(OUT / 'symbol.o')], text=True)
(OUT / 'symbols.txt').write_text(symbols)
assert '_centralprofileprobe_' in symbols

# Bounded 0053 correction: the two BLAS declarations were inside HAVE_LAPACK,
# while their callers are inside HAVE_BLAS. Relocate only that declaration block.
start = raw.index('#define F77_dgemm')
end = raw.index('\n\n\nextern "C" {', start)
declarations = raw[start:end]
corrected = raw[:start] + raw[end:]
corrected = corrected.replace('#ifdef HAVE_LAPACK\n', '#ifdef HAVE_BLAS\n' + declarations + '\n#endif\n\n#ifdef HAVE_LAPACK\n', 1)
(OUT / 'declaration-guard.patch').write_text(''.join(difflib.unified_diff(raw.splitlines(True), corrected.splitlines(True), fromfile='pinned/T2accel.C', tofile='corrected/T2accel.C')))
records = []
config = (REF / 'config.h').read_text()
for variant, source in [('original', raw), ('corrected', corrected)]:
    directory = OUT / variant
    directory.mkdir()
    (directory / 'T2accel.C').write_text(source)
    (directory / 'T2accel.h').write_bytes(header)
    for name, blas, lapack in [('disabled', False, False), ('blas-only', True, False), ('blas-lapack', True, True)]:
        case = directory / name
        case.mkdir()
        suffix = '\n#undef HAVE_BLAS\n#undef HAVE_LAPACK\n#undef F77_FUNC\n#define F77_FUNC(name,NAME) name ## _\n'
        if blas:
            suffix += '#define HAVE_BLAS 1\n'
        if lapack:
            suffix += '#define HAVE_LAPACK 1\n'
        (case / 'config.h').write_text(config + suffix)
        command = ['/opt/homebrew/bin/g++-16', '-fsyntax-only', '-DHAVE_CONFIG_H', '-I' + str(case), '-I' + str(REF), str(directory / 'T2accel.C')]
        result = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
        (case / 'compiler.log').write_text(result.stdout)
        records.append({'variant': variant, 'profile': name, 'command': command,
                        'exit_code': result.returncode, 'config_sha256': sha(case / 'config.h'),
                        'log_sha256': sha(case / 'compiler.log')})

assert [r['exit_code'] for r in records] == [0, 1, 0, 0, 0, 0]
failure = (OUT / 'original/blas-only/compiler.log').read_text()
assert 'F77_dgemm' in failure and 'F77_dgemv' in failure
receipt = {
    'scope': 'Source syntax checks only. No BLAS/LAPACK library is linked or executed, no Rust code is changed, and no provider numerical/ABI qualification is claimed.',
    'pin': PIN, 'reference_config_sha256': sha(REF / 'config.h'),
    'configured_symbol_macro': 'F77_FUNC(name,NAME) name ## _, checked against a tiny GFortran 16 external symbol on this host; not a provider ABI assertion.',
    'symbol_witness_sha256': sha(OUT / 'symbol.o'), 'symbol_listing_sha256': sha(OUT / 'symbols.txt'),
    'original_source_sha256': sha(OUT / 'original/T2accel.C'),
    'header_sha256': sha(OUT / 'original/T2accel.h'),
    'corrected_source_sha256': sha(OUT / 'corrected/T2accel.C'),
    'patch_sha256': sha(OUT / 'declaration-guard.patch'),
    'compiler': subprocess.check_output(['/opt/homebrew/bin/g++-16', '--version'], text=True),
    'fortran_compiler': subprocess.check_output(['/opt/homebrew/bin/gfortran-16', '--version'], text=True),
    'records': records,
}
(OUT / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
print(json.dumps({'directory': str(OUT), 'receipt_sha256': sha(OUT / 'receipt.json'),
                  'results': [(r['variant'], r['profile'], r['exit_code']) for r in records]}, indent=2))
