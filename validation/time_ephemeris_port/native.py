"""Disposable corrected native expectations; never modifies V sources or outputs."""
import sys,json,difflib
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];V=ROOT/'validation/time_ephemeris'
sys.path.insert(0,str(V))
import profile as native
import fixtures
import run as original
OUT=HERE/'outputs/native';BUILD=HERE/'build/native'
prepare=native.prepare
PATCHES={
 'ifteph.C': [('    ifte.iinfo.pc[1] = 0.0;','    ifte.iinfo.pc[1] = 0.0;\n    ifte.iinfo.twot = 0.0; // Y: reset the argument paired with pc[1].')],
 'tt2tdb.C': [('#include "read_fortran.h"','#include "read_fortran.h"\n#include <string>'),
 ('deltaT = psr[0].obsn[i].correctionTT_calcEph;', 'deltaT = psr[p].obsn[i].correctionTT_calcEph; // Y: current pulsar supplied value.'),
 ('    open_file(fname); /* Open a Fortran made file for reading in C */', '''    open_file(fname); /* Open a Fortran made file for reading in C */
    // Y independent native correction: compare complete opened contents. This
    // is an exact-content witness for Rust's SHA-256 key, not a production hash.
    static std::string previous_contents;
    std::string contents;
    int next_byte;
    while ((next_byte=fgetc(c_fileptr)) != EOF) contents.push_back((char)next_byte);
    fseek(c_fileptr,0,SEEK_SET);
    if (contents != previous_contents) { tdbnrl=-1; for(int q=0;q<16;q++)buf[q]=0; }'''),
 ('    close_file();\n\n    return ctatv;', '    previous_contents=contents; // Publish only after successful computation.\n    close_file();\n\n    return ctatv;')]
}
LEVEL=3
def patched_prepare(name='source',mutation=None):
 src=prepare(name,None);patch=''
 for file,edits in PATCHES.items():
  if file=='tt2tdb.C':
   if LEVEL==1:continue
   if LEVEL==2:edits=[e for i,e in enumerate(edits) if i!=1]
  p=src/file;before=p.read_text();after=before
  for old,new in edits:
   assert after.count(old)==1,(file,old);after=after.replace(old,new)
  p.write_text(after);patch+=''.join(difflib.unified_diff(before.splitlines(True),after.splitlines(True),fromfile=file,tofile=file+'.corrected'))
 (src.parent/'corrections.patch').write_text(patch);return src

def main():
 global LEVEL
 native.BUILD=BUILD;original.BUILD=BUILD;original.OUT=OUT
 native.prepare=patched_prepare
 import importlib.util
 spec=importlib.util.spec_from_file_location('y_run',HERE/'run.py');y=importlib.util.module_from_spec(spec);spec.loader.exec_module(y)
 source_identity={str(p):y.sha(p) for p in HERE.glob('*.py')}
 for LEVEL,name in [(1,'twot'),(2,'twot_fb'),(3,'corrected')]:
  exe,identity=native.compile_probe(name,('Y',name,'explicit corrections.patch'))
  patch=(BUILD/'corrections.patch').read_bytes();(BUILD/(name+'.patch')).write_bytes(patch)
  records=[]
  destination=OUT/('equalpaths' if LEVEL==3 else name)
  for case in fixtures.catalog():
   raw=y.native_launch(exe,identity,case);path=destination/(case['id']+'.json.gz');y.pack(path,raw)
   records.append(dict(id=case['id'],sha256=y.sha(path),events=len(raw['events'].splitlines()),status=raw['status']))
   print(name,case['id'],flush=True)
  assert source_identity=={str(p):y.sha(p) for p in HERE.glob('*.py')}
  receipt=dict(profile=identity,records=records,patch_sha256=native.sha(patch),authored_sources=source_identity)
  native.save(OUT/(name+'-receipt.json'),receipt)
  if LEVEL==3:native.save(OUT/'receipt.json',receipt)
if __name__=='__main__':main()
