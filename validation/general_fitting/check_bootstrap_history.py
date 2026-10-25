"""Terrestrial bootstrap partial-copy and pulsar-zero ownership histories."""
from pathlib import Path
from decimal import Decimal as D
import argparse,json,os,subprocess
from check_bootstrap import ROOT,HERE,build,sha

def run(profile):
    native,commands,identity,src=build();rust=ROOT/'rust/target'/profile/'examples/fitting_bootstrap'
    out=HERE/'build/bootstrap-history'/profile;out.mkdir(parents=True,exist_ok=True)
    runtime=ROOT/'validation/reference/build/source/T2runtime';base=HERE/'inputs/bootstrap.par'
    pars=[out/'first.par',out/'second.par'];pars[0].write_bytes(base.read_bytes())
    pars[1].write_text(base.read_text().replace('J0000-0000','J0000-0001').replace('10.12345678902','10.12345678904'))
    tim=out/'tagged.tim'
    tim.write_text('\n'.join(line if i==0 else line+f' -history {i}'for i,line in enumerate((ROOT/'validation/fitting/inputs/pks.tim').read_text().splitlines()))+'\n')
    records=[]
    for n in [1,2]:
        case=out/str(n);case.mkdir(exist_ok=True);dirs={e:case/e for e in ['native','rust']}
        for d in dirs.values():d.mkdir(exist_ok=True)
        trace=dirs['native']/'trace.tsv'
        cmds=[[str(native),str(trace),str(pars[0]),str(tim)], [str(rust),str(runtime),str(pars[0]),str(tim),str(dirs['rust'])]]
        if n==2:
            for cmd in cmds:cmd.extend([str(pars[1]),str(tim)])
        for e,cmd in zip(dirs,cmds):
            p=subprocess.run(cmd,cwd=dirs[e],env={**os.environ,'TEMPO2':str(runtime)},capture_output=True,timeout=120)
            (dirs[e]/'stdout').write_bytes(p.stdout);(dirs[e]/'stderr').write_bytes(p.stderr)
            assert p.returncode==0,(n,e,p.stderr.decode()[-1000:])
        got=json.loads((dirs['rust']/'stdout').read_text());native_rows={'before':[[]for _ in range(n)],'after':[[]for _ in range(n)]}
        lines=[line.split()for line in trace.read_text().splitlines()];failures=[]
        for r in lines:
            if r[0]!='O':continue
            _,stage,p,i,sat,bat,bbat,freq,ssb,err,deleted,name,nflag,*tail=r;nflag=int(nflag)
            flags=[tail[2*j:2*j+2]for j in range(nflag)];vectors=tail[nflag*2:];assert len(vectors)==12
            native_rows[stage][int(p)].append({'sat':sat,'bat':bat,'bbat':bbat,'frequency':freq,'frequency_ssb':ssb,'uncertainty':err,'deleted':int(deleted),'filename':name,'flags':flags,'earth':vectors[::2],'observatory':vectors[1::2]})
        for stage in ['before','after']:
            actual=got[stage+'_observations'];assert len(actual)==n
            for p in range(n):
                assert len(actual[p])==len(native_rows[stage][p])
                for i,(r,a)in enumerate(zip(native_rows[stage][p],actual[p])):
                    for key in ['filename','flags','deleted']:assert a[key]==r[key],(stage,p,i,key)
                    for key in ['sat','bat','bbat','frequency','frequency_ssb','uncertainty']:
                        ref=D(r[key]);value=D(str(a[key]));gate=D('1e-12')/86400 if key in ['sat','bat','bbat'] else D('5e-14')*abs(ref)
                        if abs(ref-value)>gate:failures.append([stage,p,i,key,str(ref-value)])
                    for key in ['earth','observatory']:
                        for j,(v,w)in enumerate(zip(r[key],a[key])):
                            gate=D('1e-12'if j<3 else'1e-15')
                            if abs(D(v)-D(str(w)))>gate:failures.append([stage,p,i,key,j,str(D(v)-D(str(w)))])
        samples=[tuple(map(int,r[1:]))for r in lines if r[0]=='S']
        assert samples==[(it,i,k)for it,v in enumerate(got['iterations'])for i,k in enumerate(v['sampled_indices'])]
        # Check the source partial-copy contract independently in each engine.
        for engine,history in [('native',native_rows),('rust',{s:got[s+'_observations']for s in ['before','after']})]:
            before,after=history['before'],history['after'];p=n-1
            if n==2:assert before[0]==after[0],(engine,'pulsar zero changed')
            for i,k in enumerate(got['iterations'][-1]['sampled_indices']):
                for field in ['sat','bat','frequency','frequency_ssb','uncertainty','deleted','filename']:
                    assert after[p][i][field]==before[p][k][field],(engine,'copied',field,i,k)
                for field in ['bbat','flags']:assert after[p][i][field]==before[p][i][field],(engine,'retained',field)
                for field in ['earth','observatory']:
                    assert after[p][i][field][:3]==before[p][k][field][:3]
                    assert after[p][i][field][3:]==before[p][i][field][3:]
        scale=D(160)*86400/D('10.12345678902')
        for r in lines:
            if r[0] not in ['P','F']:continue
            values=got['values']if r[0]=='F'else got['iterations'][int(r[1])]['fitted_values']
            if abs((D(values[0])-D(r[4]))*scale)>D('1e-11'):failures.append(['physical_value',r[0],r[1]])
            if r[0]=='F'and abs((D(got['errors'][0])-D(r[5]))*scale)>D('1e-11')+D('1e-6')*abs(D(r[5])*scale):failures.append(['physical_error'])
        assert (dirs['native']/'testout.tim').read_bytes()==(dirs['rust']/'testout.tim').read_bytes()
        records.append({'pulsars':n,'commands':cmds,'failures':failures});print(n,'pulsars; history failures',failures,flush=True)
    (out/'receipt.json').write_text(json.dumps({'profile':profile,'commands':commands,'inputs':{str(p):sha(p)for p in pars+[tim]},'native_sha256':sha(native),'rust_sha256':sha(rust),'builder_sha256':sha(__file__),'records':records},indent=2)+'\n')

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--profile',choices=['debug','release'],default='debug');run(p.parse_args().profile)
