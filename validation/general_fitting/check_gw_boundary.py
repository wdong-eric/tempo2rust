"""0053 cosmic-string pre-event initialization, with defined neighboring epochs."""
from pathlib import Path
import hashlib, json, re, subprocess
from fitting_corrections import corrected

ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
BUILD=HERE/'build/gw-boundary';BUILD.mkdir(parents=True,exist_ok=True)
REF=ROOT/'validation/reference/build';CC='/opt/homebrew/bin/g++-16'
PIN='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
old=subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',PIN+':t2fit_gw.C']).decode()
records=[];commands=[]
for version, source in [('original',old),('corrected',corrected('t2fit_gw.C',old))]:
    start=source.index('double t2FitFunc_gwcs_amp(');end=source.index('double t2FitFunc_quad_om(',start)
    block=source[start:end].replace('double extra;','double extra;bool ae_written=false;')
    block=re.sub(r'\bextra\s*=', 'ae_written=true,extra=', block)
    token='    if (k==0)';assert block.count(token)==1
    block=block.replace(token,'    extern void ae_gw_initialized(bool);ae_gw_initialized(ae_written);\n'+token)
    source=source[:start]+block+source[end:]
    src=BUILD/(version+'.C');src.write_text(source);obj=src.with_suffix('.o');exe=BUILD/version
    cmd=[CC,'-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(REF/'source'),'-c',str(src),'-o',str(obj)]
    subprocess.run(cmd,check=True,capture_output=True);commands.append(cmd)
    cmd=[CC,'-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(REF/'source'),str(HERE/'gw_boundary_probe.cc'),str(obj),str(REF/'libtempo2-reference.a'),'-lquadmath','-lgfortran','-o',str(exe)]
    subprocess.run(cmd,check=True,capture_output=True);commands.append(cmd)
    for epoch in ['53998','53999','53999.5','54000','54000.5','54001','54002']:
        p=subprocess.run([str(exe),epoch],capture_output=True)
        assert p.returncode==(86 if version=='original' and epoch=='53998' else 0),(version,epoch,p.stderr)
        rows=[l for l in p.stdout.decode().splitlines()if l.startswith(('VALUE','UNWRITTEN'))]
        if version=='corrected' and epoch=='53998':assert rows==['VALUE 0 0','VALUE 1 0']
        records.append({'version':version,'epoch':epoch,'exit':p.returncode,'rows':rows,'executable_sha256':sha(exe),'source_sha256':sha(src)})
for epoch in ['53999','53999.5','54000','54000.5','54001','54002']:
    assert next(r['rows']for r in records if r['epoch']==epoch and r['version']=='original')==next(r['rows']for r in records if r['epoch']==epoch and r['version']=='corrected')
(BUILD/'receipt.json').write_text(json.dumps({'pin':PIN,'builder_sha256':sha(__file__),'probe_sha256':sha(HERE/'gw_boundary_probe.cc'),'corrections_sha256':sha(HERE/'fitting_corrections.py'),'archive_sha256':sha(REF/'libtempo2-reference.a'),'commands':commands,'records':records},indent=2)+'\n')
print('original stops before unwritten pre-event value; corrected zero and six neighbors pass')
