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
                'linux/predictor-performance/measure/after-100000-0.usage',
                'linux/predictor-performance/verify/after-outputs.stdout',
                'linux/predictor-performance/qualify-before/c-shared-run/cheby.dat',
                'linux/predictor-performance/qualify-after/c-shared-run/exchanged.dat',
                'linux/predictor-performance/native-gate/receipt.json',
                'linux/predictor-performance/evaluator/direct-measure/after-leaf-0.usage',
                'linux/predictor-performance/evaluator/direct-verify/before-edges.stdout',
                'linux/predictor-performance/evaluator/direct-verify/after-model-1001.stderr',
                'linux/predictor-performance/evaluator/measure/after-100000-0.usage',
                'linux/predictor-performance/evaluator/native-gate/receipt.json',
                'linux/predictor-performance/recurrence/direct-measure/after-leaf-0.usage',
                'linux/predictor-performance/recurrence/measure/native-100000-0.usage',
                'linux/predictor-performance/recurrence/consumers/before/recurrence_consumer.stdout',
                'linux/predictor-performance/recurrence/consumers/after/recurrence_consumer.stderr',
                'linux/predictor-performance/recurrence/consumers/receipt.json',
                'linux/predictor-performance/recurrence/direct-verify/after-edges.stdout',
                'linux/predictor-performance/recurrence/native-gate/receipt.json',
                'linux/recurrence-object.stdout','linux/recurrence-wide-range-release.stderr',
                'host/recurrence-performance-after-fetch-status.txt',
                'linux/predictor-performance/coefficient-view/coefficient-verify/after-c-shared.stdout',
                'linux/predictor-performance/coefficient-view/coefficient-verify/after-asan-ubsan.stderr',
                'linux/predictor-performance/coefficient-view/direct-measure/after-leaf-0.usage',
                'linux/predictor-performance/coefficient-view/measure/native-100000-0.usage',
                'linux/predictor-performance/coefficient-view/consumers/after/recurrence_consumer.stdout',
                'linux/predictor-performance/coefficient-view/native-gate/receipt.json',
                'linux/coefficient-object.stdout','host/coefficient-performance-after-fetch-status.txt',
                'linux/corpus/time-ephemeris-objects/native.s',
                'linux/corpus/time-ephemeris-objects/rust.s',
                'linux/corpus/time-ephemeris-objects/receipt.json',
                'linux/corpus/time-objects.stderr',
                'linux/gui/render/viewer.png','linux/gui/render/excluded.png',
                'linux/gui/render/saved-123/session.par','linux/gui/render/saved-123/session.tim',
                'linux/gui/render/saved-123/session.json','linux/gui/render/save-error.png',
                'linux/gui/parity/bary-unsorted-gui/session.par','linux/gui/parity/bary-unsorted-gui/session.tim',
                'linux/gui/parity/pks-engine/session.json','linux/gui/parity/pks-cli.stdout',
                'linux/gui/native/native.png','linux/gui/native/native-timing.json','linux/gui/native/opengl.txt',
                'linux/gui/evidence/receipt.json','linux/gui-native.stderr','linux/gui-libraries.stdout',
                'linux/event-source-profile/machine-code.tar.gz',
                'linux/allocation/verify/same-engine/science-correlated-global/after/diagnostics/cov.matrix',
                'linux/allocation/verify/same-engine/science-correlated-global/before/diagnostics/param.labels',
                'linux/allocation/verify/same-engine/science-correlated-global/after/diagnostics/postfit.res',
                'linux/allocation/measure/correlated-global-before-0/fitted.par_1',
                'linux/allocation/measure/correlated-global-after-0/fitted.par_2',
                'linux/allocation/measure/correlated-global-native-0/global_covar.cvm',
                'linux/contraction-source-profile/planetary-readEphemeris.object.s',
                'linux/contraction-source-profile/propagation-shapelet.object.s',
                'linux/contraction-source-profile/ephemeris-offsets/source.patch',
                'linux/contraction-source-profile/ephemeris-offsets/c076_absent_nutation.trace.tsv',
                native+'outputs/coverage_receipts.json.gz',native+'outputs/raw/case.jsonl.gz',
                native+'outputs/failed/events.jsonl',
                native+'build/coverage/get_obsCoord.gcov.json.gz',
                'linux/predictors/case/rust/polyco_new.dat',
                'linux/predictors/case/native/newpolyco.dat',
                'linux/event-diagnostic/gaussian-only.par',
                'linux/event-diagnostic/without-events.par',
                'linux/candidate/source/validation/general_fitting/build/public/reference.json.gz']
            excluded=[native+'build/ordinary/source/config.json.gz',
                'linux/predictor-performance/after/source/rust/Cargo.toml',
                'linux/predictor-performance/after/prefix-build/target/fingerprint.json',
                'linux/predictor-performance/after/prefix/share/install.json',
                'linux/predictor-performance/predictor-tests/fingerprint.json',
                'linux/predictor-performance/verify/after-evaluation',
                'linux/predictor-performance/evaluator/before-direct/plain',
                'linux/predictor-performance/evaluator/after-direct/counted',
                'linux/predictor-performance/evaluator/after-direct/target/fingerprint.json',
                'linux/predictor-performance/evaluator/predictor-tests/fingerprint.json',
                'linux/predictor-performance/recurrence/after/source/rust/Cargo.toml',
                'linux/predictor-performance/recurrence/after/prefix/share/install.json',
                'linux/predictor-performance/recurrence/after-direct/plain',
                'linux/predictor-performance/recurrence/predictor-tests/fingerprint.json',
                'linux/predictor-performance/recurrence/consumers/after/target/fingerprint.json',
                'linux/predictor-performance/recurrence/consumers/after/src/bin/recurrence_consumer.rs',
                'linux/predictor-performance/coefficient-view/after/source/rust/Cargo.toml',
                'linux/predictor-performance/coefficient-view/after/prefix/share/install.json',
                'linux/predictor-performance/coefficient-view/predictor-tests/fingerprint.json',
                'linux/predictor-performance/coefficient-view/coefficient-verify/after-c-shared',
                'linux/gui/target/debug/fingerprint.json','linux/gui/target/debug/test.png',
                'linux/gui/prefix/bin/tempo2-gui','linux/gui/prefix/.crates2.json',
                'linux/gui/source/private.par','linux/not-gui/image.png',
                'linux/event-source-profile/other.tar.gz','linux/other/machine-code.tar.gz',
                'linux/event-source-profile/nested/machine-code.tar.gz',
                'linux/allocation/before/source/rust/target/fingerprint.json',
                'linux/allocation/provider-target/fingerprint.json','linux/allocation/debug-target/fingerprint.json',
                'linux/allocation/before/bin/observe',
                'linux/test-target/fingerprint.json.gz','linux/corpus-target/fingerprint.json',
                'linux/runtime/resources.json.gz',
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
