"""Bounded native QR optional-output witness using the host Accelerate provider."""
from pathlib import Path
import difflib, hashlib, json, os, resource, subprocess, tempfile

ROOT = Path('/Users/Eric.Dong/Downloads/tempo2rust')
PIN = 'fda6ad4a9753ad7dc5d291406f2f310a9161423f'
REF = ROOT / 'validation/reference/build/source'
OUT = Path(tempfile.mkdtemp(prefix='tempo2-native-qr-outputs-'))
Path('/tmp/tempo2-native-qr-outputs-current').write_text(str(OUT) + '\n')
print('OUT', OUT, flush=True)
resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
raw = lambda n: subprocess.check_output(['git', '-C', str(ROOT / 'tempo2'), 'show', PIN + ':' + n], text=True)
archive = ROOT / 'validation/reference/build/libtempo2-reference.a'
manifest = ROOT / 'validation/reference/build_manifest.json'
assert sha(archive) == json.loads(manifest.read_text())['static_library_sha256']
protected = {str(p): sha(p) for p in [archive, manifest, REF / 'config.h', ROOT / 'decisions/0008-preservation-policy-and-stage-plan.md']}
commands = []
env = {**os.environ, 'PATH': '/opt/homebrew/bin:/usr/bin:/bin'}

def run(cmd, cwd, label, require_success=True):
    cmd = list(map(str, cmd))
    with (cwd / (label + '.stdout')).open('w') as stdout, (cwd / (label + '.stderr')).open('w') as stderr:
        r = subprocess.run(cmd, cwd=cwd, env=env, stdout=stdout, stderr=stderr, timeout=45)
    record = {'command': cmd, 'cwd': str(cwd), 'returncode': r.returncode,
              'stdout_sha256': sha(cwd / (label + '.stdout')), 'stderr_sha256': sha(cwd / (label + '.stderr'))}
    commands.append(record)
    (OUT / 'commands.json').write_text(json.dumps(commands, indent=2) + '\n')
    if require_success:
        assert r.returncode == 0, record
    return r.returncode

sdk = Path(subprocess.check_output(['/usr/bin/xcrun', '--show-sdk-path'], text=True).strip())
provider = sdk / 'System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A'
provider_files = [provider / 'Headers/clapack.h', provider / 'libLAPACK.tbd', provider / 'libBLAS.tbd']
protected.update({str(p): sha(p) for p in provider_files})
abi = OUT / 'provider.c'
abi.write_text('''#include <Accelerate/Accelerate.h>
#include <mach-o/dyld.h>
#include <stdio.h>
#include <string.h>
int main(void) {
 printf("integer_bytes %zu\\ndouble_bytes %zu\\n",sizeof(__CLPK_integer),sizeof(__CLPK_doublereal));
 for (unsigned i=0;i<_dyld_image_count();i++) {const char* p=_dyld_get_image_name(i);if(strstr(p,"libLAPACK")||strstr(p,"libBLAS")||strstr(p,"Accelerate"))printf("image %s\\n",p);}
 return 0;
}
''')
run(['/usr/bin/clang', abi, '-framework', 'Accelerate', '-o', OUT / 'provider'], OUT, 'compile-provider')
run([OUT / 'provider'], OUT, 'provider-profile')
profile = (OUT / 'provider-profile.stdout').read_text()
assert 'integer_bytes 4\n' in profile and 'double_bytes 8\n' in profile
assert 'libLAPACK.dylib' in profile and 'libBLAS.dylib' in profile

config = (REF / 'config.h').read_text() + '\n#undef HAVE_BLAS\n#undef HAVE_LAPACK\n#undef F77_FUNC\n#define F77_FUNC(name,NAME) name ## _\n#define HAVE_BLAS 1\n#define HAVE_LAPACK 1\n'
original = raw('TKfit.C')
before = '''        double **augmented_DM=NULL;
        if (computeParam){
            augmented_DM  = malloc_blas(ndata+nconstraints,nparams);
            for (i=0;i<ndata;i++){
                augmented_white_data[i] = white_data[i];
            }
        }'''
after = '''        // Diagnostic 0053 correction: TKfit.C:261-270 fills this matrix
        // even when outP is NULL. QR still consumes the real supplied RHS.
        double **augmented_DM = malloc_blas(ndata+nconstraints,nparams);
        for (i=0;i<ndata;i++){
            augmented_white_data[i] = white_data[i];
        }'''
assert original.count(before) == 1
corrected = original.replace(before, after)
(OUT / 'optional-outputs.patch').write_text(''.join(difflib.unified_diff(original.splitlines(True), corrected.splitlines(True), fromfile='pinned/TKfit.C', tofile='diagnostic/TKfit.C')))
harness = OUT / 'probe.C'
harness.write_text('''#include "tempo2.h"
#include "TKfit.h"
#include "TKmatrix.h"
#include "T2accel.h"
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <cmath>
int main(int argc,char** argv){
 if(argc!=4)return 2;
 const char* mode=argv[1];int scaled=atoi(argv[2]);int constrained=atoi(argv[3]);
 useT2accel=2;writeResiduals=0;
 double **dm=malloc_blas(3,2),**wm=malloc_blas(3,2),**cov=malloc_blas(2,2),**constraint=malloc_blas(1,2);
 double data[]={1,2,4},white[]={1,2,4},targets[]={3},parameters[]={-99,-99},errors[]={-99,-99};
 for(int i=0;i<3;i++){dm[i][0]=wm[i][0]=1;dm[i][1]=wm[i][1]=i-1;}
 constraint[0][0]=0;constraint[0][1]=1;
 double *p=!strcmp(mode,"full")?parameters:NULL;
 double *e=(!strcmp(mode,"full")||!strcmp(mode,"errors"))?errors:NULL;
 double **c=(!strcmp(mode,"full")||!strcmp(mode,"covariance"))?cov:NULL;
 double chi=TKrobustDefConstrainedLeastSquares(data,white,dm,wm,constraint,3,2,constrained,1e-10,scaled,p,e,c,0,constrained?targets:NULL);
 if(!std::isfinite(chi))return 3;
 printf("result chi %.17g\\n",chi);
 for(int i=0;i<2;i++){if(p)printf("result p%d %.17g\\n",i,p[i]);if(e)printf("result e%d %.17g\\n",i,e[i]);if(c)for(int j=0;j<2;j++)printf("result c%d%d %.17g\\n",i,j,c[i][j]);}
 for(int i=0;i<3;i++)if(data[i]!=(i==2?4:i+1)||white[i]!=data[i]||dm[i][0]!=1||wm[i][0]!=1||dm[i][1]!=i-1||wm[i][1]!=i-1)return 4;
 free_blas(dm);free_blas(wm);free_blas(cov);free_blas(constraint);return 0;
}
''')
records = []
for variant, text in [('original', original), ('corrected', corrected)]:
    work = OUT / variant
    work.mkdir()
    (work / 'config.h').write_text(config)
    (work / 'TKfit.C').write_text(text)
    for name in ['T2accel.C', 'T2accel.h']:
        (work / name).write_text(raw(name))
    objects = []
    for name in ['TKfit.C', 'T2accel.C']:
        obj = work / (name + '.o')
        run(['/opt/homebrew/bin/g++-16', '-O2', '-DHAVE_CONFIG_H', '-I'+str(work), '-I'+str(REF), '-c', work/name, '-o', obj], work, 'compile-'+name)
        objects.append(obj)
    executable = work / 'probe'
    run(['/opt/homebrew/bin/g++-16', '-O2', '-DHAVE_CONFIG_H', '-I'+str(work), '-I'+str(REF), harness, *objects, archive, '-framework', 'Accelerate', '-lquadmath', '-lgfortran', '-o', executable], work, 'link')
    run(['/usr/bin/otool', '-L', executable], work, 'linkage')
    for constraint in [0,1]:
        for scaled in [0,1]:
            for mode in ['full','covariance','errors','none']:
                case = work / f'constraint-{constraint}-scaled-{scaled}-{mode}'
                case.mkdir()
                code = run([executable,mode,scaled,constraint],case,'run',False)
                values = {}
                if code == 0:
                    for line in (case/'run.stdout').read_text().splitlines():
                        if line.startswith('result '):
                            _, key, value = line.split();values[key] = value
                row = {'variant':variant,'mode':mode,'constraints':constraint,'rescale_errors':scaled,'returncode':code,'values':values}
                records.append(row)
                print(variant,constraint,scaled,mode,code,values,flush=True)
                (OUT/'records.json').write_text(json.dumps(records,indent=2)+'\n')
                assert code == (0 if variant == 'corrected' or mode == 'full' else -11), row
for row in records:
    if row['returncode']:
        continue
    control = next(r for r in records if r['variant']=='original' and r['mode']=='full' and r['constraints']==row['constraints'] and r['rescale_errors']==row['rescale_errors'])
    assert row['values'] and all(v == control['values'][k] for k,v in row['values'].items()), row
for path,digest in protected.items():
    assert sha(Path(path)) == digest, path
receipt = {'scope':'Bounded native source-bug witness on macOS arm64 with host Accelerate legacy LP64 LAPACK/BLAS; no Rust execution, provider selection, complete backend qualification or numerical acceptance.',
           'pin':PIN,'review_source_sha256':sha(Path(__file__)),'provider_profile':profile,
           'os':subprocess.check_output(['/usr/bin/sw_vers'],text=True),'kernel':subprocess.check_output(['/usr/bin/uname','-a'],text=True),
           'compiler':subprocess.check_output(['/opt/homebrew/bin/g++-16','--version'],text=True),'sdk_path':str(sdk),
           'patch_sha256':sha(OUT/'optional-outputs.patch'),'source_sha256':{str(p.relative_to(OUT)):sha(p) for p in OUT.rglob('*') if p.is_file() and p.suffix in ['.C','.c','.h']},
           'executable_sha256':{v:sha(OUT/v/'probe') for v in ['original','corrected']},'protected_files':protected,'commands':commands,'records':records,
           'checks':'Original full-output cases complete. Original optional-output cases terminate at the null augmented matrix. Corrected requests consume the supplied RHS and match every requested full-output control value exactly, including augmented-row chi-square and its rescaled covariance. Caller data/design arrays remain unchanged. No absent-RHS case is included.'}
(OUT/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print('RECEIPT',OUT/'receipt.json',sha(OUT/'receipt.json'),flush=True)
