"""Protect defined telescope inputs and cross-platform measurement units."""
import ast
import unittest
from common import ROOT
from native_corpora import bounded_telescope_inputs, TELESCOPE
from process_usage import parse_usage


class PortabilityTests(unittest.TestCase):
    def test_actual_q_catalog_respects_native_telescope_capacity(self):
        source = (ROOT/'validation/par_specialized/fixtures.py').read_text()
        def families(text):
            return ast.literal_eval(next(n.value for n in ast.walk(ast.parse(text))
                if isinstance(n, ast.Assign) and any(isinstance(t, ast.Name) and t.id == 'families' for t in n.targets)))
        original = families(source)
        bounded = families(bounded_telescope_inputs(source))
        self.assertEqual(len(original), len(bounded))
        for before, after in zip(original, bounded, strict=True):
            self.assertEqual(after, (before[0], 500, before[2]) if before[0] in TELESCOPE else before)
        with self.assertRaises(AssertionError):
            bounded_telescope_inputs(source.replace("('TEL_DX',1000,", "('TEL_DX',999,"))

    def test_time_profiles_preserve_cpu_and_rss_units(self):
        mac = ' 3.00 real 1.25 user 0.75 sys\n 1048576 maximum resident set size\n'
        linux = 'User time (seconds): 1.25\nSystem time (seconds): 0.75\nMaximum resident set size (kbytes): 1024\n'
        self.assertEqual(parse_usage(mac, 'Darwin'), (2.0, 1048576))
        self.assertEqual(parse_usage(linux, 'Linux'), (2.0, 1048576))
        with self.assertRaises((TypeError, AttributeError)):
            parse_usage('incomplete measurement', 'Linux')
        with self.assertRaises(ValueError):
            parse_usage(linux, 'unknown')


if __name__ == '__main__':
    unittest.main()
