#!/usr/bin/env python3
"""Build the read-only-library observer and export independent M2 fixtures.
--create-inputs is only for reconstructing the declared deterministic fixtures.
Default runs verify frozen input hashes before exporting to reference/.
"""
import argparse
from decimal import Decimal, getcontext
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess

ROOT = Path(__file__).resolve().parent
PROJECT = ROOT.parent.parent
A2 = PROJECT / 'validation/reference'
RUNTIME = PROJECT / 'tempo2/T2runtime'
BUILD = ROOT / 'build'
REF = ROOT / 'reference'
INPUT = ROOT / 'inputs'
COMMANDS = []

def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

def save(p, value):
    p.write_text(json.dumps(value, indent=2, sort_keys=True)+'\n')

def run(cmd, name, timeout=60):
    trace = REF / (name+'.resources.tsv')
    trace.write_text('')
    r = subprocess.run(list(map(str,cmd)), cwd=BUILD, env={**os.environ,'TEMPO2':str(RUNTIME),'REFERENCE_RESOURCE_LOG':str(trace)}, stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True,timeout=timeout)
    (REF / (name+'.log')).write_text(r.stdout)
    COMMANDS.append({'command':list(map(str,cmd)),'returncode':r.returncode,'log':name+'.log'})
    return r

def create_inputs():
    getcontext().prec=70
    solvers=[]
    x=[-3,-1,0,2,5,9];noise=['3e-6','-2e-6','4e-6','1e-6','-5e-6','6e-6'];sigma=['1e-6','2e-6','1.5e-6','.7e-6','3e-6','1.1e-6']
    kinds=[('weighted_unscaled',0,1,0),('weighted_rescaled',0,1,1),('unweighted',0,0,1),('dependent_columns',1,0,1),('below_cutoff',2,0,0),('above_cutoff',3,0,0),('at_cutoff',5,0,0),('square_rescaled',4,0,1),('square_unscaled',4,0,0),('zero_column',6,0,1),('zero_matrix',7,0,0),('zero_weight',0,1,1),('underflow_weight',0,1,1),('underdetermined',8,0,0)]
    for name,kind,mode,rescale in kinds:
        n=2 if kind==4 else 1 if kind==8 else 6;m=2
        rows=[]
        for i in range(n):
            s=sigma[i] if kind==0 else '1'
            if name=='zero_weight' and i==0:s='0'
            if name=='underflow_weight' and i==0:s='1e-320'
            a0='1' if kind<2 or kind==8 else '1' if i==0 else '0'
            a1=str(x[i]) if kind==0 else '1' if kind in [1,8] else ({2:'1e-28',3:'1e-26',5:'1e-27',4:'1'}.get(kind,'0') if i==1 else '0')
            if kind==7:a0='0'
            rows.append(' '.join([noise[i],s,a0,a1]))
        path=INPUT/(name+'.txt');path.write_text(f'{n} {m} {mode} {rescale}\n'+'\n'.join(rows)+'\n')
        solvers.append({'name':name,'input':path.name,'expected':'boundary' if name in ['zero_weight','underflow_weight','underdetermined'] else 'completed'})
    # Deterministic defined overflow: covariance/errors and reported increments narrow to f64.
    for name,y,rescale in [('infinite_covariance','1e-200',0),('infinite_increment','1e200',0),('infinite_rescaled_error','1e-200',1)]:
        path=INPUT/(name+'.txt');path.write_text(f'2 1 0 {rescale}\n{y} 1 1e-200\n{y} 1 1e-200\n')
        solvers.append({'name':name,'input':path.name,'expected':'completed'})
    truth='PSRJ J0000-0000\nRAJ 01:00:00.00000\nDECJ -00:00:00.00000\nF0 10.123456789\nF1 -1e-15\nF2 2e-26\nF3 -3e-34\nPEPOCH 54000\nPOSEPOCH 54000\nDMEPOCH 54000\nDM 100\nEPHVER 5\nCLK TT(TAI)\nEPHEM DE405\nUNITS TCB\n'
    (INPUT/'truth.par').write_text(truth)
    epochs=[-90,-73,-55,-34,-17,-3,8,19,37,52,71,90,104,119,133,150]
    for site in ['@','pks']:
        name='bary' if site=='@' else 'pks'
        seed=INPUT/(name+'_seed.tim')
        seed.write_text('FORMAT 1\n'+''.join(f'toa{i} 1400 {54000+d}.125 1 {site}\n' for i,d in enumerate(epochs)))
        r=run([BUILD/'export_reference','align',INPUT/'truth.par',seed,'0',INPUT/(name+'.tim')],name+'_alignment')
        if r.returncode:raise RuntimeError(r.stdout)
    # An explicit permutation retains TOA identities and makes first-input phase semantics observable.
    lines=(INPUT/'bary.tim').read_text().splitlines();order=[7,0,15,2,11,1,14,5,9,3,13,4,12,6,10,8]
    (INPUT/'bary_unsorted.tim').write_text(lines[0]+'\n'+'\n'.join(lines[i+1] for i in order)+'\n')
    cases=[]
    for name,selected,mode,nits,rescale,tim in [
        ('zero',[],0,1,1,'bary.tim'),('f0',[0],1,1,0,'bary.tim'),('f1',[1],0,1,0,'bary.tim'),('f2',[2],1,1,1,'bary.tim'),('f3',[3],1,1,0,'bary.tim'),('subset',[0,2],1,2,1,'bary_unsorted.tim'),('all',[0,1,2,3],1,2,0,'bary.tim'),('all_mode0',[0,1,2,3],0,2,0,'bary.tim'),('pks_all',[0,1,2,3],1,2,1,'pks.tim'),('pks_all_mode0',[0,1,2,3],0,2,0,'pks.tim')]:
        par=truth
        vals=['10.12345678902','-9.998e-16','5e-26','3.7e-33']
        for k in range(4):
            old=next(s for s in truth.splitlines() if s.startswith(f'F{k} '))
            par=par.replace(old,f'F{k} {vals[k]} {int(k in selected)} 1e-40')
        # zero exercises defaults; subset exercises sequential MODE/WEIGHT overrides.
        if name!='zero':par+=('MODE 0\nWEIGHT 0\n' if name=='subset' else '')+f'MODE {mode}\nNITS {nits}\n'
        (INPUT/(name+'.par')).write_text(par)
        cases.append({'name':name,'par':name+'.par','tim':tim,'rescale':bool(rescale),'truth':'truth.par'})
    original=(A2/'fixtures/original/test2.tim').read_text().splitlines()
    (INPUT/'pks_seven.tim').write_text(original[0]+'\n'+'\n'.join(' '.join([f[0],f[1],f[2],str(1+i%5),f[4]]) for i,line in enumerate(original[1:]) if (f:=line.split()))+'\n')
    for mode in [0,1]:
        name=f'pks_seven_mode{mode}'
        par=(A2/'fixtures/original/test1.par').read_text().replace('F0             1.00','F0             1.00 1')+f'MODE {mode}\n'
        (INPUT/(name+'.par')).write_text(par)
        cases.append({'name':name,'par':name+'.par','tim':'pks_seven.tim','rescale':bool(mode)})
    # All x=0: zero derivative column, unchanged fitted derivative, finite ZERO solve.
    (INPUT/'epoch.tim').write_text('FORMAT 1\na 1400 54000 1 @\nb 1400 54000 2 @\nc 1400 54000 3 @\n')
    (INPUT/'zero_derivative.par').write_text(truth.replace('F3 -3e-34','F3 0 1')+'MODE 1\n')
    cases.append({'name':'zero_derivative','par':'zero_derivative.par','tim':'epoch.tim','rescale':False})
    # Square ZERO+F0 case: no accidental long-span timing effects; supplied uncertainty is metadata.
    (INPUT/'square.tim').write_text('FORMAT 1\na 1400 54000 1 @\nb 1400 54001 2 @\n')
    (INPUT/'square.par').write_text('PSRJ J0000-0000\nF0 1 1 0.25\nPEPOCH 54000\nUNITS TCB\nMODE 1\n')
    for scale in [False,True]:cases.append({'name':'timing_square_'+str(scale).lower(),'par':'square.par','tim':'square.tim','rescale':scale})
    (INPUT/'zero_error.tim').write_text((INPUT/'square.tim').read_text().replace('54000 1 @','54000 0 @'))
    (INPUT/'tiny_error.tim').write_text((INPUT/'square.tim').read_text().replace('54000 1 @','54000 1e-320 @'))
    save(ROOT/'cases.json',{'solver':solvers,'timing':cases})
    save(ROOT/'input_hashes.json',{p.name:sha(p) for p in sorted(INPUT.iterdir()) if p.is_file()})

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--create-inputs',action='store_true');args=parser.parse_args()
    BUILD.mkdir(exist_ok=True);REF.mkdir(exist_ok=True);INPUT.mkdir(exist_ok=True)
    assert subprocess.check_output(['git','-C',str(PROJECT/'tempo2'),'rev-parse','HEAD'],text=True).strip()=='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
    assert subprocess.check_output(['git','-C',str(A2),'rev-parse','HEAD'],text=True).strip()=='7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572'
    library=A2/'build/libtempo2-reference.a';assert sha(library)==json.loads((A2/'build_manifest.json').read_text())['static_library_sha256']
    compiler='/opt/homebrew/bin/g++-16'
    cmd=[compiler,'-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(A2/'build/source'),'-I'+str(A2),ROOT/'export_reference.cc',library,'-lquadmath','-lgfortran','-o',BUILD/'export_reference']
    r=run(cmd,'build');assert r.returncode==0,r.stdout
    if args.create_inputs:create_inputs()
    assert {p.name:sha(p) for p in sorted(INPUT.iterdir()) if p.is_file()}==json.loads((ROOT/'input_hashes.json').read_text())
    cases=json.loads((ROOT/'cases.json').read_text());boundaries={}
    for case in cases['solver']:
        name=case['name'];r=run([BUILD/'export_reference','solver',INPUT/case['input'],REF/(name+'.json')],name)
        if case['expected']=='completed':assert r.returncode==0,(name,r.stdout)
        else:boundaries[name]={'returncode':r.returncode,'log':r.stdout,'output':(REF/(name+'.json')).read_text()}
        print(name,r.returncode,flush=True)
    r=run([BUILD/'export_reference','callbacks','unused',REF/'callbacks.json'],'callbacks');assert r.returncode==0
    for case in cases['timing']:
        name=case['name'];r=run([BUILD/'export_reference','fit',INPUT/case['par'],INPUT/case['tim'],int(case['rescale']),REF/(name+'.json')],name)
        assert r.returncode==0,(name,r.stdout)
        output=REF/(name+'.json');doc=json.loads(output.read_text())
        doc['input_identity']={'par_sha256':sha(INPUT/case['par']),'tim_sha256':sha(INPUT/case['tim']),'par_text':(INPUT/case['par']).read_text(),'tim_text':(INPUT/case['tim']).read_text()}
        save(output,doc);print(name,r.returncode,flush=True)
    # Invalid weights reach source SVD before unsafe/termination; isolate each process.
    for name in ['zero_error','tiny_error']:
        r=run([BUILD/'export_reference','fit',INPUT/'square.par',INPUT/(name+'.tim'),1,REF/(name+'.partial')],name)
        boundaries[name]={'returncode':r.returncode,'log':r.stdout,'stage':'after whitening, within source SVD; partial output is not a numerical solution'}
    save(REF/'boundaries.json',boundaries)
    # Independently reproduce the unchanged central probe without replacing its evidence.
    cmd=[compiler,'-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(A2/'build/source'),'-I'+str(A2),PROJECT/'decisions/evidence/m2/source_probe.cc',library,'-lquadmath','-lgfortran','-o',BUILD/'central_probe']
    assert run(cmd,'central_build').returncode==0
    assert run([BUILD/'central_probe',REF/'central_probe.json'],'central_probe').returncode==0
    assert (REF/'central_probe.json').read_bytes()==(PROJECT/'decisions/evidence/m2/result.json').read_bytes()
    resources=json.loads((PROJECT/'rust/tests/terrestrial/manifest.json').read_text()) if (PROJECT/'rust/tests/terrestrial/manifest.json').exists() else None
    for path,entry in resources['resources'].items():assert sha(RUNTIME/path)==entry['sha256'],path
    manifest={'platform':platform.platform(),'compiler':subprocess.check_output([compiler,'--version'],text=True),'native_long_double_bits':53,'source_longdouble_bits':113,'library_sha256':sha(library),'a2_build_manifest_sha256':sha(A2/'build_manifest.json'),'exporter_sha256':sha(ROOT/'export_reference.cc'),'binary_sha256':sha(BUILD/'export_reference'),'cases_sha256':sha(ROOT/'cases.json'),'input_hashes_sha256':sha(ROOT/'input_hashes.json'),'source_hashes':{n:sha(PROJECT/'tempo2'/n) for n in ['t2fit.C','TKfit.C','TKsvd.C','t2fit_stdFitFuncs.C','formResiduals.C','T2toolkit.C','TKlongdouble.float128.h']},'commands':COMMANDS,'output_hashes':{p.name:sha(p) for p in sorted(REF.iterdir()) if p.name!='manifest.json' and p.is_file()},'runtime_resources':resources,'instrumentation':'No source changes. Additional formResiduals(...,0) observes raw residuals, then formResiduals(...,1). SVD re-call observes singular values; linearized residual and multiplier are named postprocessing.'}
    save(REF/'manifest.json',manifest)
if __name__=='__main__':main()
