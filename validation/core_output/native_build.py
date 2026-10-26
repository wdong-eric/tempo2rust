"""AH oracle: pinned entry/output, accepted corrected engine objects, explicit patches.

Accepted generated artifacts are read-only and hash checked. Run in this directory:
python3 native_build.py build/native
"""
from pathlib import Path
import subprocess, hashlib, json, sys, difflib

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
PIN = 'fda6ad4a9753ad7dc5d291406f2f310a9161423f'
REF = ROOT / 'validation/reference/build'
CXX = '/opt/homebrew/bin/g++-16'
sys.path.insert(0, str(ROOT / 'validation/general_fitting'))
from fitting_corrections import corrected as ae_corrected

def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

def blob(name):
    return subprocess.check_output(['git', '-C', str(ROOT / 'tempo2'), 'show', PIN + ':' + name]).decode()

def output_corrected(s):
    s = s.replace('psr->param[param_start]', 'psr[p].param[param_start]').replace('psr->param[param_finish]', 'psr[p].param[param_finish]')
    s = s.replace('for (p=0;p<npsr;p++)\n    {', 'for (p=0;p<npsr;p++)\n    {\n        chisqr=psr[p].fitChisq;')
    s = s.replace('if (p==0 && psr[0].globalNfit > 0)', 'if (p==0 && psr[0].nGlobal > 0 && psr[0].globalNfit > 0)')
    # Snapshot the existing defined value when report-owned fields first become active.
    for label, marker in [('param_tzrmjd', '                psr[p].param[param_tzrmjd].val[0] ='), ('param_tzrfrq', '                psr[p].param[param_tzrfrq].val[0] ='), ('param_tres', '    psr[p].param[param_tres].val[0] =')]:
        assert marker in s
        s = s.replace(marker, f'    if (!psr[p].param[{label}].paramSet[0]) {{ psr[p].param[{label}].prefit[0]=psr[p].param[{label}].val[0]; psr[p].param[{label}].prefitErr[0]=psr[p].param[{label}].err[0]; }}\n' + marker)
    return s

def fit_corrected(s):
    s = ae_corrected('t2fit.C', s)
    marker='void t2fit_prefit(pulsar* psr, int npsr){'
    s=s.replace(marker,marker+'\n    for(int i=0;i<npsr;i++)psr[i].globalNfit=0;')
    for label in ['param_start', 'param_finish']:
        marker = f'    psr->param[{label}].val[0] ='
        assert marker in s
        s = s.replace(marker, f'    if (!psr->param[{label}].paramSet[0]) {{ psr->param[{label}].prefit[0]=psr->param[{label}].val[0]; psr->param[{label}].prefitErr[0]=psr->param[{label}].err[0]; }}\n' + marker)
    return s

def observer_corrected(s):
    # Retain the already used clock correction and EOP dut1; no extra query,
    # timing expression, legacy model call or fallback is introduced.
    signature = '    void \nget_obsCoord_IAU2000B('
    assert signature in s
    s = s.replace(signature, 'static double\nah_get_obsCoord_IAU2000B(', 1)
    marker = '    iau_sxp_(&one_on_c, crs[1], observatory_velocity_crs);\n}'
    assert marker in s
    s = s.replace(marker, marker[:-1] + '''    return dut1;
}
void get_obsCoord_IAU2000B(double trs[3], double zen[3], longdouble tt, longdouble utc,
    double pos[3], double z[3], double vel[3], char* eop) {
    (void)ah_get_obsCoord_IAU2000B(trs,zen,tt,utc,pos,z,vel,eop);
}''', 1)
    marker = '                        longdouble utc = psr[p].obsn[i].sat;'
    assert marker in s
    s = s.replace(marker, '                        double ah_site_to_utc_s=0.0;\n' + marker)
    old = '''                            utc += getCorrection(psr[p].obsn+i, 
                                    psr[p].clockFromOverride,
                                    "UTC", psr[p].noWarnings)/SECDAY;'''
    assert old in s
    s = s.replace(old, '''                        {
                            ah_site_to_utc_s = getCorrection(psr[p].obsn+i,
                                    psr[p].clockFromOverride, "UTC", psr[p].noWarnings);
                            utc += ah_site_to_utc_s/SECDAY;
                        }''')
    s = s.replace('                        get_obsCoord_IAU2000B(trs, zenith_trs,',
                  '                        double ah_dut1 = ah_get_obsCoord_IAU2000B(trs, zenith_trs,')
    marker = '                                psr[p].obsn[i].siteVel,psr[p].eopc04_file);'
    assert marker in s
    return s.replace(marker, marker + '\n                        psr[p].obsn[i].correctionUT1 = ah_site_to_utc_s + ah_dut1;')

def build(out):
    out = Path(out).resolve(); (out / 'source').mkdir(parents=True, exist_ok=True)
    dependencies = {}
    for directory in [ROOT / 'validation/timing_models/build/native', ROOT / 'validation/predictors/build/native-corrected']:
        identity = json.loads((directory / 'identity.json').read_text())
        if directory.name == 'native-corrected':
            assert identity['executable'] == '3bf82ca766e45599c23a575868d6977d97c2f25c46428c02c877ccca0276a27f'
        for p in directory.glob('*.o'):
            assert sha(p) == identity['objects'][str(p)], p
            dependencies[p.name] = p
        if directory.name == 'native':
            for p in (directory / 'ac').glob('*.o'):
                if p.name == 'initialise.C.o': continue
                assert sha(p) == identity['objects'][str(p)], p
                dependencies[p.name] = p
    archive = REF / 'libtempo2-reference.a'
    assert sha(archive) == json.loads((ROOT / 'validation/reference/build_manifest.json').read_text())['static_library_sha256']
    commands = []; sources = {}; patch = ''
    def compile(name, original, value, extra=()):
        nonlocal patch
        p = out / 'source' / name; p.write_text(value)
        sources[name] = {'original': hashlib.sha256(original.encode()).hexdigest(), 'compiled': sha(p)}
        patch += ''.join(difflib.unified_diff(original.splitlines(True), value.splitlines(True), fromfile='pinned/' + name, tofile='corrected/' + name))
        obj = out / (name + '.o')
        cmd = [CXX, '-O2', '-DHAVE_CONFIG_H', '-I' + str(REF / 'source'), '-I' + str(out / 'source'), *extra, '-c', str(p), '-o', str(obj)]
        r = subprocess.run(cmd, capture_output=True); (out / (name + '.log')).write_bytes(r.stdout + r.stderr)
        assert r.returncode == 0, r.stderr.decode(); commands.append(cmd)
        return obj
    names = ['TKfit.C', 'TKrobust.C', 't2fit.C', 't2fit_nestlike.C', 't2fit_stdFitFuncs.C', 't2fit_fitwaves.C', 't2fit_gw.C', 'constraints_covar.C', 'cholesky.C', 'choleskyRoutines.C']
    for name in names:
        original = blob(name); value = fit_corrected(original) if name == 't2fit.C' else ae_corrected(name, original)
        dependencies[name + '.o'] = compile(name, original, value)
    original = blob('get_obsCoord.C')
    original_observer = compile('original_observer.C', original, original)
    dependencies['get_obsCoord.C.o'] = compile('get_obsCoord.C', original, observer_corrected(original))
    original = blob('preProcessSimple.C')
    value = original.replace('    /* Set tempo emulation mode */', '    if (!psr->param[param_ephver].paramSet[0]) { psr->param[param_ephver].prefit[0]=psr->param[param_ephver].val[0]; psr->param[param_ephver].prefitErr[0]=psr->param[param_ephver].err[0]; }\n    /* Set tempo emulation mode */')
    marker = '        psr->param[param_start].val[0] ='
    value = value.replace(marker, '        if (!psr->param[param_start].paramSet[0]) { psr->param[param_start].prefit[0]=psr->param[param_start].val[0]; psr->param[param_start].prefitErr[0]=psr->param[param_start].err[0]; }\n' + marker)
    dependencies['preProcessSimple.C.o'] = compile('preProcessSimple.C', original, value)
    original = blob('TKlongdouble.C')
    formatter = original.replace('    va_end(args);\n    va_end(oargs);', '    // AH: caller owns args; oargs was never initialized.')
    # %.0u for zero and %.0s legitimately produce zero bytes. The source's
    # preincrement skips the terminator and scans uninitialized output memory.
    formatter = formatter.replace("while(*(++o)!='\\0'){\n                    continue;", "while(*o!='\\0'){\n                    ++o;")
    dependencies['TKlongdouble.C.o'] = compile('TKlongdouble.C', original, formatter)
    for name in ['displayParameters.C', 'TKlog.C', 'getInputs.C']:
        original = blob(name)
        if name == 'getInputs.C':
            h = ''.join('const char ' + symbol + '[] = {' + ','.join(map(str, (ROOT / 'tempo2' / filename).read_bytes())) + ',0};\n' for filename, symbol in [('help.txt', 'help_txt'), ('help_extra.txt', 'help_extra_txt')])
            (out / 'source/t2help.h').write_text(h)
        dependencies[name + '.o'] = compile(name, original, original)
    original = blob('textOutput.C')
    original_output = compile('original_output.C', original, original)
    dependencies['textOutput.C.o'] = compile('textOutput.C', original, output_corrected(original))
    hooks = '#include "tempo2.h"\n#include "tempo2pred.h"\nvoid af_polyco(pulsar*,int,int,longdouble*,longdouble,longdouble,double,double,double,const char*,int,double,const char*,longdouble,longdouble){}\nextern "C" void af_plan_interval(int,const T2Predictor*,long double,long double){}\n'
    dependencies['hooks.o'] = compile('hooks.cc', hooks, hooks)
    main = compile('tempo2.C', blob('tempo2.C'), blob('tempo2.C'))
    def link(exe, objects):
        cmd = [CXX, *map(str, objects), str(archive), '-lquadmath', '-lgfortran', '-o', str(exe)]
        r = subprocess.run(cmd, capture_output=True); exe.with_suffix('.link.log').write_bytes(r.stdout + r.stderr)
        assert r.returncode == 0, r.stderr.decode(); commands.append(cmd)
    link(out / 'tempo2', [main, *dependencies.values()])
    # argc==2 selects standalone TIM/help grammar, so resource flags cannot be
    # added to that command. A separate witness bounds only default allocation.
    bounded_main = compile('bounded_main.C', blob('tempo2.C'), blob('tempo2.C'), ['-Dmain=ah_source_main'])
    driver = '#include "tempo2.h"\nextern int ah_source_main(int,char**);\nint main(int n,char** a){MAX_PSR=2;MAX_OBSN=100;return ah_source_main(n,a);}\n'
    bounded_driver = compile('bounded_driver.cc', driver, driver)
    link(out / 'tempo2-bounded', [bounded_driver, bounded_main, *dependencies.values()])
    if (HERE / 'native_probe.cc').exists():
        probe = compile('native_probe.cc', (HERE / 'native_probe.cc').read_text(), (HERE / 'native_probe.cc').read_text())
        link(out / 'probe', [probe, *dependencies.values()])
        originals = dict(dependencies); originals['textOutput.C.o'] = original_output
        originals['get_obsCoord.C.o'] = original_observer
        originals['t2fit.C.o'] = compile('original_fit.C', blob('t2fit.C'), ae_corrected('t2fit.C', blob('t2fit.C')))
        originals['preProcessSimple.C.o'] = compile('original_simple.C', blob('preProcessSimple.C'), blob('preProcessSimple.C'))
        link(out / 'probe-original', [probe, *originals.values()])
        # Defined poison witnesses diagnose reads, never golden undefined bytes.
        original = blob('textOutput.C')
        originals['textOutput.C.o'] = compile('poison_output.C', original, original.replace('    double chisqr;', '    double chisqr=-12345.0;'))
        link(out / 'probe-poison', [probe, *originals.values()])
    (out / 'corrections.patch').write_text(patch)
    (out / 'identity.json').write_text(json.dumps({'pin': PIN, 'sources': sources, 'objects': {str(p): sha(p) for p in dependencies.values()}, 'archive': sha(archive), 'commands': commands, 'executable': sha(out / 'tempo2'), 'executables': {name: sha(out / name) for name in ['tempo2', 'tempo2-bounded', 'probe', 'probe-original', 'probe-poison']}, 'af_identity': sha(ROOT / 'validation/predictors/build/native-corrected/identity.json'), 'builder': sha(Path(__file__)), 'patch': sha(out / 'corrections.patch')}, indent=2) + '\n')
    return out / 'tempo2'

if __name__ == '__main__':
    print(build(sys.argv[1]))
