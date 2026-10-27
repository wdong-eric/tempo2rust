"""Record the actual local provider, immutable source correspondence and ABI."""
import hashlib, json, subprocess, tarfile
from pathlib import Path
from check import HERE, ROOT, sha

if __name__=='__main__':
    b=HERE/'build';prefix=b/'provider'
    with tarfile.open(b/'calceph-718e794.tar.gz') as archive:
        members=[m for m in archive.getmembers() if m.isfile()]
        for m in members:
            local=b/'calceph-main'/m.name.split('/',1)[1]
            assert local.read_bytes()==archive.extractfile(m).read(),local
    cmd=['clang','-O2','-I'+str(prefix/'include'),str(HERE/'provider_probe.c'),str(prefix/'lib/libcalceph.a'),'-framework','Accelerate','-o',str(b/'provider-probe-published')]
    r=subprocess.run(cmd,capture_output=True);(b/'provider-probe-build.log').write_bytes(r.stdout+r.stderr);assert r.returncode==0
    probe=subprocess.check_output([str(b/'provider-probe-published'),str(b/'inpop17a_TDB_m100_p100_tt.dat')])
    (b/'provider-profile-published.txt').write_bytes(probe)
    versions={}
    for command in [['sw_vers'],['uname','-a'],['rustc','-vV'],['cargo','-V'],['clang','--version'],['/opt/homebrew/bin/g++-16','--version'],['gfortran-16','--version'],['xcrun','--show-sdk-version'],[str(b/'tools/bin/cmake'),'--version'],['otool','-L',str(b/'provider-probe-published')]]:
        versions[' '.join(command)]=subprocess.check_output(command,text=True)
    paths=[b/n for n in ['calceph-718e794.tar.gz','calceph-mirror.tar.gz','inpop17a_TDB_m100_p100_tt.dat','provider/include/calceph.h','provider/lib/libcalceph.a','provider-probe-published','provider-profile-published.txt','calceph-cmake/CMakeCache.txt','mirror-commit.json']]+[HERE/'provider_probe.c',Path(__file__),ROOT/'validation/reference/build/source/T2runtime/ephemeris/DE405.1950.2050']
    record={'source_commit':'718e7945ccd27216b522243c3ff8aab63ce1d99d','immutable_source_files_matched':len(members),'files':{str(p):sha(p) for p in paths},'versions':versions,'probe_command':cmd,'observations':probe.decode()}
    (b/'provider-identity.json').write_text(json.dumps(record,indent=2)+'\n')
    print(json.dumps(record,indent=2))
