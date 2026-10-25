"""AC staged corrected native integration; accepted A2 archive is read-only.

The archive supplies existing source/SOFA dependencies. Rebuilt raw primary
units below carry explicit scoped corrections; no accepted file is changed.
"""
from pathlib import Path
import difflib,hashlib,json,os,subprocess
HERE=Path(__file__).resolve().parent
ROOT=HERE.parent.parent
BUILD=HERE/'build'/'pipeline'
PIN='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
REF=ROOT/'validation/reference/build'
CC='/opt/homebrew/bin/g++-16'
def sha(data):return hashlib.sha256(data).hexdigest()
def blob(name):return subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',PIN+':'+name])
def build():
 src=BUILD/'source';src.mkdir(parents=True,exist_ok=True)
 lib=REF/'libtempo2-reference.a'
 manifest=json.loads((ROOT/'validation/reference/build_manifest.json').read_text())
 assert sha(lib.read_bytes())==manifest['static_library_sha256']
 patches={
 'preProcess.C':[('    int ndm;','    int ndm=0; // AC: no -setdm means an empty DM table.')],
 'refphs.C':[('if(psr->refphs==REFPHS_TZR)','if(psr[p].refphs==REFPHS_TZR)')],
 'calculate_bclt.C':[('if (psr[p].correctTroposphere==0) psr[0].obsn[i].troposphericDelay=0;','if (psr[p].correctTroposphere==0) psr[p].obsn[i].troposphericDelay=0;')],
 'tropo.C':[('c = frac*bs[ilat2] + (1.0-frac)*cs[ilat1];','c = frac*cs[ilat2] + (1.0-frac)*cs[ilat1];')],
 'initialise.C':[('    psr->ne_sw  = NE_SW_DEFAULT;', '    psr->ne_sw_ifuncN = 0;\n    psr->ne_sw  = NE_SW_DEFAULT;'),
                 ('    psr->ndmx = 0;', '    psr->ndmx = 0;\n    psr->nchromx = 0; // AC: empty chromatic-range table; readParfile:983, dm_delays:282.'),
                 ('    psr->nTNShapeletEvents=0;', '    psr->param[param_dmmodel].linkTo[0]=-1; // AC: fresh unlinked DMMODEL warning.\n    psr->nTNShapeletEvents=0;')],
 }
 # Fresh allocation: initialize only fields whose absent values are consumed
 # before any producer writes them. Direct retained callers are checked against
 # original N/Z/AB evidence separately; this patch does not overwrite such calls.
 allocation='    psr->obsn = (observation *)malloc(sizeof(observation)*MAX_OBSN);'
 scratch='''
    if (psr->obsn) for (int ac=0;ac<MAX_OBSN;ac++) {
        observation& o=psr->obsn[ac];
        o.nclock_correction=0; o.correctionTT_Teph=0; o.einsteinRate=0;
        o.shklovskii=0; o.bat=0; // AC: DM shapelet first-pass SAT sentinel.
        for(int j=0;j<3;j++)o.siteVel[j]=0;
        for(int k=0;k<9;k++)for(int j=0;j<6;j++){o.planet_ssb[k][j]=0;o.planet_ssb_tmr[k][j]=0;}
    }
'''
 patches['initialise.C'].append((allocation,allocation+scratch))
 # readValue may consume omitted uncertainty before any source assignment.
 # Initialize the fresh allocation only. paramSet==0 does not establish that
 # an already allocated uncertainty is unwritten, so never use that predicate
 # in readValue or overwrite retained values during subsequent PAR reads.
 patches['initialise.C'].append((
  'psr->param[i].prefitErr = (longdouble *)malloc(psr->param[i].aSize*sizeof(longdouble));',
  'psr->param[i].prefitErr = (longdouble *)calloc(psr->param[i].aSize,sizeof(longdouble)); // AC: fresh omitted uncertainty.'
 ))
 commands=[];changes=[];objects=[];sources={}
 for name,edits in patches.items():
  raw=blob(name).decode();text=raw
  for old,new in edits:
   if text.count(old)!=1:raise AssertionError((name,old,text.count(old)))
   text=text.replace(old,new)
  (src/name).write_text(text);sources[name]={'original':sha(raw.encode()),'corrected':sha(text.encode())}
  changes.extend(difflib.unified_diff(raw.splitlines(True),text.splitlines(True),fromfile=name,tofile=name))
  obj=BUILD/(name+'.o');cmd=[CC,'-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(REF/'source'),'-I'+str(ROOT/'validation/reference'),'-c',str(src/name),'-o',str(obj)]
  run=subprocess.run(cmd,capture_output=True);(BUILD/(name+'.log')).write_bytes(run.stdout+run.stderr);assert run.returncode==0,run.stderr.decode();commands.append(cmd);objects.append(obj)
 (BUILD/'corrections.patch').write_text(''.join(changes))
 exe=BUILD/'probe';cmd=[CC,'-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(REF/'source'),str(HERE/'native_pipeline.cc'),*map(str,objects),str(lib),'-lquadmath','-lgfortran','-Wl,-map,'+str(BUILD/'probe.map'),'-o',str(exe)]
 run=subprocess.run(cmd,capture_output=True);(BUILD/'link.log').write_bytes(run.stdout+run.stderr);assert run.returncode==0,run.stderr.decode();commands.append(cmd)
 identity={'pin':PIN,'archive_sha256':sha(lib.read_bytes()),'sources':sources,'commands':commands,'patch_sha256':sha((BUILD/'corrections.patch').read_bytes()),'executable_sha256':sha(exe.read_bytes()),'harness_sha256':sha((HERE/'native_pipeline.cc').read_bytes()),'compiler':subprocess.check_output([CC,'--version'],text=True)}
 (BUILD/'identity.json').write_text(json.dumps(identity,indent=2)+'\n')
 return exe
if __name__=='__main__':print(build())
