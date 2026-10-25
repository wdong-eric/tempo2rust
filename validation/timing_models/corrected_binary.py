#!/usr/bin/env python3
"""G's original observer, relinked with identified AD corrections in owned staging.
Only callbacks whose missing reads are removed acquire corrected defined values.
The accepted original G corpus and undefined classifications remain untouched.
"""
from pathlib import Path
import json,subprocess,os
import native_timing as native
ROOT=native.ROOT;HERE=native.HERE;G=ROOT/'validation/binary';BUILD=HERE/'build/corrected-binary'
def main():
 BUILD.mkdir(parents=True,exist_ok=True)
 identity=json.loads((native.BUILD/'identity.json').read_text())
 objects=list(identity['objects']);exe=BUILD/'export_binary'
 cmd=[native.ac.CC,'-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(native.ac.REF/'source'),'-I'+str(ROOT/'validation/reference'),str(G/'export_binary.cc'),*objects,str(native.ac.REF/'libtempo2-reference.a'),'-lquadmath','-lgfortran','-o',str(exe)]
 subprocess.run(cmd,check=True,capture_output=True)
 cases=['ddk_baseline','ddk_derivatives_undefined','ddk_defaults','ell1h_h3_only','ell1h_h4_nharm','t2_ell1h_h4','t2_companions','ell1_orbifunc_step_undefined']
 out=BUILD/'corpus';out.mkdir(exist_ok=True)
 for name in cases:
  old=json.loads((G/f'outputs/{name}.json').read_text());case=old['case'].copy()
  req=[]
  for line in (G/'fixtures'/case['requests']).read_text().splitlines():
   fields=line.split()
   # STIG is still undefined in H4/H3-only modes; SINI/M2 are undefined in H3-only.
   if name in ['ell1h_h3_only','ell1h_h4_nharm','t2_ell1h_h4'] and fields[1] in ['param_stig','param_sini','param_m2']:continue
   fields[-1]='defined';req.append(' '.join(fields))
  request=BUILD/(name+'.requests');request.write_text('\n'.join(req)+'\n');case['requests']=str(request)
  raw=BUILD/(name+'.jsonl');command=[str(exe),str(G/'fixtures'/case['par']),str(G/'fixtures'/case['tim']),str(request),str(G/'fixtures'/case['labels']),str(raw)]
  r=subprocess.run(command,env={**os.environ,'TEMPO2':str(native.ac.REF/'source/T2runtime')},capture_output=True,timeout=60)
  (BUILD/(name+'.stdout')).write_bytes(r.stdout);(BUILD/(name+'.stderr')).write_bytes(r.stderr)
  assert r.returncode==0,(name,r.returncode)
  records=[json.loads(line) for line in raw.read_text().splitlines()]
  doc={'case':case,'records':records,'original_corpus_sha256':native.sha(G/f'outputs/{name}.json'),'corrected_executable_sha256':native.sha(exe),'native_identity_sha256':native.sha(native.BUILD/'identity.json'),'command':command}
  (out/(name+'.json')).write_text(json.dumps(doc,indent=2)+'\n');print(name,len(records),flush=True)
 (BUILD/'identity.json').write_text(json.dumps({'command':cmd,'observer_sha256':native.sha(G/'export_binary.cc'),'executable_sha256':native.sha(exe),'native_identity_sha256':native.sha(native.BUILD/'identity.json')},indent=2)+'\n')
if __name__=='__main__':main()
