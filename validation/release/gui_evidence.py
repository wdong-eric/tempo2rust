#!/usr/bin/env python3
"""Bind Linux GUI products and exact CLI parity; image interpretation is separate."""
import argparse
import json
from pathlib import Path
import struct
from common import Run, sha
from native_metadata import loader_identity
from rust_suite import test_counts


def verify(run, gui, candidate):
    matrix=gui.parent;inputs={};tests={}
    for label,name in [('gui-library-tests',None),('gui-control-render','tests::load_inspect_and_render'),
                       ('gui-native','native_test::native_x11_load_render_close')]:
        path=matrix/(label+'.stdout');text=path.read_text();inputs[str(path)]=sha(path)
        counts=test_counts(text)
        assert counts['executed'] and counts['failing_tests']==0, (label,'tests did not pass')
        if name:assert 'test '+name+' ... ok' in text.splitlines(), (label,'named test absent or failed')
        tests[label]=counts
    parity=gui/'parity/receipt.json';record=json.loads(parity.read_text())
    assert record['status']=='pass' and record['core_dependency_versions_preserved']
    native=gui/'native/native-timing.json';cli=gui/'parity/bary-unsorted-cli.stdout'
    assert native.read_bytes()==cli.read_bytes(), 'complete native/CLI timing output differs'
    assert record['outputs']['bary-unsorted-cli.stdout']==sha(cli), 'CLI output differs from parity receipt'
    context=gui/'native/opengl.txt';lines=context.read_text().splitlines()
    assert len(lines)==3 and all(lines), 'missing OpenGL version/vendor/renderer'
    images={}
    for relative in ['render/'+name+'.png' for name in
                     ['viewer','excluded','fitted','saved','save-error','input-error']]+['native/native.png']:
        path=gui/relative;data=path.read_bytes()
        assert data[:8]==b'\x89PNG\r\n\x1a\n' and data[12:16]==b'IHDR', ('missing PNG header',path)
        width,height=struct.unpack('>II',data[16:24]);assert width and height
        images[relative]={'sha256':sha(path),'width':width,'height':height}
    # The GUI runner already compares every saved file with the direct engine.
    # Require each public product here too, so collection cannot silently omit it.
    for name in ['bary-unsorted','pks']:
        for mode in ['gui','engine']:
            for product in ['session.par','session.tim','session.json']:
                path=gui/'parity'/f'{name}-{mode}'/product
                assert path.is_file(), ('missing saved product',path)
                inputs[str(path)]=sha(path)
    binaries=[gui/'prefix/bin/tempo2-gui',gui/'target/debug/examples/export_timing',
              gui/'target/debug/examples/session_workflow']
    build=matrix/'gui-tests-build.stdout'
    artifacts=[json.loads(line) for line in build.read_text().splitlines() if line.startswith('{')]
    executed=[row for row in artifacts if row.get('executable') and row.get('profile',{}).get('test')]
    assert {row['target']['name'] for row in executed}=={'tempo2_gui','tempo2-gui'}, 'missing GUI test artifact'
    binaries.extend(Path(row['executable']) for row in executed)
    libraries=matrix/'gui-libraries.stdout'
    loader=loader_identity(libraries.read_text().splitlines())
    for path in [*binaries,build,parity,native,cli,context,libraries,candidate.parent/'receipt.json',
                 candidate/'gui/Cargo.toml',candidate/'gui/Cargo.lock',candidate/'rust/Cargo.lock']:
        inputs[str(path)]=sha(path)
    run.save({'status':'pass','tests':tests,'native_timing_exact_cli':True,
              'opengl':dict(zip(['version','vendor','renderer'],lines)),
              'linked_libraries':loader,'images':images,'inputs':inputs,'producer':sha(__file__),
              'scope':'Linked-library inventory, existing GUI checks and exact CLI/session parity. PNG presence/dimensions only; central visual inspection and Wayland qualification are not implied.'})


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    for name in ['out','gui','candidate']:parser.add_argument('--'+name,type=Path,required=True)
    args=parser.parse_args()
    verify(Run(args.out),args.gui.resolve(),args.candidate.resolve())
