#!/usr/bin/env python3
"""Fixture construction and inventory for TEMPO2 *model.C/readParfile.C.
No timing calculation is reproduced here. Decimal arithmetic only places TOAs.
"""
import hashlib
import json
from decimal import Decimal, getcontext
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parent
PROJECT = ROOT.parents[1]
MODELS = ['BT','BTJ','BTX','ELL1','DD','DDK','DDS','MSS','DDGR','T2','T2-PTA','DDH','ELL1H','ELL1k']


def save(path, value):
    path.write_text(json.dumps(value, indent=2, sort_keys=True)+'\n')


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def uncomment(text):
    return re.sub(r'/\*.*?\*/|//[^\n]*', lambda m:'\n'*m[0].count('\n'), text, flags=re.S)


def inventory():
    rows = {}
    for model in MODELS:
        filename=model+'model.C';text=(PROJECT/'tempo2'/filename).read_text();code=uncomment(text)
        start=code.index('void update'+model.replace('-','_'))
        body=code[:start];update=code[start:];update=update[:update.find('\n}\n')+3]
        derivatives=sorted(set(re.findall(r'\bparam\s*==\s*(param_\w+)',body)))
        updates=sorted(set(re.findall(r'\bpos\s*==\s*(param_\w+)',update)))
        reads=sorted(set(re.findall(r'\bparam_\w+',code)))
        # Every source if-condition is retained, including helper/optional gates.
        conditions=[]
        for m in re.finditer(r'\bif\s*\(',code):
            pos=m.end();depth=1;end=pos
            while depth:
                depth+=(code[end]=='(')-(code[end]==')');end+=1
            expression=' '.join(code[pos:end-1].split())
            conditions.append({'line':code[:m.start()].count('\n')+1,'condition':expression,
                               'coverage':'inventory_only_not_branch_instrumented'})
        rows[model]={'source':filename,'sha256':sha(PROJECT/'tempo2'/filename),
                     'native_return':'binary128' if model=='DD' else 'binary64',
                     'wrapper_return':'binary64','derivative_labels':derivatives,'update_labels':updates,
                     'update_default':'add_to_index_zero_for_any_other_label' if model=='T2-PTA' else 'no_op',
                     'parameter_reads':reads,'conditions':conditions}
    return rows


def main():
    getcontext().prec=70
    inputs=ROOT/'fixtures';inputs.mkdir(exist_ok=True)
    inv=inventory()
    base={'PSRJ':'J0102-2030','RAJ':'01:02:03.123456789012345','DECJ':'-20:30:40.123456789012345',
          'F0':'123.456789012345678901234567890123','F1':'-1.234567890123456789e-15',
          'F2':'2.345678901234567890e-26','F3':'-3.45678901234567890e-36',
          'PEPOCH':'54000.1250000000000000000000000000','POSEPOCH':'54000.1250000000000000000000000000',
          'DMEPOCH':'54000.1250000000000000000000000000','DM':'12.34567890123456789',
          'EPHVER':'5','CLK':'TT(TAI)','EPHEM':'DE405','UNITS':'TCB'}
    eccentric={'PB':'2.3456789012345678901234567890123','A1':'1.234567890123456789012345678901',
               'ECC':'0.12345678901234567890123456789','OM':'73.12345678901234567890123456789',
               'T0':base['PEPOCH']}
    lowecc={k:v for k,v in eccentric.items() if k in ['PB','A1']}
    lowecc.update(TASC=base['PEPOCH'],EPS1='0.00012345678901234567890123456789',EPS2='-0.0002345678901234567890123456789')
    secular={'PBDOT':'1.234567890123456789e-13','A1DOT':'2.34567890123456789e-15',
             'OMDOT':'0.01234567890123456789','EDOT':'-1.234567890123456789e-17','GAMMA':'0.0001234567890123456789',
             'XPBDOT':'-2.34567890123456789e-14'}
    shapiro={'SINI':'0.678901234567890123456789','M2':'0.3456789012345678901234567'}
    harmonic={'H3':'0.0000001234567890123456789','STIG':'0.45678901234567890123'}
    motion={'KIN':'57.1234567890123456789','KOM':'123.4567890123456789','PMRA':'3.1234567890123456789',
            'PMDEC':'-5.234567890123456789','PX':'0.4567890123456789'}
    epoch=Decimal(base['PEPOCH']);pb=Decimal(eccentric['PB'])
    phases=['0.37','-2','-1.25','-1','-0.000000000001','0','0.125','0.25','0.5','0.75','1','2.25']
    for site,filename in [('@','barycentric.tim'),('pks','pks.tim')]:
        (inputs/filename).write_text('FORMAT 1\n'+''.join(f'toa{i:02d} 1400.123456789012345 {epoch+pb*Decimal(f)} 1.23456789012345 {site}\n' for i,f in enumerate(phases)))
    (inputs/'epochs.json').write_text(json.dumps({'t0_tasc_mjd':str(epoch),'pb_days':str(pb),'original_order_orbit_offsets':phases},indent=2)+'\n')
    cases=[]
    # Request language: op label subindex observation repeat increment error source_semantics.
    def add(name,model,values,derivatives=None,updates=None,tim='barycentric.tim',residual=True,undefined=False,extra=None,notes=''):
        full={**base,'BINARY':model,**values}
        path=inputs/(name+'.par')
        # Input uncertainties initialize prefitErr; source readValue sets err to zero.
        # Sequential update probes separately reveal error addition versus overwrite.
        controls={'PSRJ','BINARY','EPHVER','CLK','EPHEM','UNITS'}
        lines=[]
        for key,value in full.items():
            suffix='' if key in controls or key.startswith('ORBIFUNC') or value in ['KIN','STIG','PBDOT'] else (' 1' if key in values else ' 0')+' 0.000000000003125'
            lines.append(f'{key} {value}{suffix}\n')
        path.write_text(''.join(lines))
        labels=sorted(set(inv[model]['parameter_reads'])|{'param_f','param_raj','param_decj','param_pepoch','param_posepoch','param_dmepoch','param_dm','param_track'})
        (inputs/(name+'.labels')).write_text('\n'.join(labels)+'\n')
        req=[]
        for i in range(len(phases)):
            req.append(f'native delay 0 {i} 1 0 0 '+('undefined_source' if undefined else 'defined'))
        if derivatives is not None:
            for label,k in derivatives:
                for i in [0,2,5,7,9]:
                    for op in ['native','wrapper']:
                        req.append(f'{op} {label} {k} {i} 1 0 0 '+('undefined_source' if undefined or model=='DDK' else 'defined'))
        if updates is not None:
            for label,k in updates:
                for error in ['0.00000003125','-1','NaN']:
                    for op in ['update_native','update_wrapper']:
                        req.append(f'{op} {label} {k} 0 1 0.000000125 {error} defined')
        if residual:
            for mean in [0,1]:req.append(f'residual delay {mean} 0 1 0 0 defined')
        req+=extra or []
        (inputs/(name+'.requests')).write_text('\n'.join(req)+'\n')
        cases.append({'name':name,'model':model,'par':path.name,'tim':tim,'labels':name+'.labels','requests':name+'.requests',
                      'integrated':residual,'source_semantics':'undefined_source' if undefined else 'defined',
                      'notes':notes,'geometry':'source formBatsAll: vectorPulsar, readEphemeris DE405, calculate_bclt; no supplied vectors',
                      'reference':'first eligible TOA in original order; no preprocessing/sort/cuts/fit; REFPHS_TZR inactive'})
    values={}
    for model in MODELS:
        low=model in ['ELL1','ELL1H','ELL1k']
        v={**(lowecc if low else eccentric)}
        if model=='BTX':v.pop('PB');v['FB0']='0.000004934210547347654006925016237054'
        if model=='DDH' or model=='ELL1H':v.update(harmonic)
        if model=='DDGR':v.update(MTOT='1.98765432109876543210',M2=shapiro['M2'])
        if model=='DDK':v.update(motion)
        # T2-PTA needs both Shapiro and Kopeikin activation to initialize every derivative local.
        if model=='T2-PTA':v.update(motion,**shapiro)
        minimum=v.copy()
        v.update(secular)
        if low:
            v.pop('EDOT',None);v.pop('GAMMA',None);v.pop('XPBDOT',None)
            v.update(EPS1DOT='1.234567890123456789e-17',EPS2DOT='-2.34567890123456789e-17')
        if model not in ['BT','BTJ','BTX','DDH','ELL1H']:v.update(shapiro)
        if model=='DDS':v['SHAPMAX']='1.234567890123456789'
        if model=='MSS':v.update(X2DOT='0.01234567890123456789',OM2DOT='0.0234567890123456789',ORBPX='0.4567890123456789',SHAPMAX='0.000000123456789',A0='0.00000023456789',B0='-0.00000012345678',DR='0.0000123456789',DTHETA='0.000023456789')
        if model=='DD':v.update(DR='0.0000123456789',DTHETA='0.000023456789')
        if model=='T2':v.update(motion,PB2DOT='1.23456789e-24',X2DOT='1.23456789e-25',DTH='0.0000123456789',DR='0.000023456789',A0='1.23456789e-7',B0='-2.3456789e-7')
        deriv=[(l,0) for l in inv[model]['derivative_labels']]
        updates=[(l,0) for l in sorted(set(inv[model]['update_labels'])|set(inv[model]['derivative_labels']))]
        if model=='T2':
            deriv=[(l,k) for l,k in deriv if l not in ['param_eps1','param_eps2','param_eps1dot','param_eps2dot','param_shapmax','param_h3','param_h4','param_stig']]
        # ELL1H exact mode initializes h4 from the parameter array, including its absent zero.
        values[model]=v
        add(model.lower()+'_baseline',model,v,deriv,updates,notes='All listed callbacks start from the same saved prepared state; binary64 callback increments/errors parsed by strtod.')
        add(model.lower()+'_defaults',model,minimum,residual=model!='DDK',notes='Optional values absent; required model inputs retained. T2-PTA retains derivative prerequisites even though no derivatives are requested.')
    # Model-specific optional branches and source mutations.
    for model in ['DD','ELL1','ELL1k','T2']:
        add(model.lower()+'_sini_clamp',model,{**values[model],'SINI':'1.01234567890123456789'},residual=model not in ['ELL1','ELL1k'],extra=['native delay 0 0 2 0 0 defined'],notes='Fresh calls and two sequential calls expose SINI overwrite and warning suppression.')
    add('t2_sini_negative','T2',{**values['T2'],'SINI':'-1.1'},extra=['native delay 0 0 2 0 0 defined'])
    add('t2_ecc_clamp','T2',{**values['T2'],'ECC':'-0.1'},extra=['native delay 0 0 2 0 0 defined'])
    add('ddgr_derived','DDGR',values['DDGR'],extra=['native derived 0 0 2 0 0 defined','native delay 0 0 2 0 0 defined'])
    add('ell1_sini_rounding','ELL1',{**values['ELL1'],'SINI':'1.0000000000000000000001'},residual=False,notes='Narrowing to double removes this tiny excess before the clamp comparison.')
    add('bt_edot_update','BT',values['BT'],extra=['update_delay param_edot 0 0 2 0.000000125 0.00000003125 defined'])
    add('ell1h_error_accumulation','ELL1H',values['ELL1H'],extra=['update_native param_a1dot 0 0 2 0.000000125 0.00000003125 defined','update_wrapper param_a1dot 0 0 2 0.000000125 0.00000003125 defined'])
    add('ell1h_missing_h3','ELL1H',{k:v for k,v in values['ELL1H'].items() if k!='H3'},notes='logerr without exit; exact STIG path with initialized zero H3.')
    add('bt_edot_zero','BT',{**values['BT'],'EDOT':'0'},notes='Compare delays with BT baseline: local edot is hardwired zero despite derivative/update routes.')
    add('ell1k_epsdot_zero','ELL1k',{**values['ELL1k'],'EPS1DOT':'0','EPS2DOT':'0'})
    for tag,fb in [('fb_gapped',{'FB0':'0.000004934210547347654006925016237054','FB2':'1.234567890123456789e-25'}),('fb_explicit',{'FB0':'0.000004934210547347654006925016237054','FB1':'-2.34567890123456789e-18','FB2':'1.234567890123456789e-25'})]:
        for model in ['BTX','ELL1']:
            add(model.lower()+'_'+tag,model,{**values[model],**fb},[('param_fb',k) for k in [0,1,2]]+([('param_pbdot',0)] if model=='ELL1' else []),[('param_fb',k) for k in [0,1,2]],notes='ELL1 explicit FB1 disables PBDOT during parsing; gapped FB2 keeps the PBDOT-derived FB1 term.')
    jumps={'BPJEP_1':str(epoch),'BPJPH_1':'0.123456789','BPJA1_1':'0.000123456789','BPJEC_1':'0.0000123456789','BPJOM_1':'0.00123456789','BPJPB_1':'0.0000123456789',
           'BPJEP_2':str(epoch+pb/2),'BPJPH_2':'-0.0123456789','BPJA1_2':'0.000023456789','BPJEC_2':'-0.0000023456789','BPJOM_2':'-0.00023456789','BPJPB_2':'-0.0000023456789'}
    for model in ['BTJ','T2']:
        labs=[(l,k) for l in ['param_bpjph','param_bpja1','param_bpjec','param_bpjom','param_bpjpb'] for k in [0,1]]
        add(model.lower()+'_jumps',model,{**values[model],**jumps},labs,labs,notes='Strict bbat > BPJEP; equality and both sides included. T2 arr=1 with one companion returns zero: jump-array and companion indices share arr.')
    second={'PB_2':'12.34567890123456789','A1_2':'2.34567890123456789','ECC_2':'0.234567890123456789','OM_2':'123.4567890123456789','T0_2':'53997.1234567890123456789','PBDOT_2':'1.23456789e-14','XDOT_2':'2.3456789e-15','X2DOT_2':'3.456789e-26'}
    labs=[(l,k) for l in ['param_pb','param_a1','param_ecc','param_om','param_t0','param_pbdot','param_a1dot','param_a2dot'] for k in [0,1]]
    add('t2_companions','T2',{**values['T2'],**second},labs,residual=False,undefined=True,notes='Two-companion route reaches getParameter k == aSize for scalar optional arrays: source guard is > rather than >=. Delay/derivative values withheld. Updates are isolated in t2_companion_updates. Third/fourth companions remain unexercised.')
    add('t2_companion_updates','T2',{**values['T2'],**second},updates=labs,residual=False,notes='Only in-bounds updates, in an independent process without entering getParameter or model delay/derivative arithmetic.')
    path=inputs/'t2_companion_updates.requests'
    path.write_text('\n'.join(line for line in path.read_text().splitlines() if line.startswith('update_'))+'\n')
    low={**lowecc,**shapiro,'EPS1DOT':'1.23456789e-17','EPS2DOT':'-2.3456789e-17'}
    lowlabs=[(l,0) for l in ['param_pb','param_a1','param_eps1','param_eps2','param_eps1dot','param_eps2dot','param_tasc','param_pbdot','param_a1dot','param_sini','param_m2']]
    add('t2_ell1','T2',low,lowlabs,lowlabs)
    add('t2_ell1_kopeikin','T2',{**low,**motion},lowlabs+[('param_kin',0),('param_kom',0)])
    add('t2_ell1h_exact','T2',{**low,**harmonic},[('param_h3',0),('param_stig',0)])
    add('t2_gr','T2',{**values['T2'],'MTOT':'1.9876543210987654321','XOMDOT':'0.000123456789','AFAC':'0.0123456789'})
    add('t2_shapmax','T2',{**values['T2'],'SHAPMAX':'1.23456789'},[('param_shapmax',0)])
    add('t2_distance','T2',{**values['T2'],'D_AOP':'1.23456789'})
    add('t2_linked_sini','T2',{**values['T2'],'SINI':'KIN'},[('param_kin',0),('param_sini',0)])
    add('t2_beta_warning','T2',{**values['T2'],'BP':'0.123456789','BPP':'0.23456789'},notes='Source prints beta model not implemented, continues unchanged.')
    add('pks_dd','DD',values['DD'],tim='pks.tim',notes='Ordinary PKS propagation boundary with the accepted DE405/IF99/TT(TAI) profile and source weather fallback.')
    add('dd_no_dispatch','DD',{'SINI':'0.5'},notes='BINARY alone does not dispatch formResiduals; direct delay omitted below.')
    # No PB direct arithmetic is undefined through integer cast; only residual dispatcher is invoked.
    (inputs/'dd_no_dispatch.requests').write_text('residual delay 0 0 1 0 0 defined\nresidual delay 1 0 1 0 0 defined\n')
    # Undefined-source and terminating cases have their own processes, with no integrated residual cast of NaN.
    add('ddh_stig_zero','DDH',{**values['DDH'],'STIG':'0'},residual=False,notes='Defined floating-point nonfinite delay; no formResiduals integer conversion.')
    add('ddh_zero_h3_stig','DDH',{**values['DDH'],'STIG':'0','H3':'0'},residual=False,notes='Defined 0/0 Shapiro range; NaN withheld from integrated integer pulse conversion.')
    add('dd_ecc_exit','DD',{**values['DD'],'ECC':'1.1'},residual=False)
    add('bt_ecc_exit','BT',{**values['BT'],'ECC':'-0.1'},residual=False)
    add('ddh_missing_stig','DDH',{k:v for k,v in values['DDH'].items() if k!='STIG'},residual=False)
    add('ddgr_negative_mass','DDGR',{**values['DDGR'],'MTOT':'-1'},residual=False)
    add('ddgr_fb_rejected','DDGR',{**values['DDGR'],'FB0':'0.0000049'},residual=False)
    add('t2_missing_epoch','T2',{k:v for k,v in values['T2'].items() if k!='T0'},residual=False)
    add('t2pta_missing_t0','T2-PTA',{k:v for k,v in values['T2-PTA'].items() if k!='T0'},residual=False)
    for name,model,v in [('ell1h_h3_only','ELL1H',{k:v for k,v in values['ELL1H'].items() if k!='STIG'}),
                         ('ell1h_h4_nharm','ELL1H',{**values['ELL1H'],'H4':'0.000000056789','NHARM':'7'}),
                         ('t2_ell1h_h4','T2',{**low,**harmonic,'H4':'0.000000056789','NHARM':'7'})]:
        add(name,model,v,[(l,0) for l in ['param_h3','param_h4','param_stig']],residual=False,undefined=True,notes='Reads uninitialized stig (also si/m2 for ELL1H H3-only). Returned numbers withheld. NHARM mode=3 is overwritten by mode=2 in source.')
    add('ell1_orbifunc_linear','ELL1',{**values['ELL1'],'SORBIFUNC':'2','ORBIFUNC1':'53990 0.0001 0.00001','ORBIFUNC2':'54000 0.0002 0.00001','ORBIFUNC3':'54010 -0.0001 0.00001'},notes='Source linear ORBFUNC branch; native sample arrays parsed as double.')
    add('ell1_orbifunc_sinc','ELL1',{**values['ELL1'],'SORBIFUNC':'1','SIFUNC':'1','ORBIFUNC1':'53990 0.0001 0.00001','ORBIFUNC2':'54000 0.0002 0.00001','ORBIFUNC3':'54010 -0.0001 0.00001'},residual=False,notes='Sinc branch tests param_ifunc rather than param_orbifunc; no integrated spin IFUNC assertion.')
    add('ell1_orbifunc_step_undefined','ELL1',{**values['ELL1'],'SORBIFUNC':'0','ORBIFUNC1':'53990 0.0001 0.00001','ORBIFUNC2':'54000 0.0002 0.00001'},residual=False,undefined=True,notes='Step branch loops to ifuncN rather than orbifuncN; ifuncN=0 leaves ival uninitialized. Values withheld.')
    # ELL1H exact H4 derivative uses h4 initialized from parameter zero, so defined; include independently.
    # DDK derivatives are isolated from the defined baseline delay and residual evidence.
    add('ddk_derivatives_undefined','DDK',values['DDK'],[(l,0) for l in inv['DDK']['derivative_labels']],residual=False,undefined=True,notes='DDKmodel.C dtdpx = dtdpx - daopx_dpx reads an uninitialized local before derivative dispatch. Values withheld even if optimized away.')
    for c in cases:
        if c['name']=='ddk_baseline':
            path=inputs/c['requests'];path.write_text('\n'.join(line for line in path.read_text().splitlines() if not (line.startswith(('native param_','wrapper '))))+'\n')
    for filename in ['test3.par','test3.tim']:
        (inputs/('original_'+filename)).write_bytes((PROJECT/'tempo2/tests/test_data'/filename).read_bytes())
    save(ROOT/'cases.json',cases)
    save(ROOT/'source_inventory.json',inv)
    save(ROOT/'input_hashes.json',{p.name:sha(p) for p in sorted(inputs.iterdir()) if p.is_file()})
    print(f'{len(cases)} cases, {len(inv)} models')

if __name__=='__main__':main()
