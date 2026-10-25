#!/usr/bin/env python3
"""Index completed AD executions; this does not execute or accept numerical gates."""
import json
from pathlib import Path
import subprocess
import native_timing as n

B = n.HERE/'build'
components = json.loads((B/'components.json').read_text())
assert len(components['commands']) == 8
assert all(c['returncode'] == 0 for c in components['commands'])
current_sources = {str(p.relative_to(n.ROOT)):n.sha(p) for p in sorted((n.ROOT/'rust/src').rglob('*.rs'))}
assert components['source_sha256'] == current_sources
for c in components['commands']:
    log = B/f"{c['name']}-{c['profile']}-final.log"
    assert n.sha(log) == c['log_sha256']
    for path, digest in c['executable_sha256'].items():
        assert n.sha(Path(path)) == digest

public = {}
recomparisons = {}
for profile in ['debug','release']:
    rows = {}
    for path in sorted((B/f'public-{profile}').glob('*/comparison.json')):
        data = json.loads(path.read_text())
        assert not data['errors'] and data['native_returncode'] == data['rust_returncode'] == 0, path
        assert data['rust_source_sha256'] == current_sources, path
        assert n.sha(Path(data['rust_command'][0])) == data['rust_executable_sha256'], path
        assert n.sha(path.parent/'native-identity.json') == data['native_identity_sha256'], path
        assert n.sha(path.parent/'rust.json') == data['rust_output_sha256'], path
        fresh_path = B/f'recomparison-{profile}'/(path.parent.name+'.json')
        fresh = json.loads(fresh_path.read_text())
        assert fresh['original_comparison_sha256'] == n.sha(path)
        assert fresh['comparator_sha256'] == n.sha(n.HERE/'check_public.py')
        assert fresh['rechecker_sha256'] == n.sha(n.HERE/'recompare_public.py')
        assert not fresh['errors'] and not fresh['pipelines_rerun']
        recomparisons[profile+'/'+path.parent.name] = n.sha(fresh_path)
        rows[path.parent.name] = dict(comparison_sha256=n.sha(path),
                                     measurement_count=len(data['measurements']),
                                     rust_executable_sha256=data['rust_executable_sha256'])
    assert len(rows) == 130, len(rows)
    public[profile] = rows
assert public['debug'].keys() == public['release'].keys()
for name in public['debug']:
    d = json.loads((B/'public-debug'/name/'rust.json').read_text())
    r = json.loads((B/'public-release'/name/'rust.json').read_text())
    assert d['pulsars'] == r['pulsars'], name

protected = n.ROOT/'decisions/0008-preservation-policy-and-stage-plan.md'
assert n.sha(protected) == '2c5f81b82c43105fcd48b62139f1d78f2dc7c4089055ea162e9feef82ff4e12c'
subprocess.run(['git','diff','--exit-code','--','tempo2','validation/binary','validation/reference',
                'rust/tests/reference/outputs','rust/tests/terrestrial/outputs',str(protected)],
               cwd=n.ROOT, check=True, capture_output=True)
open_m2 = json.loads((B/'compatibility/m2-open-gates.json').read_text())
assert len(open_m2) == 4
assert all('shared/dependent_columns/1:' in s and
           ('strict_raw_retention_order' in s or 'strict_raw_singular_values[0]' in s) for s in open_m2)
logs = ['standard-debug-final.log','standard-release-final.log',
        'g-original-debug-final.log','g-original-release-final.log',
        'g-corrected-debug-final.log','g-corrected-release-final.log',
        'lifecycle-tests-debug-final.log','lifecycle-tests-release-final.log',
        'public-debug-final.log','public-release-final.log','cli-recovery.log',
        'corrections-final.log','cross-pulsar.log','compatibility.log','clippy-final.log','fmt-final.log',
        'comparator-controls.log','recomparison.log']
identities = ['native/identity.json','native/ac/identity.json','corrected-binary/identity.json',
              'corrections/identity.json','cross-pulsar/identity.json','components.json',
              'compatibility/commands.json','compatibility/m2-open-gates.json',
              'cli-recovery/debug/receipt.json','cli-recovery/release/receipt.json',
              'comparator-controls/receipt.json','recomparison-summary.json']
controls = json.loads((B/'comparator-controls/receipt.json').read_text())
assert controls['comparator_sha256'] == n.sha(n.HERE/'check_public.py')
assert len(controls['rejected_mutations']) == 11
assert all(c['errors'] for c in controls['rejected_mutations'])
receipt = dict(
    source_pin=n.ac.PIN,
    reviewed_baseline='18c906594595e18c92a59a83842d56bc2119d5de',
    interface_commit='8b1bdcb',
    head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=n.ROOT,text=True).strip(),
    rustc=subprocess.check_output(['rustc','-vV'],text=True),
    native_compiler=subprocess.check_output([n.ac.CC,'--version'],text=True),
    source_sha256=current_sources,
    validation_source_sha256={str(p.relative_to(n.ROOT)):n.sha(p) for p in sorted(n.HERE.iterdir()) if p.suffix in ['.py','.cc','.md']},
    fixtures_sha256={str(p.relative_to(n.ROOT)):n.sha(p) for p in sorted((n.HERE/'fixtures').rglob('*')) if p.is_file()},
    logs_sha256={p:n.sha(B/p) for p in logs},
    identities_sha256={p:n.sha(B/p) for p in identities},
    public=public,
    corrected_comparator_recomparison_sha256=recomparisons,
    public_scientific_projection_debug_release_exact=True,
    m2_unresolved=open_m2,
    other_inherited_gates=dict(P=dict(SIMUL=32,units=71,ordered_SAT=2,links=4,epochs=45),utility_0049=4,DDH_original_1ns='open',original_AB_differences=277),
    original_0008_sha256=n.sha(protected),
    original_G_sha256={str(p.relative_to(n.ROOT)):n.sha(p) for p in sorted((n.ROOT/'validation/binary/outputs').glob('*.json'))},
    note='Engineering execution receipts only. Central numerical/milestone acceptance remains pending; unchanged P full history was not replayed.'
)
(B/'handover.json').write_text(json.dumps(receipt,indent=2)+'\n')
print('handover', n.sha(B/'handover.json'))
print('130 public comparisons per profile; scientific projections identical between profiles')
print('M2 retains four original failures; all inherited gates remain explicit')
