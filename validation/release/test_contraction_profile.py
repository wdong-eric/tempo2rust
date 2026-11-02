"""Fail closed if an offset observer changes its source or replayed intermediates."""
from pathlib import Path
import tempfile
import unittest
from contraction_profile import observed_ephemeris, check_offsets


class OffsetWitnessTests(unittest.TestCase):
    def test_observer_retains_each_source_expression(self):
        text='\n'.join(
            f'm = (psr[p].telD{axis}_v[k]-psr[p].telD{axis}_v[k+1])/(psr[p].telD{axis}_t[k]-psr[p].telD{axis}_t[k+1]);\n'
            f'c = psr[p].telD{axis}_v[k]-m*psr[p].telD{axis}_t[k];\n'
            'ival = m*(double)psr[p].obsn[i].sat+c;\nbreak;'
            for axis in 'XYZ')
        observed,expression=observed_ephemeris(text)
        for statement in text.splitlines():
            self.assertEqual(observed.count(statement),text.count(statement))
        self.assertEqual(''.join(expression.split()),'m=(v0-v1)/(t0-t1);c=v0-m*t0;ival=m*(double)sat+c;')
        with self.assertRaises(AssertionError):
            observed_ephemeris(text.replace('telDY_v[k]-m*','telDY_v[k]+m*'))

    def test_each_observed_intermediate_and_signed_zero_must_match(self):
        with tempfile.TemporaryDirectory() as temporary:
            path=Path(temporary)/'trace.tsv'
            row=['1','0','0','0']+['0000000000000000']*11
            path.write_text('\t'.join(row)+'\n')
            self.assertEqual(len(check_offsets(path)),1)
            for field in [9,10,11,12,13,14]:
                changed=list(row);changed[field]='8000000000000000'
                path.write_text('\t'.join(changed)+'\n')
                with self.assertRaises(AssertionError):check_offsets(path)
            path.write_text('')
            with self.assertRaises(AssertionError):check_offsets(path)


if __name__=='__main__':unittest.main()
