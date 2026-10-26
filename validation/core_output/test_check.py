"""Mutation checks for the output checker, including its actual process status."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from check import compare, run, summary

class CheckerTests(unittest.TestCase):
    def test_metadata_normalizer_never_touches_data_files(self):
        with tempfile.TemporaryDirectory(prefix='ah-data-') as tmp:
            a, b = Path(tmp) / 'a', Path(tmp) / 'b'
            a.mkdir(); b.mkdir()
            (a / 'model.par').write_bytes(b'PSRJ [source.C:1]\n')
            (b / 'model.par').write_bytes(b'PSRJ [source.C:2]\n')
            result = compare(a, b, lambda data, kind: b'masked')
            self.assertEqual(result['different'], ['model.par'])

    def test_rejects_missing_reordered_or_altered_content(self):
        with tempfile.TemporaryDirectory(prefix='ah-checker-') as tmp:
            root = Path(tmp)
            a, b = root / 'native', root / 'rust'
            a.mkdir(); b.mkdir()
            source = b'A 1 s 1\nB 2 Hz 0\n'
            (a / 'output').write_bytes(source)
            mutants = [b'A 1 s 1\n', b'B 2 Hz 0\nA 1 s 1\n', source.replace(b' s ', b' ms '),
                       source.replace(b's 1', b's 0'), source.replace(b'A 1', b'A 9'), None]
            for value in [source, *mutants]:
                if value is None:
                    (b / 'output').unlink()
                else:
                    (b / 'output').write_bytes(value)
                r = subprocess.run([sys.executable, str(Path(__file__).with_name('check.py')),
                                    '--compare', str(a), str(b)], capture_output=True)
                self.assertEqual(r.returncode, int(value != source))
                self.assertEqual(json.loads(r.stdout)['summary']['exit_status'], r.returncode)

    def test_unexpected_execution_status_and_timeout_fail(self):
        with tempfile.TemporaryDirectory(prefix='ah-status-') as tmp:
            for command, timeout in [([sys.executable, '-c', 'raise SystemExit(7)'], 5),
                                      ([sys.executable, '-c', 'import time; time.sleep(10)'], 0.02)]:
                record = run(command, Path(tmp), None, expected=0, timeout=timeout)
                self.assertEqual(summary({}, [record])['unexpected_status'], 1)
                self.assertEqual(summary({}, [record])['exit_status'], 1)
            record = run([sys.executable, '-c', 'pass'], Path(tmp), None)
            self.assertEqual(summary({}, [record])['exit_status'], 0)

if __name__ == '__main__':
    unittest.main(verbosity=2)
