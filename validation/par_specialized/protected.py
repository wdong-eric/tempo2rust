"""Read-only accepted-evidence membership; excludes both active peers explicitly."""
import json,subprocess
from profile import ROOT,HERE,sha,save,project_blob,RELEASE
DIRS=['validation/parameters','validation/clocks','validation/numeric_toolkit','validation/tim_io','validation/precision','validation/binary','validation/jpl','validation/coverage']
def snapshot():
    files=subprocess.check_output(['git','-C',str(ROOT),'ls-tree','-r','--name-only','-z',RELEASE,'--',*DIRS]).decode().split('\0')
    result={'project':{n:sha((ROOT/n).read_bytes() if (ROOT/n).exists() else project_blob(n,RELEASE)) for n in files if n},'nested':{}}
    result['membership_revision']=RELEASE
    result['archived_project_files']=[n for n in files if n and not (ROOT/n).exists()]
    for n in ['tempo2','validation/reference']:
        root=ROOT/n
        names=subprocess.check_output(['git','-C',str(root),'ls-files','-z']).decode().split('\0')
        result['nested'][n]=dict(commit=subprocess.check_output(['git','-C',str(root),'rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','-C',str(root),'status','--porcelain'],text=True),files={f:sha((root/f).read_bytes()) for f in names if f})
        assert result['nested'][n]['status']=='',n
    return result
if __name__=='__main__':
    import sys
    actual=snapshot();p=HERE/'protected.json'
    if '--record' in sys.argv:save(p,actual)
    else:assert json.loads(p.read_text())==actual
    print('PASS immutable membership; preprocessing and parameter_storage_port excluded')
