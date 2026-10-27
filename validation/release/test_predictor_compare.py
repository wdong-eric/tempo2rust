"""Protect the release comparator against narrowing the Linux ABI to binary64."""
from decimal import Decimal as D
import unittest
from predictor_compare import native_number, compare_values


class NativePrecision(unittest.TestCase):
    def test_x87_neighbors_remain_distinct(self):
        one = native_number('0x8p-3')
        neighbor = native_number('0x8.000000000000001p-3')
        self.assertEqual(one, D(1))
        self.assertGreater(neighbor, one)
        self.assertEqual(float(one), float(neighbor))
        with self.assertRaises(AssertionError):
            compare_values([b'V exact 0x8.000000000000001p-3'], [b'V exact 0x8p-3'])

    def test_x87_extremes_and_signed_zero(self):
        self.assertGreater(native_number('0x0.000000000000001p-16385'), 0)
        self.assertTrue(native_number('0xf.fffffffffffffffp+16380').is_finite())
        self.assertTrue(native_number('-0x0p+0').is_signed())
        self.assertTrue(native_number('-nan').is_signed())
        with self.assertRaises(AssertionError):
            compare_values([b'V polynomial -0x0p+0'], [b'V polynomial 0x0p+0'])


if __name__ == '__main__':
    unittest.main()
