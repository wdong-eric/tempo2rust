"""Retain compressed observations without admitting source/cache/runtime trees."""
import gzip
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from common import sha


class EvidenceCollection(unittest.TestCase):
    def test_missing_input_is_not_an_empty_success(self):
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary);out=root/'selected'
            result=subprocess.run([sys.executable,Path(__file__).with_name('collect_evidence.py'),
                '--input',root/'missing','--out',out],capture_output=True,text=True)
            self.assertNotEqual(result.returncode,0)
            self.assertIn('input directory does not exist',result.stderr)
            self.assertFalse(out.exists())

    def test_compressed_native_records_and_existing_exclusions(self):
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary);source=root/'input';out=root/'selected'
            native='linux/native-corpora/staging/validation/observer_coordinates/'
            included=['receipt.json','linux/native-corpora/producer.py',native+'outputs/reference.json.gz',
                native+'outputs/coverage_receipts.json.gz',native+'outputs/raw/case.jsonl.gz',
                native+'outputs/failed/events.jsonl',
                native+'build/coverage/get_obsCoord.gcov.json.gz',
                'linux/predictors/case/rust/polyco_new.dat',
                'linux/predictors/case/native/newpolyco.dat',
                'linux/event-diagnostic/gaussian-only.par',
                'linux/event-diagnostic/without-events.par',
                'linux/candidate/source/validation/general_fitting/build/public/reference.json.gz']
            excluded=[native+'build/ordinary/source/config.json.gz',
                'linux/test-target/fingerprint.json.gz','linux/runtime/resources.json.gz',
                'linux/install-default/share/source.json.gz','linux/gmp-cache/cache.json.gz',
                'linux/c-concurrency-default/cache-target/fingerprint.json',
                'linux/c-concurrency-default/library-owned-cache-source/Cargo.toml',
                'linux/providers/inpop17a.dat',
                'linux/providers/source.tar.gz',native+'build/probe',native+'outputs/raw.bin.gz']
            data=gzip.compress(b'{"case":"observer","events":[{"phase":"end","state":{}}]}\n',mtime=0)
            for name in included+excluded:
                path=source/name;path.parent.mkdir(parents=True,exist_ok=True)
                path.write_bytes(data if name.endswith('.gz') else b'{}\n')
            (source/'linked.json.gz').symlink_to(source/included[1])
            subprocess.run([sys.executable,Path(__file__).with_name('collect_evidence.py'),
                '--input',source,'--out',out],check=True,capture_output=True)
            manifest=json.loads((out/'manifest.json').read_text())['files']
            self.assertEqual(set(manifest),set(included))
            for name in included:
                self.assertEqual((source/name).read_bytes(),(out/name).read_bytes())
                self.assertEqual(manifest[name],{'sha256':sha(source/name),'bytes':(source/name).stat().st_size})


if __name__=='__main__':unittest.main()
