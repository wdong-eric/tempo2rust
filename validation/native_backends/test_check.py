"""Mutation controls for the frozen numerical projection, without native providers."""
import tempfile
import unittest
from pathlib import Path
from check import compare


class ComparatorControls(unittest.TestCase):
    def comparison(self, native, rust):
        with tempfile.TemporaryDirectory() as d:
            a, b = Path(d)/'native.tsv', Path(d)/'rust.tsv'
            a.write_text(native); b.write_text(rust)
            return compare(a, b)['failures']

    def test_finite_sign_is_exact_inside_time_gate(self):
        x = 'probe/tt\ttime\t5e-14\n'
        self.assertFalse(self.comparison(x, x))
        self.assertTrue(self.comparison(x, x.replace('5e-14', '-5e-14')))

    def test_scalar_and_vector_families_cannot_lend_scales(self):
        x = 'qr/rhs/0\tmatrix\t1\nqr/rhs/1\tmatrix\t2\nqr/p/0\tmatrix\t1000000\nqr/p/1\tmatrix\t3\nqr/chi\tmatrix\t1\n'
        self.assertFalse(self.comparison(x, x))
        self.assertTrue(self.comparison(x, x.replace('rhs/0\tmatrix\t1', 'rhs/0\tmatrix\t1.000000001')))
        self.assertTrue(self.comparison(x, x.replace('chi\tmatrix\t1', 'chi\tmatrix\t1.000000001')))
        self.assertFalse(self.comparison(x, x.replace('rhs/0\tmatrix\t1', 'rhs/0\tmatrix\t1.00000000000009')))

    def test_matrix_scale_uses_same_column_and_family(self):
        x = 'qr/a/0/0\tmatrix\t1\nqr/a/1/0\tmatrix\t10\nqr/a/0/1\tmatrix\t1000000\nqr/c/0/0\tmatrix\t1000000\n'
        self.assertFalse(self.comparison(x, x.replace('a/0/0\tmatrix\t1', 'a/0/0\tmatrix\t1.0000000000004')))
        self.assertTrue(self.comparison(x, x.replace('a/0/0\tmatrix\t1', 'a/0/0\tmatrix\t1.0000000000006')))

    def test_membership_kind_zero_and_nonfinite_classes(self):
        x = 'x\ttime\t0\n'
        for y in ['', x+'y\ttime\t0\n', x.replace('time', 'phase'), x.replace('\t0', '\t-0'), x.replace('\t0', '\t1e-30')]:
            self.assertTrue(self.comparison(x, y))
        for a,b in [('NaN','Infinity'),('Infinity','-Infinity'),('NaN','-NaN')]:
            self.assertTrue(self.comparison(f'x\ttime\t{a}\n',f'x\ttime\t{b}\n'))

    def test_row_order_is_exact(self):
        x='a\ttime\t1\nb\ttime\t2\n'
        self.assertFalse(self.comparison(x,x))
        self.assertTrue(self.comparison(x,'b\ttime\t2\na\ttime\t1\n'))


if __name__ == '__main__':
    unittest.main()
