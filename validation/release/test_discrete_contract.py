"""0127 target binding and rejection through the release comparators."""
import copy
from fractions import Fraction
import unittest
from unittest.mock import patch
import boundaries
from discrete_contract import utility_target, resolve_simul


class DiscreteContract(unittest.TestCase):
    def test_exact_utility_targets_and_original_failure(self):
        p='0x1.999999999999999999999999999ap-4'
        below='0x1.ffffffffffffffffffffffffffffp-2'
        rows=[['half_below','nlong',below,'0','1'],
              ['negative_half_inside','nlong','-'+below,'0','-1'],
              ['mod_rounded_quotient','mod','1',p,'0'],
              ['mod_negative_quotient','mod','-1',p,'0']]
        rem=Fraction(4153837486827862102824397063376075,2**115)
        expected=[('finite',False,Fraction(0)),('finite',True,Fraction(0)),
                  ('finite',False,rem),('finite',True,-rem)]
        self.assertEqual([utility_target(*r[:4]) for r in rows],expected)
        actual=[[r[0],'finite',str(sign).lower(),str(v.numerator),str(-(v.denominator.bit_length()-1))]
                for r,(_,sign,v) in zip(rows,expected)]
        actual[0][-1]=actual[1][-1]='-1073741823'
        with patch.object(boundaries,'NAMES',[r[0] for r in rows]):
            report=boundaries.comparison(rows,actual)
            self.assertEqual((report['failures'],report['original_failures']),(0,4))
            for index,field,value in [(0,3,'1'),(1,2,'false'),(2,1,'NaN'),(3,2,'false')]:
                bad=copy.deepcopy(actual);bad[index][field]=value
                if field==3:bad[index][-1]='0'
                self.assertGreater(boundaries.comparison(rows,bad)['failures'],0)
            for bad in [actual[:-1],actual+[actual[0]],list(reversed(actual))]:
                with self.assertRaisesRegex(AssertionError,'membership/order'):boundaries.comparison(rows,bad)
            changed=copy.deepcopy(rows);changed[0][-1]='2'
            with self.assertRaisesRegex(AssertionError,'native witness'):boundaries.comparison(changed,actual)
        with self.assertRaises(AssertionError):utility_target('half_below','nlong','0.5','0')

    def test_unaffected_class_zero_sign_and_native_value_gates(self):
        rows=[['mod_negative_exact','mod','-4','2','0'],['mod_infinite_divisor','mod','1','inf','nan'],
              ['half','nlong','0.5','0','1']]
        actual=[['mod_negative_exact','finite','false','0','-1073741823'],
                ['mod_infinite_divisor','NaN','false','0','0'],['half','finite','false','1','0']]
        with patch.object(boundaries,'NAMES',[r[0] for r in rows]):
            self.assertEqual(boundaries.comparison(rows,actual)['failures'],0)
            for index,field,value in [(0,2,'true'),(1,1,'finite'),(2,3,'0'),(2,2,'true'),(2,2,'unknown')]:
                bad=copy.deepcopy(actual);bad[index][field]=value
                self.assertEqual(boundaries.comparison(rows,bad)['failures'],1)

    def test_complete_simul_differences_not_a_field_allowance(self):
        expected=[{'tag':f'simul_123 {i} simul','field':'p0.obs0.deleted','native':0,'rust':1} for i in [6,7]]
        other={'tag':'units100 0 units','field':'epoch','native':'original','rust':'different'}
        self.assertEqual(resolve_simul(expected+[other],expected),[other])
        wrong=copy.deepcopy(expected);wrong[0]['rust']=0
        unrelated={**expected[0],'field':'p0.obs1.freq'}
        missing_state={**expected[0],'error':'membership','missing':['p0.obs0.sat'],'extra':[]}
        for actual in [wrong,expected[:-1],expected+[unrelated],[missing_state,*expected],expected[::-1]]:
            with self.assertRaises(AssertionError):resolve_simul(actual,expected)


if __name__=='__main__':unittest.main()
