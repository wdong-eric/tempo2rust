#!/usr/bin/env python3
"""Export one committed candidate and bind explicitly staged scientific resources."""
import argparse
import sys
if sys.version_info < (3, 12):
    raise SystemExit("Python 3.12 or newer is required for safe archive extraction")
from pathlib import Path
import shutil
import subprocess
import tarfile
from common import ROOT, Run, identities, sha


def export(out, revision, a2):
    run = Run(out)
    out = run.out
    commit = run.run(['git', '-C', ROOT, 'rev-parse', revision + '^{commit}'], 'commit').decode().strip()
    archive = out / 'candidate.tar'
    run.run(['git', '-C', ROOT, 'archive', '--format=tar', '-o', archive, commit], 'export')
    source = out / 'source'
    source.mkdir()
    with tarfile.open(archive) as tar:
        tar.extractall(source, filter='data')
    source_files = identities(source)
    archive_hash = sha(archive)
    archive.unlink()
    # Existing tests use source-relative runtime paths. Bind only the declared
    # fresh reference, never a user's canonical install or historical outputs.
    native = source / 'tempo2'
    native.mkdir()
    for path in (a2 / 'source').iterdir():
        raw = a2 / ('raw-' + path.name + '.stdout')
        if raw.is_file():
            # Source-line comparators require the pinned, uninstrumented blob.
            # Build headers and noncompiled inputs still use the staged tree.
            shutil.copyfile(raw, native / path.name)
        else:
            (native / path.name).symlink_to(path, target_is_directory=path.is_dir())
    (native / 'T2runtime').symlink_to(a2 / 'runtime', target_is_directory=True)
    reference = source / 'validation/reference/build'
    reference.mkdir(parents=True)
    (reference / 'source').symlink_to(native, target_is_directory=True)
    for name in ['libtempo2-reference.a', 'export_reference', 'tempo2-reference']:
        (reference / name).symlink_to(a2 / name)
    # AA's test adapter canonicalizes its native diagnostic source directory.
    # Bind that declared source to this fresh construction, not an old build.
    observer = source / 'validation/observer_coordinates/build/ordinary/source'
    observer.parent.mkdir(parents=True)
    observer.symlink_to(a2 / 'source', target_is_directory=True)
    run.save({'status': 'pass', 'candidate': commit, 'archive_sha256': archive_hash,
              'source': source_files, 'a2_receipt': sha(a2 / 'receipt.json'),
              'resource_binding': str(a2 / 'runtime')})
    return source


if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--out', type=Path, required=True)
    p.add_argument('--revision', required=True)
    p.add_argument('--a2', type=Path, required=True)
    a = p.parse_args()
    print(export(a.out, a.revision, a.a2.resolve()))
