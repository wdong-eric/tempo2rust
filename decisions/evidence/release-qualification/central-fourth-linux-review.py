#!/usr/bin/env python3
"""Review the verified fourth Linux packet without rerunning scientific producers."""
from collections import Counter
from datetime import datetime, timezone
import hashlib,json,re,subprocess,sys,zipfile
from pathlib import Path
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust')
PACKET=ROOT/'decisions/evidence/release-qualification/build/linux-run-41294531402-review-20261101'
OUT=Path(sys.argv[1]);OUT.mkdir(parents=True,exist_ok=False)
PIN='fecbc18af95196a16db2046306099b6345a50976'
sha=lambda b:hashlib.sha256(b).hexdigest()
collection=json.loads((PACKET/'collection-receipt.json').read_bytes())
assert collection['candidate']==PIN and collection['stage_count']==69 and not collection['blocked']
assert (PACKET/'artifact.zip').stat().st_size==collection['archive_bytes']
selected={}
def separate_debug_strings(lines):
    remaining=[];strings={};section=None;label=None
    for line in lines:
        stripped=line.strip()
        if stripped.startswith(b'.section'):
            section=stripped.split(None,1)[1].split(b',',1)[0]
        elif stripped in (b'.text',b'.data',b'.bss'):
            section=stripped
        if section!=b'.debug_str':
            remaining.append(line);label=None;continue
        if stripped.startswith(b'.section'):continue
        if stripped.startswith(b'.LASF') and stripped.endswith(b':'):
            label=stripped;assert label not in strings;strings[label]=[];continue
        assert label is not None and stripped.startswith((b'.string',b'.ascii'))
        strings[label].append(stripped)
    assert all(strings.values())
    return remaining,{k:b'\n'.join(v) for k,v in strings.items()}
with zipfile.ZipFile(PACKET/'artifact.zip') as z:
    manifest_bytes=z.read('manifest.json')
    assert sha(manifest_bytes)==collection['manifest_sha256']
    manifest=json.loads(manifest_bytes)['files'];names=set(z.namelist())
    def read(name):
        data=z.read(name);assert sha(data)==manifest[name]['sha256']
        selected[name]=sha(data);return data
    def record(name):return json.loads(read(name))
    matrix=record('linux/receipt.json')
    assert matrix['candidate']==PIN and len(matrix['stages'])==69
    counts=Counter('blocked' if r['exit'] is None else 'pass' if r['exit']==0 else 'fail' for r in matrix['stages'].values())
    cache={}
    for label in ['default','combined']:
        prefix='linux/c-concurrency-'+label+'/'
        receipt=record(prefix+'receipt.json')
        installed=record('linux/install-'+label+'-build/install.json')
        assert installed['features']==('' if label=='default' else 'calceph,lapack')
        assert len(receipt['records'])==23 and all(r['exit']==0 for r in receipt['records'])
        assert receipt['sanitizer_products_equal'] is True
        cc=receipt['cache_control'];assert cc['production_exit']==cc['cleanup_exit']==0
        assert all(cc[k] is True for k in ['complete_workflows','workflow_output_equal','storage_equal'])
        bindings={}
        for path,digest in receipt['inputs'].items():
            if '/share/tempo2-rust/' in path:
                relative=path.split('/share/tempo2-rust/',1)[1]
                native=subprocess.check_output(['git','show',PIN+':'+relative],cwd=ROOT)
                assert sha(native)==digest==installed['files']['share/tempo2-rust/'+relative]
                bindings[relative]=digest
            elif '/install-'+label+'/' in path:
                relative=path.split('/install-'+label+'/',1)[1]
                assert installed['files'][relative]==digest
                bindings[relative]=digest
            else:
                relative=path.split('/tempo2rust/tempo2rust/',1)[1]
                native=subprocess.check_output(['git','show',PIN+':'+relative],cwd=ROOT)
                assert sha(native)==digest;bindings[relative]=digest
        modes={};products=[]
        for mode in ['plain','asan','tsan','asan-cache-cleanup']:
            raw=read(prefix+mode+'-execute.stdout')
            err=read(prefix+mode+'-execute.stderr')
            matches=re.findall(rb'AK_BEGIN (\d+) (\d+)\n(.*?)AK_END \1 \2\n',raw,re.S)
            rows={(int(i),int(j)):body for i,j,body in matches}
            assert len(matches)==len(rows)==12
            assert set(rows)=={(i,j) for i in range(4) for j in range(3)}
            assert raw.count(b'AK_BEGIN')==raw.count(b'AK_END')==12
            for marker in [b'ERROR: AddressSanitizer',b'runtime error:',b'WARNING: ThreadSanitizer',b'ERROR: LeakSanitizer']:
                assert marker not in err
            files={n.removeprefix(prefix+mode+'/'):sha(read(n)) for n in names if n.startswith(prefix+mode+'/') and n.endswith('.tsv')}
            assert len(files)==12
            assert all(receipt['files'][mode+'/'+k]==v for k,v in files.items())
            products.append((rows,files))
            modes[mode]={'workflow_count':len(rows),'storage_file_count':len(files),'stdout_sha256':sha(raw),'stderr_sha256':sha(err)}
        assert all(p==products[0] for p in products)
        cache[label]={'commands':23,'modes':modes,'source_bindings':bindings,'all_products_equal':True,'scope':receipt['scope']}
    cr=record('linux/contraction-source-profile/receipt.json')
    assert cr['status']=='pass' and cr['bounded_differences']==[]
    bounded=record('linux/contraction-source-profile/build/arithmetic/receipt.json')
    assert sha(read('linux/contraction-source-profile/build/arithmetic/receipt.json'))==cr['bounded_receipt']
    assert bounded['differences']==[]
    contractions={}
    for group,g in cr['groups'].items():
        products={}
        for unit,p in g['products'].items():
            dump='linux/contraction-source-profile/'+group+'-'+p['actual_translation_unit']+'.object.s'
            raw=read(dump);assert sha(raw)==p['object_disassembly']
            # Check actual disassembly plus both supplementary assembly products.
            # This records instruction evidence, not an automatic Rust edit rule.
            fma=re.findall(rb'\b(?:v?f(?:n?madd|n?msub|maddsub|msubadd)[a-z0-9]*|fma[fl]?)\b',raw)
            outputs=[]
            for mode in ['source','separate']:
                entry=p['supplementary_source_assembly'][mode]
                member=entry['path'].replace('/tmp/t2/','',1)
                b=read(member);assert sha(b)==entry['sha256']
                outputs.append([line for line in b.splitlines() if line.strip() and not line.lstrip().startswith(b'#')])
            code0,debug0=separate_debug_strings(outputs[0]);code1,debug1=separate_debug_strings(outputs[1])
            assert code0==code1 and debug0.keys()==debug1.keys()
            changed=[k for k in debug0 if debug0[k]!=debug1[k]]
            assert len(changed)==1
            key=changed[0]
            assert b'GNU C++11 13.3.0 ' in debug0[key]
            assert debug1[key].count(b' -ffp-contract=off')==1
            assert debug1[key].replace(b' -ffp-contract=off',b'')==debug0[key]
            products[unit]={'fma_mnemonics_or_symbols':sorted({m.decode() for m in fma}),
                            'assembly_equal_except_producer_debug_string_and_debug_string_order':True,
                            'noncomment_source_assembly_equal':outputs[0]==outputs[1],
                            'native_source_sha256':p['source'],'actual_object_sha256':p['actual_object']}
        contractions[group]={'compiler':g['compiler_version'].splitlines()[0],'flags':g['flags'],'products':products}
    native=record('linux/native-corpora/receipt.json');corpus=record('linux/corpus/receipt.json')
    assert native['status']=='pass' and len(native['stages'])==13 and not native['failed']
    assert not corpus['blocked'] and not corpus['source_mismatches']
    ledger={}
    for stage in matrix['failed']:
        path={'corpora-comparison':'corpus','m2-comparison':'m2-comparison'}.get(stage,stage)
        d=record('linux/'+path+'/receipt.json')
        if stage=='corpora-comparison': row={'stages':d['stages'],'failed':d['failed'],'blocked':d['blocked'],'source_mismatches':d['source_mismatches']}
        elif stage=='m2-comparison': row={'failures':d['failures'],'records':len(d['records']),'failed_records':[{'name':r['name'],'failures':r['failures']} for r in d['records'] if r['failures']]}
        elif stage=='fits':row={k:d[k] for k in ['failures','campaigns']}
        elif stage=='boundaries':row={'failures':d['failures'],'records':len(d['records']),'failed_records':[r for r in d['records'] if not r['equal']]}
        elif stage=='ddh':row={'fidelity_failures':d['fidelity_failures'],'historical_ideal_failures':d['historical_ideal_failures'],'records':d['records']}
        elif stage=='timing':row={k:d[k] for k in ['cases','failures']}
        elif stage=='event-diagnostic':row={'invariant_failures':d['invariant_failures'],'scientific_failures':d['scientific_failures'],'record_count':len(d['records']),'controls':len(d['product_controls'])}
        elif stage in ['event-source-profile','spin-source-profile']:row={'rows':len(d['rows']),'failures':len(d['failed']),'wide_differences':len(d.get('wide_differences',[]))}
        elif stage=='predictors':row={'cases':len(d['cases']),'checks':sum(r['checks'] for r in d['cases']),'failed':[{'name':r['name'],'kind':r['kind'],'failures':r['failures']} for r in d['cases'] if r['failures']]}
        elif stage=='output':row={'summary':d['summary'],'comparison':d['comparison']}
        ledger[stage]=row
    result={'scope':'Central saved-artifact review, no producer execution; scientific failures remain open',
            'candidate':PIN,'run':collection['run'],'collection_receipt_sha256':sha((PACKET/'collection-receipt.json').read_bytes()),
            'archive_sha256':collection['archive_sha256'],'manifest_sha256':collection['manifest_sha256'],
            'stage_counts':dict(counts),'cache':cache,'contraction':contractions,'native_stages':native['stages'],
            'failed_ledger':ledger,'selected_members':selected,'reviewer_sha256':sha(Path(__file__).read_bytes()),
            'completed_utc':datetime.now(timezone.utc).isoformat()}
    (OUT/'receipt.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({'stage_counts':dict(counts),'cache_profiles':list(cache),'contraction':contractions,'selected_members':len(selected),'receipt_sha256':sha((OUT/'receipt.json').read_bytes())},indent=2))
