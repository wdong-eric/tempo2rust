#!/usr/bin/env python3
"""Original/corrected focused 0053 witnesses, without replacing accepted evidence."""
import json,subprocess,os
import native_timing as n
B=n.HERE/'build/corrections';B.mkdir(parents=True,exist_ok=True)
identity=json.loads((n.BUILD/'identity.json').read_text());commands=[]
for variant in ['original','corrected']:
 objects=[p for p in identity['objects'] if variant=='corrected' or not p.endswith(('formResiduals.C.o','initialise.C.o'))]
 if variant=='original':
  # AC initialization corrections are inherited; AD defaults are omitted here.
  objects.append(str(n.ac.BUILD/'initialise.C.o'))
 exe=B/variant
 cmd=[n.ac.CC,'-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(n.ac.REF/'source'),str(n.HERE/'correction_probe.cc'),*objects,str(n.ac.REF/'libtempo2-reference.a'),'-lquadmath','-lgfortran','-o',str(exe)]
 subprocess.run(cmd,check=True,capture_output=True);commands.append(cmd)
 for fixture in ['high-spin','tzr']:
  for spare in ['0.25','4.75']:
   f=n.HERE/'fixtures';cmd=[str(exe),fixture,str(f/(fixture+'.par')),str(f/(fixture+'.tim')),spare]
   r=subprocess.run(cmd,env={**os.environ,'TEMPO2':str(n.ac.REF/'source/T2runtime'),'LC_ALL':'C'},capture_output=True,check=True)
   name=f'{variant}-{fixture}-{spare}';(B/(name+'.stdout')).write_bytes(r.stdout);(B/(name+'.stderr')).write_bytes(r.stderr)
   (B/(name+'.json')).write_text(json.dumps({'command':cmd,'executable_sha256':n.sha(exe),'stdout_sha256':n.sha(B/(name+'.stdout'))},indent=2)+'\n')
def values(variant,fixture,spare):
 return dict(line.split('\t',1) for line in (B/f'{variant}-{fixture}-{spare}.stdout').read_text().splitlines() if '\t' in line)
for f in ['high-spin','tzr']:
 a,b=values('corrected',f,'0.25'),values('corrected',f,'4.75')
 assert all(a[k]==b[k] for k in a if k.startswith('residualtn.'))
 x,y=values('original',f,'0.25'),values('original',f,'4.75')
 assert any(x[k]!=y[k] for k in x if k.startswith('residualtn.'))
 assert x['default.AverageDMResiduals']=='763' and x['default.nWhite_dm']=='41'
 assert a['default.AverageDMResiduals']=='0' and a['default.nWhite_dm']=='0'
 print(f,'original active noise mean depends on spare; corrected active mean is invariant')
(B/'identity.json').write_text(json.dumps({'commands':commands,'native_identity_sha256':n.sha(n.BUILD/'identity.json'),'probe_sha256':n.sha(n.HERE/'correction_probe.cc')},indent=2)+'\n')
