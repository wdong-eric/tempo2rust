import tempfile
import json
import tarfile
from pathlib import Path
import unittest
from unittest.mock import patch
from common import Run, sha
from event_profile import comparison, machine_code


class EventProfileTests(unittest.TestCase):
    def test_ordinary_neighbor_and_signed_classes_are_not_tolerated(self):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);native=root/'native.tsv';rust=root/'rust.tsv'
            native.write_text('profile\tordinary_bits\t64\nprofile\ttiming_bits\t113\na\tdt\t1\na\tcallback_1\t-0\na\texp\tNaN\n')
            rust.write_text(native.read_text().replace('timing_bits\t113','timing_bits\t128'))
            self.assertTrue(all(r['equal'] for r in comparison(native,rust)))
            rust.write_text(rust.read_text().replace('dt\t1','dt\t1.0000000000000000001084202172485504434')
                .replace('callback_1\t-0','callback_1\t0').replace('exp\tNaN','exp\t-NaN'))
            self.assertEqual(sum(not r['equal'] for r in comparison(native,rust)),3)

    def test_machine_code_binds_archive_and_preserves_executed_bytes(self):
        for fault in [None,'archive','source','object','before','during']:
            with self.subTest(fault=fault),tempfile.TemporaryDirectory() as directory:
                root=Path(directory);a2=root/'a2';(a2/'source').mkdir(parents=True)
                library=a2/'libtempo2-reference.a';library.write_bytes(b'bound archive')
                source=a2/'source/t2fit_glitch.C';source.write_bytes(b'bound callback source')
                callback=a2/'t2fit_glitch.C.o';callback.write_bytes(b'linked callback object')
                (a2/'receipt.json').write_text(json.dumps({'library':sha(library),
                    'source':{'t2fit_glitch.C':sha(source)},'commands':[{'label':'compile-t2fit_glitch.C',
                    'status':0,'argv':['g++','-c',str(source),'-o',str(callback)]}]}))
                executable=root/'executed';executable.write_bytes(b'executed binary')
                expected=sha(executable);run=Run(root/'out')
                if fault in ['archive','source','object','before']:
                    {'archive':library,'source':source,'object':callback,'before':executable}[fault].write_bytes(b'changed')
                def command(argv,label):
                    if label=='native-callback-archive-member':
                        raw=b'linked callback object'
                        (run.out/(label+'.stdout')).write_bytes(raw)
                        return raw
                    if fault=='during' and label=='rust-event-tests-disassembly':executable.write_bytes(b'changed')
                    return b'actual disassembly\n'
                with patch('event_profile.platform.system',return_value='Linux'),patch.object(run,'run',side_effect=command):
                    if fault:
                        with self.assertRaises(AssertionError):machine_code(run,a2,{'rust-event-tests':(executable,expected)})
                    else:
                        record=machine_code(run,a2,{'rust-event-tests':(executable,expected)})
                        with tarfile.open(record['bundle']) as archive:
                            self.assertEqual(set(archive.getnames()),{'rust-event-tests','native-callback.o'})
                            self.assertEqual(archive.extractfile('rust-event-tests').read(),executable.read_bytes())
                            self.assertEqual(archive.extractfile('native-callback.o').read(),callback.read_bytes())
                        self.assertEqual(record['bundle_sha256'],sha(record['bundle']))
                        for entry in record['products'].values():
                            self.assertEqual(entry['disassembly_sha256'],sha(entry['disassembly']))


if __name__=='__main__':unittest.main()
