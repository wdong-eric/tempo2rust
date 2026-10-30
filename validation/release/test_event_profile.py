import tempfile
from pathlib import Path
import unittest
from event_profile import comparison


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


if __name__=='__main__':unittest.main()
