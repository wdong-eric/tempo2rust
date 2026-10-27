#!/usr/bin/env python3
"""Link unchanged AC/AD/AF/AH observers to the freshly reconstructed core."""
import argparse
import difflib
import json
import os
from pathlib import Path
import shutil
import sys
from common import ROOT, Run, sha, identities


def build(out, core, a2):
    run = Run(out)
    out = run.out
    source = out / 'source'
    shutil.copytree(core / 'source', source)
    identity = json.loads((core / 'receipt.json').read_text())
    cxx = os.environ.get('NATIVE_CXX', 'g++')
    flags = ['-O2', '-DHAVE_CONFIG_H', '-I' + str(source), '-I' + str(a2 / 'source')]
    # The recorded link tail includes the actual selected provider libraries.
    old_link = next(c['argv'] for c in identity['commands'] if c['label'] == 'probe-link')
    objects = [core / name for name in identity['objects']]
    archive_index = old_link.index(str(a2 / 'libtempo2-reference.a'))
    libraries = old_link[archive_index:old_link.index('-o')]
    def link(name, driver, selected=objects, extra=()):
        run.run([cxx, *flags, *extra, driver, *selected, *libraries, '-o', out / name], name + '-link')
    link('timing', ROOT / 'validation/timing_models/native_timing.cc')
    link('predictor', ROOT / 'validation/predictors/native.cc',
         [p for p in objects if p.name != 'hooks.cc.o'], ['-ffp-contract=off'])
    link('probe', ROOT / 'validation/core_output/native_probe.cc')
    shutil.copy2(core / 'tempo2', out / 'tempo2')
    # argc==2 cannot accept allocation flags. Same AH bounded-entry witness.
    run.run([cxx, *flags, '-Dmain=ah_source_main', '-c', source / 'tempo2.C', '-o', out / 'main.o'], 'bounded-main')
    driver = source / 'bounded_driver.cc'
    driver.write_text('#include "tempo2.h"\nextern int ah_source_main(int,char**);\nint main(int n,char**a){MAX_PSR=2;MAX_OBSN=100;return ah_source_main(n,a);}\n')
    link('tempo2-bounded', driver, [out / 'main.o', *objects])
    # Retain the AH original helper witness independently of corrected output.
    # Its upstream t2fit uses the established AE corrections, without AH edits.
    sys.path.insert(0, str(ROOT / 'validation/general_fitting'))
    from fitting_corrections import corrected
    original_objects = {p.name: p for p in objects}
    for name in ['textOutput.C', 'get_obsCoord.C', 't2fit.C', 'preProcessSimple.C']:
        original = (core / ('raw-' + name + '.stdout')).read_text()
        if name == 't2fit.C':
            original = corrected(name, original)
        path = out / ('original-' + name)
        path.write_text(original)
        obj = out / ('original-' + name + '.o')
        run.run([cxx, *flags, '-I' + str(ROOT / 'validation/release/a2'), '-c', path, '-o', obj], 'original-' + name)
        original_objects[name + '.o'] = obj
    link('probe-original', ROOT / 'validation/core_output/native_probe.cc', list(original_objects.values()))
    maps = {}
    for path in source.glob('*.C'):
        raw = core / ('raw-' + path.name + '.stdout')
        if not raw.exists():
            continue
        original = raw.read_text().splitlines()
        new = path.read_text().splitlines()
        maps[path.name] = {str(block.b + i + 1): block.a + i + 1
            for block in difflib.SequenceMatcher(a=original,b=new,autojunk=False).get_matching_blocks()
            for i in range(block.size)}
    run.save({'status': 'pass', 'core_receipt': sha(core / 'receipt.json'), 'source': identities(source),
              'diagnostic_line_maps': maps, 'executables': {name: sha(out / name) for name in
               ['timing','predictor','probe','probe-original','tempo2','tempo2-bounded']}})
    # AH accepts an identity filename; this is the new construction receipt,
    # not a fabricated predecessor identity.
    shutil.copyfile(out / 'receipt.json', out / 'identity.json')


if __name__ == '__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['out','core','a2']:
        p.add_argument('--'+name,type=Path,required=True)
    a=p.parse_args();build(a.out,a.core.resolve(),a.a2.resolve())
