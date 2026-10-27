import unittest
from af_compare import NativeChecks, span
from decimal import Decimal as D

class NativeFields(unittest.TestCase):
    def tearDown(self):NativeChecks.bits=53
    def test_wide_exact_field_rejects_binary64_collision(self):
        NativeChecks.bits=64;c=NativeChecks();neighbor=D(1)+D(2)**-63
        c.number('model[0].mjd_mid',neighbor,1)
        self.assertEqual(len(c.failures),1)
        c=NativeChecks();c.number('model[0].dm',neighbor,1)
        self.assertFalse(c.failures)
    def test_measured_metadata_keeps_x87_neighbor(self):
        NativeChecks.bits=64;mid=D(1)+D(2)**-63
        self.assertNotEqual(span(str(mid),'0',1),D(1))
        NativeChecks.bits=53
        self.assertEqual(span(str(mid),'0',1),D(1))

if __name__=='__main__':unittest.main()
