#!/usr/bin/env python3
"""Seal already executed evidence and check current binaries and protected inputs."""
import collections
import gzip
import json
import subprocess
from pathlib import Path

from profile import HERE, ROOT, PIN, PRIMARY, blob, save, sha
from run import compare, execution_identity, load_results


def main():
    (HERE/'delivery.json').unlink(missing_ok=True)
    verification = HERE/'verification.json'
    if verification.exists():
        data = verification.read_bytes()
        (HERE/'verification.json.gz').write_bytes(gzip.compress(data, mtime=0))
    else:
        data = gzip.decompress((HERE/'verification.json.gz').read_bytes())
    v = json.loads(data)
    regeneration = json.loads((HERE/'regeneration.json').read_text())
    coverage = json.loads((HERE/'coverage.json').read_text())
    classification = json.loads((HERE/'coverage_classification.json').read_text())
    inspection = json.loads((HERE/'inspection.json').read_text())
    gold = load_results()
    ids = [r['id'] for r in gold]
    compare(ids, [o['case'] for o in coverage['observations']], 'coverage.membership')
    assert all(o['comparison'] == 'exact' for o in coverage['observations'])
    assert regeneration['complete_ordered_comparison']
    assert regeneration['cases'] == inspection['cases'] == len(gold)
    assert regeneration['events'] == inspection['events']
    assert inspection['all_delta_chains_valid']
    compare(regeneration['observations_sha256'], sha((HERE/'observations.json.gz').read_bytes()))
    compare(regeneration['raw_replay_sha256'], sha((HERE/'replay.raw.json.gz').read_bytes()))
    compare(classification['coverage_sha256'], sha((HERE/'coverage.json').read_bytes()))
    compare(classification['script_sha256'], sha((HERE/'classify_coverage.py').read_bytes()))
    compare([a['raw_sha256'] for a in inspection['raw_archives']],
            [sha((HERE/n).read_bytes()) for n in ['reference.raw.json.gz', 'replay.raw.json.gz']])
    assert all(a['lossless_reconstruction'] for a in inspection['raw_archives'])
    profiles = {'reference': json.loads((HERE/'profile.json').read_text()),
                'replay': regeneration['executed_profile'], 'verification': v['profile'],
                'coverage': coverage['profile']}
    post_initial = {}
    by_id = {r['id']: r for r in gold}
    for m in v['compiled_mutations']:
        profiles['mutation.'+m['name']] = m['profile']
        # Require a changed action/result after the exported initial constants.
        # Neither source filenames nor initial metadata alone can satisfy this.
        a, b = by_id[m['case']], m['result']
        try:
            compare({'events': a['events'][1:], 'outputs': a['outputs'], 'returncode': a['returncode']},
                    {'events': b['events'][1:], 'outputs': b['outputs'], 'returncode': b['returncode']})
        except AssertionError as e:
            post_initial[m['name']] = str(e)
        else:
            raise AssertionError('No post-initial sensitivity: '+m['name'])
    binaries = {}
    for name, p in profiles.items():
        binaries[name] = execution_identity(Path(p['executable_path']), p, True)
    protected = {}
    inputs = json.loads((HERE/'input_hashes.json').read_text())
    for name, expected in inputs.items():
        actual = sha((ROOT/name).read_bytes())
        if name.startswith('validation/'):
            compare(expected, actual, 'protected.'+name)
            protected[name] = actual
    nested = {}
    for name, expected in [('tempo2', PIN), ('validation/reference', '7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572')]:
        command = ['git', '-C', str(ROOT/name)]
        head = subprocess.check_output(command+['rev-parse', 'HEAD'], text=True).strip()
        status = subprocess.check_output(command+['status', '--porcelain'], text=True).strip()
        compare(expected, head, name+'.HEAD')
        assert not status, (name, status)
        nested[name] = {'head': head, 'status': status}
    sources = {name: sha(blob(name)) for name in PRIMARY+['ifteph.h']}
    for name, h in sources.items():
        compare(profiles['reference']['stable']['sources'][name], h, 'pinned.'+name)
    artifacts = {p.name: sha(p.read_bytes()) for p in sorted(HERE.iterdir())
                 if p.is_file() and p.name not in ['delivery.json', 'verification.json']}
    save(HERE/'delivery.json', dict(
        artifact_sha256=artifacts, current_executed_identities=binaries,
        source_pin=PIN, raw_primary_and_constant_sha256=sources,
        protected_artifact_sha256=protected, nested_repositories=nested,
        immutable_membership='The listed accepted K/M/N artifacts and clean pinned nested repositories. Active Q/R paths and mutable central coordination documents are outside membership.',
        cases=len(gold), actions=inspection['actions'], events=inspection['events'],
        classifications=dict(collections.Counter(r['classification'] for r in gold)),
        identity_controls=len(v['identity_controls']), comparison_controls=len(v['comparison_controls']),
        compiled_mutations=len(v['compiled_mutations']), post_initial_mutation_differences=post_initial,
        coverage_summary=coverage['summary'], coverage_gaps=classification['classification_counts'],
        acceptance='Native reference evidence delivered for central review. No Rust verification, scientific accuracy validation, numerical gate change or milestone acceptance.'))
    print('PASS sealed', len(artifacts), 'artifacts;', len(gold), 'cases;', len(post_initial), 'post-initial mutation witnesses')


if __name__ == '__main__':
    main()
