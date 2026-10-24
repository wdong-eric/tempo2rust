"""Compiled V behavioral controls against Y corrected semantics, kept in Y."""
import importlib.util,json,sys
from pathlib import Path
HERE=Path(__file__).resolve().parent

def load(name,path):
 spec=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m

def main():
 y=load('y_run_mutations',HERE/'run.py');n=load('y_native_mutations',HERE/'native.py')
 v=load('v_verify_mutations',y.V/'verify.py');native=n.native
 native.BUILD=HERE/'build/mutations';native.BUILD.mkdir(parents=True,exist_ok=True)
 def prepare(name,mutation):
  src=n.patched_prepare(name,None);file,old,new=mutation;p=src/file;s=p.read_text()
  assert s.count(old)==1,(name,old,s.count(old));p.write_text(s.replace(old,new));return src
 native.prepare=prepare
 cases={c['id']:c for c in y.fixtures.catalog()};rows=[];identity={str(p):y.sha(p) for p in HERE.glob('*.py')}
 for name,file,old,new,case in v.MUTATIONS:
  disposition='unchanged V compiled mutation'
  if name=='fb_file_cache':
   old='if (contents != previous_contents) { tdbnrl=-1; for(int q=0;q<16;q++)buf[q]=0; }'
   new='/* Y negative control: restore record-only cache across content changes. */'
   disposition='revert Y content correction; V forced reload no longer distinguishes this corrected fixture'
  if name=='calceph_pulsar':
   old='deltaT = psr[p].obsn[i].correctionTT_calcEph; // Y: current pulsar supplied value.'
   new='deltaT = psr[0].obsn[i].correctionTT_calcEph;'
   disposition='revert Y current-pulsar correction'
  exe,profile=native.compile_probe(name,(file,old,new));raw=y.native_launch(exe,profile,cases[case])
  baseline=y.project(y.native_payload(y.unpack(y.OUT/'native/equalpaths'/(case+'.json.gz'))))
  actual=y.project(y.native_payload(raw))
  try:y.compare(baseline,actual)
  except AssertionError as error:witness=str(error)
  else:raise AssertionError('no behavioral witness: '+name)
  assert baseline['events'][0]==actual['events'][0]
  out=y.OUT/'mutations'/(name+'.json.gz');y.pack(out,raw)
  rows.append(dict(name=name,case=case,disposition=disposition,witness=witness,profile=profile,path=str(out),sha256=y.sha(out)))
  print('PASS compiled mutation',name,witness[:200],flush=True)
 assert identity=={str(p):y.sha(p) for p in HERE.glob('*.py')}
 y.save(y.OUT/'mutations/receipt.json',dict(mutations=rows,authored_sources=identity,accepted_V_sha256=y.sha(y.V/'verify.py')))
if __name__=='__main__':main()
