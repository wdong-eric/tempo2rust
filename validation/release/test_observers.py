import unittest
from af_compare import NativeChecks
from m2 import observer_differences
from polyco_serialization import legacy_checks


class ObserverTests(unittest.TestCase):
    def tearDown(self):NativeChecks.bits=53

    def test_nan_sign_is_the_only_observer_correction(self):
        self.assertEqual(observer_differences({'x':['NaN','-0','1']},{'x':['-NaN','-0','1']}),1)
        for before,after in [('0','-0'),('1','1.00001'),('-NaN','NaN'),('NaN','1')]:
            with self.assertRaises(AssertionError):observer_differences(before,after)

    def test_writer_parser_contract_rejects_wrong_token_and_decoded_value(self):
        NativeChecks.bits=64
        original=[{'coeff':['0.100000000000000005551115123125782702']}]
        parsed=[{'coeff':['0.100000000000000006001601775495093875']}]
        legacy=[['1.00000000000000006e-01']]
        modern=[['1.000000000000000055511151231258e-01']]
        self.assertFalse(legacy_checks(original,parsed,legacy,modern).failures)
        self.assertTrue(legacy_checks(original,original,legacy,modern).failures)
        self.assertTrue(legacy_checks(original,parsed,[['1.00000000000000007e-01']],modern).failures)
        # A wide interval containing a double midpoint must not be guessed.
        self.assertTrue(legacy_checks([{'coeff':['1']}],[{'coeff':['1']}],[['1.00000000000000000e+00']],
            [['1.000000000000000111022302462516e+00']]).failures)

    def test_writer_zero_sign_and_binary64_exponent_boundaries(self):
        NativeChecks.bits=64
        for value in ['0','-0']:
            model=[{'coeff':[value]}]
            token=format(float(value),'.17e')
            wide=format(float(value),'.30e')
            self.assertFalse(legacy_checks(model,model,[[token]],[[wide]]).failures)
            wrong=[{'coeff':['-0' if value=='0' else '0']}]
            self.assertTrue(legacy_checks(model,wrong,[[token]],[[wide]]).failures)
            self.assertTrue(legacy_checks(model,model,[[('-' if value=='0' else '')+token.lstrip('-')]],[[wide]]).failures)
        for value in ['1e-320','-1e-330','1e309']:
            source=[{'coeff':[value]}];token=format(float(value),'.17e')
            parsed=[{'coeff':[token]}]
            from decimal import Decimal
            wide=format(Decimal(value),'.30e')
            self.assertFalse(legacy_checks(source,parsed,[[token]],[[wide]]).failures)


if __name__=='__main__':unittest.main()
