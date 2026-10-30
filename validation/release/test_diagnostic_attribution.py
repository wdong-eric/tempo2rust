"""Reject false locations and retain ordered/numeric diagnostic content (0107)."""
from pathlib import Path
import unittest
from diagnostic_attribution import Attribution
from attribute_generated_output import attribute

ROOT=Path(__file__).resolve().parents[2]


class DiagnosticAttribution(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mapping=Attribution(ROOT)

    def test_location_then_body_and_order(self):
        m=self.mapping
        (file,line),native=next((site,value) for site,value in m.sites.items() if site[0]=='general_fitting.rs')
        prefix=f'[{file}:{line}]'.encode();old=f'[{native[0]}:{native[1][0]}]'.encode()
        before=old+b' value=1.25 user-file.C\n'+old+b' next\n'
        after=prefix+b' value=1.25 user-file.C\n'+prefix+b' next\n'
        expected=m.stream(before,'native')
        self.assertEqual(m.stream(after,'rust'),expected)
        for changed in [after.replace(b'1.25',b'1.26'),after.splitlines(keepends=True)[0],b''.join(reversed(after.splitlines(keepends=True)))]:
            self.assertNotEqual(m.stream(changed,'rust'),expected)
        for changed in [after.replace(file.encode(),b'fictional.rs'),after.replace(f':{line}]'.encode(),b':1]'),before]:
            with self.assertRaises(AssertionError):m.stream(changed,'rust')
        data={'stdout':after.decode(),'model.par':after.decode(),'value':1.25}
        projected=m.structured(data,'rust')
        self.assertEqual(projected['stdout'].encode(),expected)
        self.assertEqual(projected['model.par'],data['model.par'])
        self.assertEqual(projected['value'],1.25)

    def test_version_identity_counterpart_and_site(self):
        m=self.mapping;(file,line),(native,function)=next(iter(m.versions.items()))
        row=f'[TEMPO2-RUST VERSION:] [{file}:{line}] [TEMPO2 counterpart: {native}::{function}]\n'.encode()
        old=f'[TEMPO2 VERSION:] [tempo2.h            ] [                    ] [2026.09.1           ]\n[TEMPO2 VERSION:] [{native:<20.20}] [{function:<20.20}] [$Id$                ]\n'.encode()
        self.assertEqual(m.stream(m.identity+row,'rust'),m.stream(old,'native'))
        for changed in [row.replace(file.encode(),b'wrong.rs'),row.replace(f':{line}]'.encode(),b':1]'),row.replace(native.encode(),b'wrong.C'),m.identity.replace(b'0.1.0',b'9.9.9')]:
            with self.assertRaises(AssertionError):m.stream(changed,'rust')

    def test_long_line_is_not_repaired_or_dropped(self):
        m=self.mapping;(file,line),native=next((site,value) for site,value in m.sites.items() if site[0]=='general_fitting.rs')
        before=(f'[{native[0]}:{native[1][0]}] '.encode()+b'x'*400+b'\n')[:359]
        after=(f'[{file}:{line}] '.encode()+b'x'*400+b'\n')[:359]
        self.assertNotEqual(m.stream(before,'native'),m.stream(after,'rust'))
        self.assertEqual(m.stream(after,'rust').count(b'x'),after.count(b'x'))
        self.assertFalse(m.stream(after,'rust').endswith(b'\n'))

    def test_internal_comparison_tags_cannot_enter_as_raw_output(self):
        m=self.mapping
        (file,line),native=next((site,value) for site,value in m.sites.items() if site[0]=='general_fitting.rs')
        raw=f'[{file}:{line}] value=1.25\n'.encode()
        changed=raw.replace(f'[{file}:{line}]'.encode(),m.native[native[0],native[1][0]])
        (file,line),(native,function)=next(iter(m.versions.items()))
        row=f'[TEMPO2-RUST VERSION:] [{file}:{line}] [TEMPO2 counterpart: {native}::{function}]\n'.encode()
        internal_row=f'[TEMPO2 SOURCE VERSION:] {native}::{function}\n'.encode()
        # These substitutions previously compared equal to genuine old output.
        for altered in [changed,b'\x1b[33m'+changed,m.header+row,m.identity+internal_row]:
            for kind in ['rust','native']:
                with self.subTest(kind=kind,altered=altered):
                    with self.assertRaisesRegex(AssertionError,'internal comparison label'):
                        m.stream(altered,kind)
        # A native filename or tag inside message data remains ordinary data.
        body=b' literal [TEMPO2:t2fit.C:850] [TEMPO2 SOURCE VERSION:]\n'
        self.assertTrue(m.stream(raw.rstrip()+body,'rust').endswith(body))

    def test_generator_successor_changes_only_location_arguments(self):
        source='d.log(b\'w\', "textOutput.C", 1847, &format("value %g", &[x.into()])?);\n'
        result,count=attribute(source)
        self.assertEqual(count,1)
        self.assertIn('d.rust_log(/* TEMPO2 textOutput.C:1847 */ b\'w\', &format("value %g", &[x.into()])?);',result)
        with self.assertRaises(AssertionError):attribute(result)
        with self.assertRaises(AssertionError):attribute(source+'d.log(b\'m\', "unknown.C", 1, b"unknown");')


if __name__=='__main__':unittest.main()
