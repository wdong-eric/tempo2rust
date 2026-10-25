#!/usr/bin/env python3
"""Separate 0053 original/corrected-native proof for mixed-pulsar AB histories.

Never writes accepted AB staging, runners or artifacts. Original and corrected
executables run against the same owned inputs, with a fresh output nonce each
launch; original records must reproduce the accepted oracle exactly.
"""
import difflib
import gzip
import hashlib
import json
import os
from pathlib import Path
import secrets
import subprocess

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
AB = ROOT / 'validation/propagation'
BUILD = HERE / 'build/propagation-corrected'
NATIVE = AB / 'build/ordinary'
CASES = {'bclt_tropo_0', 'phase_cross_0_1', 'phase_cross_1_0'}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    BUILD.mkdir(parents=True, exist_ok=True)
    profile = json.loads((NATIVE / 'profile.json').read_text())
    for path, expected in profile['dependencies'].items():
        assert sha(Path(path).read_bytes()) == expected, path
    original = NATIVE / 'probe'
    assert sha(original.read_bytes()) == profile['executable_sha256']
    assert profile['executable_sha256'] == json.loads((AB / 'outputs/record_identity.json').read_text())['actual_executable_sha256']
    changes = {
        'calculate_bclt.C': ('if (psr[p].correctTroposphere==0) psr[0].obsn[i].troposphericDelay=0;',
                            'if (psr[p].correctTroposphere==0) psr[p].obsn[i].troposphericDelay=0;'),
        'refphs.C': ('if(psr->refphs==REFPHS_TZR)', 'if(psr[p].refphs==REFPHS_TZR)'),
    }
    objects, sources, commands, patch = {}, {}, [], []
    for name, (old, new) in changes.items():
        raw = (NATIVE / 'source' / name).read_text()
        assert raw.count(old) == 1
        corrected = raw.replace(old, new)
        path = BUILD / name
        path.write_text(corrected)
        obj = path.with_suffix('.o')
        command = [profile['commands'][0][0], *profile['flags'], '-c', str(path), '-o', str(obj)]
        subprocess.run(command, check=True, capture_output=True)
        commands.append(command)
        objects[str(NATIVE / obj.name)] = str(obj)
        sources[name] = {'original_sha256': sha(raw.encode()), 'corrected_sha256': sha(corrected.encode())}
        patch.extend(difflib.unified_diff(raw.splitlines(True), corrected.splitlines(True), fromfile=name, tofile=name))
    (BUILD / 'corrections.patch').write_text(''.join(patch))
    executable = BUILD / 'probe'
    command = [objects.get(a, a) for a in profile['commands'][-1]]
    command[command.index('-o') + 1] = str(executable)
    command = [a.replace(str(NATIVE / 'probe.map'), str(BUILD / 'probe.map')) for a in command]
    subprocess.run(command, check=True, capture_output=True)
    commands.append(command)
    corrected_identity = sha(executable.read_bytes())
    originals = {c['id']: c for c in json.load(gzip.open(AB / 'outputs/reference.json.gz', 'rt'))}
    catalog = json.loads((AB / 'outputs/frozen/cases.json').read_text())
    receipts, results = [], []
    for case in catalog:
        if case['id'] not in CASES:
            continue
        work = BUILD / case['id']
        work.mkdir(exist_ok=True)
        for name, content in case['files'].items():
            path = work / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(content)
        actions = [[v.replace('{case}', str(work)) for v in a] for a in case['actions']]
        inputs = work / 'actions.tsv'
        inputs.write_text('\n'.join('\t'.join('x' + v.encode().hex() for v in a) for a in actions) + '\n')
        input_ids = {name: sha((work / name).read_bytes()) for name in [*case['files'], 'actions.tsv']}
        normalize = lambda t: t.replace(str(NATIVE / 'source'), '{source}').replace(str(BUILD), '{source}').replace('{source}/' + case['id'], '{cases}/' + case['id']).replace(str(AB), '{propagation}')
        for mode, exe, identity in [('original', original, profile['executable_sha256']), ('corrected', executable, corrected_identity)]:
            output = work / f'{mode}.jsonl'
            output.unlink(missing_ok=True)
            nonce = secrets.token_hex(16)
            assert sha(exe.read_bytes()) == identity
            proc = subprocess.run([str(exe), str(inputs), str(output), nonce], capture_output=True,
                                  env={**os.environ, 'LC_ALL': 'C', 'LANG': 'C'}, timeout=case['timeout'])
            assert proc.returncode == case['status'] == 0
            assert sha(exe.read_bytes()) == identity
            assert input_ids == {name: sha((work / name).read_bytes()) for name in input_ids}
            lines = output.read_text().splitlines()
            assert json.loads(lines[0]) == {'launch': nonce}
            result = {k: case[k] for k in ['id', 'classification', 'actions']}
            result.update(status=proc.returncode, events=[json.loads(normalize(s)) for s in lines[1:]],
                          stdout=normalize(proc.stdout.decode('latin1')), stderr=normalize(proc.stderr.decode('latin1')))
            (work / f'{mode}.stdout').write_bytes(proc.stdout)
            (work / f'{mode}.stderr').write_bytes(proc.stderr)
            if mode == 'original':
                assert result == originals[case['id']], case['id'] + ': fresh original differs from accepted oracle'
            else:
                results.append(result)
            receipts.append({'case': case['id'], 'mode': mode, 'nonce': nonce, 'executable_sha256': identity,
                             'input_sha256': input_ids, 'output_sha256': sha(output.read_bytes()),
                             'stdout_sha256': sha(proc.stdout), 'stderr_sha256': sha(proc.stderr)})
    assert len(results) == len(CASES)
    reference = BUILD / 'reference.json.gz'
    reference.write_bytes(gzip.compress(json.dumps(results).encode(), mtime=0))
    receipt = {'status': 'PASS original reproduction and corrected-native production',
               'sources': sources, 'commands': commands, 'executions': receipts,
               'reference_sha256': sha(reference.read_bytes()), 'patch_sha256': sha((BUILD / 'corrections.patch').read_bytes()),
               'linked_objects_sha256': {a: sha(Path(a).read_bytes()) for a in command if a.endswith('.o')},
               'original_reference_sha256': sha((AB / 'outputs/reference.json.gz').read_bytes())}
    (BUILD / 'native-proof.json').write_text(json.dumps(receipt, indent=2) + '\n')
    print('PASS: 3 original histories reproduced; 3 separate corrected-native histories generated')


if __name__ == '__main__':
    main()
