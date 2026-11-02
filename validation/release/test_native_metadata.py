"""Reject non-metadata differences at the measured Linux failure boundaries."""
from pathlib import Path
import ast
import json
import tempfile
import unittest
from native_metadata import assertion_emitter, cheby_assertion, loader_identity, precision_profile, preservation_expected
from common import ROOT
from corpus_compare import stage_preservation


class NativeMetadata(unittest.TestCase):
    def test_gnu_assertion_requires_source_predicate_and_abort(self):
        text="asan: validation/jpl/build/source/jpleph.c:326: void interp(int): Assertion `ncf < MAX_CHEBY' failed."
        self.assertEqual(cheby_assertion(text,-6),['ncf < MAX_CHEBY'])
        for changed,status in [(text,-11),(text,0),(text.replace(':326:',':327:'),-6),
                (text.replace('ncf < MAX_CHEBY','ncf <= MAX_CHEBY'),-6),
                (text+'\nSUMMARY: AddressSanitizer: heap-buffer-overflow',-6)]:
            with self.assertRaises(AssertionError):cheby_assertion(changed,status)

    def test_assertion_label_is_bound_to_the_actual_executable(self):
        tail="{source}/TKmatrix.C:171: double** malloc_blas(size_t, size_t): Assertion `rows*cols > 0' failed.\n"
        expected=assertion_emitter('probe: '+tail,Path('/tmp/probe'),-6)
        self.assertEqual(expected,assertion_emitter('coverage: '+tail,Path('/tmp/coverage'),-6))
        self.assertNotEqual(expected,assertion_emitter('fixed: '+tail.replace('> 0','>= 0'),Path('/tmp/fixed'),-6))
        with self.assertRaises(AssertionError):assertion_emitter('other: '+tail,Path('/tmp/probe'),-6)
        with self.assertRaises(AssertionError):assertion_emitter('probe: '+tail,Path('/tmp/probe'),-11)

    def test_loader_only_ignores_addresses(self):
        with tempfile.TemporaryDirectory() as directory:
            library=Path(directory)/'libexample.so';library.write_bytes(b'original')
            lines=['\tlinux-vdso.so.1 (0x1234)',f'\tlibexample.so => {library} (0x5678)']
            expected=loader_identity(lines)
            self.assertEqual(expected,loader_identity([line.replace('1234','4321').replace('5678','8765') for line in lines]))
            library.write_bytes(b'changed')
            self.assertNotEqual(expected,loader_identity(lines))
            self.assertNotEqual(expected,loader_identity(list(reversed(lines))))
            with self.assertRaises(AssertionError):loader_identity(['libexample.so => not found'])
            with self.assertRaises(AssertionError):loader_identity(['unclassified loader failure'])

    def test_precision_requires_consistent_qualified_roles(self):
        names=['__SIZEOF_DOUBLE__','__DBL_MANT_DIG__','__SIZEOF_LONG_DOUBLE__','__LDBL_MANT_DIG__','__SIZEOF_FLOAT128__','__FLT128_MANT_DIG__']
        def macros(values):return '\n'.join(f'#define {name} {value}' for name,value in zip(names,values,strict=True))
        for values in [[8,53,8,53,16,113],[8,53,16,64,16,113]]:
            self.assertEqual(precision_profile(macros(values)),values)
            self.assertEqual(precision_profile(macros(values).replace('#define __SIZEOF_FLOAT128__ 16\n','')),values)
        with self.assertRaises(AssertionError):precision_profile('')
        for values in [[8,53,8,64,16,113],[8,53,16,64,16,128],[4,24,16,64,16,113]]:
            with self.assertRaises(AssertionError):precision_profile(macros(values))

    def test_preservation_only_substitutes_a_bound_staging_patch(self):
        original={'catalog.py':'old','fixture.par':'unchanged'}
        changes={'catalog.py':{'original':'old','adapted':'new'}}
        expected=preservation_expected(original,changes)
        self.assertEqual(expected,{'catalog.py':'new','fixture.par':'unchanged'})
        self.assertEqual(original['catalog.py'],'old')
        self.assertNotEqual(expected,{'catalog.py':'unexpected','fixture.par':'unchanged'})
        with self.assertRaises(AssertionError):preservation_expected({'catalog.py':'different'},changes)

    def test_staged_par_preservation_needs_no_runner_root_global(self):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);here=root/'validation/par_specialized';here.mkdir(parents=True)
            paths=[here/name for name in ['run.py','verify.py']]
            for path in paths:path.write_bytes((ROOT/'validation/par_specialized'/path.name).read_bytes())
            protected=here/'protected.json'
            protected.write_text(json.dumps({'catalog.py':'old','fixture.par':'unchanged'}))
            (root/'al-source-adaptations.json').write_text(json.dumps({'catalog.py':{'original':'old','adapted':'new'}}))
            stage_preservation(root,lambda path,text:path.write_text(text),['par_specialized'])
            # Execute both actual before/after check expressions from the staged
            # historical runner, whose namespace has HERE and Path but no ROOT.
            expressions=[n for path in paths for n in ast.walk(ast.parse(path.read_text()))
                         if isinstance(n,ast.Call) and isinstance(n.func,ast.Name)
                         and n.func.id=='preservation_expected']
            self.assertEqual(len(expressions),3)
            namespace={'HERE':here,'Path':Path,'json':json,'preservation_expected':preservation_expected}
            for expression in expressions:
                code=compile(ast.Expression(expression),str(path),'eval')
                self.assertEqual(eval(code,namespace),{'catalog.py':'new','fixture.par':'unchanged'})
                protected.write_text(json.dumps({'catalog.py':'unexpected'}))
                with self.assertRaises(AssertionError):eval(code,namespace)
                protected.write_text(json.dumps({'catalog.py':'old','fixture.par':'unchanged'}))


if __name__=='__main__':unittest.main()
