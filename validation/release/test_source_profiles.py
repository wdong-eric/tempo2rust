"""Keep source observers neutral and localize the first differing recorded field."""
from decimal import Decimal
from pathlib import Path
import tempfile
import unittest
from event_diagnostic import detailed_native, detailed_rust, first_divergence
from spin_profile import equal
from common import ROOT


class SourceProfiles(unittest.TestCase):
    def test_trace_preserves_accumulation_statements_before_bootstrap(self):
        # Harness controls run before native checkout/bootstrap. The live recipe
        # separately asserts these statements against its identity-bound source.
        statements=['phase4 +=  dm*powl(freq, gamma)*exp(-dt/tau)*psr[p].param[param_f].val[0];',
                    'phase4 += val*psr[p].param[param_f].val[0];']
        observed=detailed_native('\n'.join(statements))
        for statement in statements:
            self.assertEqual(observed.count(statement),1)
        source=(ROOT/'rust/src/residual_source.rs').read_text()
        observed=detailed_rust(source)
        for name in ['dm','val']:
            start=source.index('phase4 += Float::with_val(PRECISION_BITS, &'+name+')')
            statement=source[start:source.index('.val;',start)+len('.val;')]
            self.assertEqual(observed.count(statement),1)

    def test_first_divergence_keeps_order_and_zero_sign(self):
        with tempfile.TemporaryDirectory() as directory:
            native=Path(directory)/'native';rust=Path(directory)/'rust'
            text='AL_DETAIL EXP 0 0 ordinary_bits 64\nAL_DETAIL EXP 0 0 exponent -0\nAL_DETAIL EXP 0 0 exp 1\n'
            native.write_text(text);rust.write_text(text)
            self.assertIsNone(first_divergence(native,rust)['first'])
            rust.write_text(text.replace('exp 1','exp 2'))
            self.assertEqual(first_divergence(native,rust)['first']['field'],'exp')
            rust.write_text(text.replace('exponent -0','exponent 0').replace('exp 1','exp 2'))
            self.assertEqual(first_divergence(native,rust)['first']['field'],'exponent')

    def test_source_profile_classes_remain_distinct(self):
        for a,b in [('NaN','-NaN'),('0','-0'),('Infinity','-Infinity'),('1','1.1')]:
            self.assertFalse(equal(Decimal(a),Decimal(b),64))
        self.assertTrue(equal(Decimal('-NaN'),Decimal('-NaN'),64))


if __name__=='__main__':unittest.main()
