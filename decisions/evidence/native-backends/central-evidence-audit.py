"""Read-only central audit of AI's completed delivery and isolated review."""
from pathlib import Path
import hashlib
import json
import subprocess

ROOT = Path('/Users/Eric.Dong/Downloads/tempo2rust')
BASE = ROOT / 'validation/native_backends/build'
OUT = Path(Path('/tmp/tempo2-ai-committed-review-current').read_text().strip())
DELIVERY = '20e368fe6e7d3e05565ae7f0132f4498dfa9069e'
PIN = 'fda6ad4a9753ad7dc5d291406f2f310a9161423f'
INDEX = 'ceb102a64449abefdc02c1557a6e12c2269550150d69a3ef8e7c08f9f60decc3'
observed = {}

def sha(path):
    path = Path(path).resolve()
    if str(path) not in observed:
        h = hashlib.sha256()
        with path.open('rb') as f:
            for block in iter(lambda: f.read(1048576), b''):
                h.update(block)
        observed[str(path)] = h.hexdigest()
    return observed[str(path)]

def verify(mapping):
    for path, expected in mapping.items():
        assert sha(path) == expected, path

assert sha(BASE/'delivery-identity.json') == INDEX
index = json.loads((BASE/'delivery-identity.json').read_text())
verify(index['artifacts'])
verify(index['authored'])
for name, expected in index['authored'].items():
    relative = str(Path(name).relative_to(ROOT))
    raw = subprocess.check_output(['git', 'show', DELIVERY + ':' + relative], cwd=ROOT)
    assert hashlib.sha256(raw).hexdigest() == expected, relative
for folder, pin in index['pins'].items():
    assert subprocess.check_output(['git', '-C', str(ROOT/folder), 'rev-parse', 'HEAD'], text=True).strip() == pin
    assert not subprocess.check_output(['git', '-C', str(ROOT/folder), 'status', '--porcelain'], text=True)

native_names = ['TKfit.C', 'TKmatrix.C', 'T2accel.C', 'cholesky.C', 'readEphemeris_calceph.C', 'tt2tdb.C', 'global.C', 'tempo2.C']
raw_hashes = {name: hashlib.sha256(subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',PIN+':'+name])).hexdigest() for name in native_names}
profiles = {}
for profile, expected in index['profiles'].items():
    folder = BASE/'profiles-final-v3'/profile
    n = json.loads((folder/'native/identity.json').read_text())
    assert n['pin'] == PIN
    verify(n['objects'])
    verify({str(folder/'native'/name): digest for name,digest in n['executables'].items()})
    assert sha(folder/'native/source/config.h') == n['config']
    assert sha(folder/'native/patch.diff') == n['patch']
    assert sha(ROOT/'validation/reference/build/libtempo2-reference.a') == n['archive']
    assert sha(ROOT/'validation/core_output/build/native-v9/identity.json') == n['AH_identity']
    verify(n.get('calceph', {}))
    for name, hashes in n['source'].items():
        assert sha(folder/'native/source'/name) == hashes['compiled']
        if name in raw_hashes:
            assert hashes['original'] == raw_hashes[name], name
    record = {}
    for group in ['paired','cli','banner']:
        path = folder/group/'receipt.json'
        if not path.exists():
            assert (group == 'paired' and profile == 'qr-default-debug') or (group == 'cli' and profile in ['blas-debug','lapack-debug','calceph-debug'])
            continue
        r = json.loads(path.read_text())
        verify(r['files'])
        if group == 'paired':
            verify(r['sources_and_inputs'])
            assert not r['failures'] and not r['changed_artifacts']
            assert all(not x['comparison']['failures'] and not x.get('execution_failure') for x in r['records'])
            assert all(c['exit'] == 0 for x in r['records'] for c in x['commands'])
            actual = {str(p) for x in r['records'] for p in (folder/group/x['case']).iterdir() if p.is_file()}
            assert actual == set(r['files'])
            record.update(cases=len(r['records']), cells=sum(x['comparison']['checks'] for x in r['records']))
        elif group == 'cli':
            verify(r['identity'])
            assert r['failures'] == 0 and all(x['exit'] == 0 for x in r['records'])
            record['cli_commands'] = len(r['records'])
        else:
            for row in r['records']:
                assert sha(row['executable']) == row['sha256'] and row['exit'] == 1
            record['banner_commands'] = len(r['records'])
    assert all(expected[key] == value for key,value in record.items()), profile
    profiles[profile] = record

provider = json.loads((BASE/'provider-identity.json').read_text())
verify(provider['files'])
final = json.loads((BASE/'final-checks-v3/receipt.json').read_text())
for row in final['records']:
    assert row['exit'] == 0
    assert sha(BASE/'final-checks-v3'/(row['label']+'.log')) == row['log_sha256']
boundaries = json.loads((BASE/'public-boundaries-final-v3/receipt.json').read_text())
verify(boundaries['files'])
fresh = json.loads((OUT/'final-review-completion.json').read_text())
assert fresh['revision'] == 'e96b5898827cfd15f0890431922460860d05d349'
assert fresh['paired_failures'] == 0
used = [name for name in fresh['committed_files'] if name.startswith(('rust/src/','rust/native/')) or name in ['rust/Cargo.toml','rust/Cargo.lock','rust/build.rs','rust/examples/native_backends.rs','rust/tests/native_backends.rs','validation/native_backends/check.py','validation/native_backends/test_check.py','validation/native_backends/cli_check.py']]
for name in used:
    assert sha(ROOT/name) == fresh['committed_files'][name], name
    assert sha(OUT/'workspace'/name) == fresh['committed_files'][name], name
for label in ['paired-release','cli-release','banner-release']:
    r = json.loads((OUT/label/'receipt.json').read_text())
    verify(r['files'])
    if 'identity' in r:
        verify(r['identity'])
    if 'sources_and_inputs' in r:
        verify(r['sources_and_inputs'])

result = dict(delivery_revision=DELIVERY, delivery_index_sha256=INDEX, profiles=profiles,
              numerical_cases=sum(x.get('cases',0) for x in profiles.values()),
              numerical_cells=sum(x.get('cells',0) for x in profiles.values()),
              cli_commands=sum(x.get('cli_commands',0) for x in profiles.values()),
              banner_commands=sum(x['banner_commands'] for x in profiles.values()),
              public_boundary_commands=len(boundaries['records']), final_check_commands=len(final['records']),
              unique_files_verified=len(observed), source_files_equivalent_to_fresh_review=len(used),
              fresh_review_receipt_sha256=sha(OUT/'final-review-completion.json'),
              original_native_sources=raw_hashes, pins=index['pins'], mismatches=[],
              scope='Independent retained-file/source identity audit plus the separately recorded fresh combined-release execution. Full inherited scientific gates, Linux and concurrency remain open.')
(OUT/'central-evidence-audit.json').write_text(json.dumps(result,indent=2)+'\n')
(OUT/'central-verified-files.json').write_text(json.dumps(observed,indent=2)+'\n')
print(json.dumps({k:v for k,v in result.items() if k not in ['profiles','original_native_sources']},indent=2))
