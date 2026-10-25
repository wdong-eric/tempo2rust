#!/usr/bin/env python3
"""Reuse complete P projections in owned staging; preserve native evidence."""
import argparse,gzip,hashlib,json,os,re,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];P=ROOT/'validation/preprocessing';BUILD=HERE/'build/preprocessing'
def sha(b):return hashlib.sha256(b).hexdigest()
def main():
    ap=argparse.ArgumentParser();ap.add_argument('--release',action='store_true');ap.add_argument('--case',action='append');ap.add_argument('--generate',action='store_true');a=ap.parse_args()
    BUILD.mkdir(parents=True,exist_ok=True)
    profile=json.loads((P/'profile.json').read_text());inc=next(x[2:] for x in profile['command'] if x.startswith('-I'))
    initial=json.loads(gzip.decompress((P/'initial.json.gz').read_bytes()))
    if a.generate:
        fields=sorted(k.removeprefix('p0.') for k in initial['state'] if k.startswith('p0.') and not k.startswith(('p0.param.','p0.obs')) and k!='p0.caller_capacity')
        source='// Generated from the accepted P complete pulsar-field membership.\nfn fixed(p:&mut SourcePulsar,root:&str,s:&mut Map,load:bool){\n'
        source+=''.join('field(&mut p.'+f+',&format!("{root}'+f+'"),s,load);\n' for f in fields)+'}\n'
        (ROOT/'rust/src/preprocessing_corpus_fields.rs').write_text(source);return
    layout=initial['abi']['layout'];ranges={}
    widths={'longdouble':16,'double':8,'float':4}
    types=json.loads((P/'TYPES.json').read_text())
    for kind in ['observation','pulsar']:
        for full,span in layout.items():
            if not full.startswith(kind+'.'):continue
            name=full.split('.',1)[1]
            for declaration in types[kind]:
                match=re.match(r'(longdouble|double|float)\s+',declaration)
                if match and re.search(r'\b'+re.escape(name)+r'\b',declaration):
                    ranges[full]=[span['offset'],span['bytes'],widths[match[1]]];break
    (BUILD/'floating-layout.json').write_text(json.dumps(ranges)+'\n')
    (BUILD/'initial.json').write_text(json.dumps(initial)+'\n')
    cases=json.loads(gzip.decompress((P/'observations.json.gz').read_bytes()));selected=[];dispositions=[]
    for c in cases:
        if a.case and c['id'] not in a.case:continue
        if any(x[0]=='plugins' for x in c['actions']):
            dispositions.append({'case':c['id'],'disposition':'native external callback/plugin ABI outside AC release; Rust explicitly rejects source selection plugin requests'})
            continue
        work=BUILD/c['id'];work.mkdir(exist_ok=True)
        for name,raw in c['files'].items():
            target=work/name;target.parent.mkdir(exist_ok=True,parents=True);target.write_bytes(bytes.fromhex(raw))
        # Retain the original native source directory in diagnostics. Resource files
        # are the accepted read-only P caller fixtures; authored inputs use AC paths.
        c=json.loads(json.dumps(c).replace('{case}',str(work)).replace('{build}',str(P/'build')).replace('{source}',inc))
        c['workdir']=str(work);selected.append(c)
    (BUILD/'cases.json').write_text(json.dumps(selected)+'\n')
    env={**os.environ,'CARGO_HOME':str(ROOT/'rust/.cargo-home'),'GMP_MPFR_SYS_CACHE':str(ROOT/'rust/target/gmp-cache'),'AC_PREPROCESS_CORPUS':str(BUILD)}
    cmd=['cargo','test','--manifest-path',str(ROOT/'rust/Cargo.toml'),'--locked','--offline',*(['--release'] if a.release else []),'--lib','preprocessing_corpus_tests::p_corpus','--','--ignored','--exact','--nocapture','--test-threads=1']
    run=subprocess.run(cmd,env=env)
    (BUILD/('receipt-'+('release' if a.release else 'debug')+'.json')).write_text(json.dumps({'reference_sha256':sha((P/'observations.json.gz').read_bytes()),'initial_sha256':sha((P/'initial.json.gz').read_bytes()),'cases':len(selected),'dispositions':dispositions,'returncode':run.returncode},indent=2)+'\n')
    raise SystemExit(run.returncode)
if __name__=='__main__':main()
