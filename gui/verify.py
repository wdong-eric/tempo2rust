#!/usr/bin/env python3
"""Compare the viewer's canonical workflow with the CLI in fresh processes."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import tomllib

ROOT = Path(__file__).resolve().parent.parent


def verify(cli, viewer_export, session_workflow, runtime, out):
    out.mkdir(parents=True, exist_ok=False)
    bindings = {}
    records = []
    for name, par_name, tim_name in [
        ('bary-unsorted', 'f0.par', 'bary_unsorted.tim'),
        ('pks', 'pks_seven_mode1.par', 'pks_seven.tim'),
    ]:
        par, tim = [ROOT / 'validation/fitting/inputs' / p for p in [par_name, tim_name]]
        commands = {
            'cli': [cli, 'timing', '--par', par, '--tim', tim, '--runtime', runtime],
            'viewer': [viewer_export, par, tim, runtime],
        }
        outputs = {}
        for label, command in commands.items():
            process = subprocess.run(command, capture_output=True)
            (out / f'{name}-{label}.stdout').write_bytes(process.stdout)
            (out / f'{name}-{label}.stderr').write_bytes(process.stderr)
            assert process.returncode == 0, (name, label, process.stderr.decode(errors='replace'))
            outputs[label] = process.stdout
        assert outputs['cli'] == outputs['viewer'], f'{name}: complete timing output differs'
        result = json.loads(outputs['viewer'])
        rows = result['pulsars'][0]['observations']
        # These authored fixtures are FORMAT 1 without INCLUDE or format switches.
        names = [line.split()[0] for line in tim.read_text().splitlines()
                 if line.strip() and not line.startswith(('FORMAT', '#'))]
        assert len(rows) == len(names)
        assert sorted(row['input_index'] for row in rows) == list(range(len(names)))
        for row in rows:
            assert row['filename']['text'] == names[row['input_index']]
            assert row['uncertainty']['unit'] == 'us'
        records.append({'case': name, 'toas': len(rows), 'exact_cli_agreement': True,
                        'original_input_identity': True})
        histories = {}
        for mode in ['gui', 'engine']:
            destination = out / f'{name}-{mode}'
            process = subprocess.run([session_workflow, mode, par, tim, runtime, destination], capture_output=True)
            (out / f'{name}-{mode}-history.stdout').write_bytes(process.stdout)
            (out / f'{name}-{mode}-history.stderr').write_bytes(process.stderr)
            assert process.returncode == 0, (name, mode, process.stderr.decode(errors='replace'))
            histories[mode] = process.stdout
        assert histories['gui'] == histories['engine'], f'{name}: session history differs'
        history = json.loads(histories['gui'])
        assert history[-1]['reloaded_toas'] == len(rows) - 1
        excluded = [r for r in history[1]['pulsars'][0]['observations'] if r['deleted'] != 0]
        assert len(excluded) == 1 and excluded[0]['input_index'] == 3
        assert all(r['deleted'] == 0 for r in history[-2]['pulsars'][0]['observations'])
        for fit in history[2:5]:
            for cycle in fit['cycles']:
                for system in cycle['systems']:
                    assert all(r['input_index'] != 3 for r in system['rows'])
        gui_files = {p.relative_to(out / f'{name}-gui') for p in (out / f'{name}-gui').rglob('*') if p.is_file()}
        engine_files = {p.relative_to(out / f'{name}-engine') for p in (out / f'{name}-engine').rglob('*') if p.is_file()}
        assert gui_files == engine_files
        for path in gui_files:
            assert (out / f'{name}-gui' / path).read_bytes() == (out / f'{name}-engine' / path).read_bytes(), (name, path)
        records[-1].update(exact_session_history=True, exact_saved_files=sorted(map(str, gui_files)),
                           excluded_input=3, reincluded=True, reloaded_toas=len(rows)-1)
        for path in [par, tim, cli, viewer_export, session_workflow]:
            bindings[str(path)] = hashlib.sha256(path.read_bytes()).hexdigest()
    core = tomllib.loads((ROOT / 'rust/Cargo.lock').read_text())['package']
    gui = tomllib.loads((ROOT / 'gui/Cargo.lock').read_text())['package']
    for dependency in core:
        assert any(p['name'] == dependency['name'] and p['version'] == dependency['version']
                   and p.get('checksum') == dependency.get('checksum') for p in gui), dependency['name']
    receipt = {'status': 'pass', 'scope': 'GUI/CLI wiring and input identity, not new native science qualification',
               'cases': records, 'core_dependency_versions_preserved': True, 'bindings': bindings,
               'outputs': {str(p.relative_to(out)): hashlib.sha256(p.read_bytes()).hexdigest() for p in out.rglob('*') if p.is_file()},
               'recipe_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    (out / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
    print(json.dumps(records, indent=2))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ['cli', 'viewer-export', 'session-workflow', 'runtime', 'out']:
        parser.add_argument('--' + name, type=Path, required=True)
    args = parser.parse_args()
    verify(args.cli.resolve(), args.viewer_export.resolve(), args.session_workflow.resolve(), args.runtime.resolve(), args.out.resolve())
