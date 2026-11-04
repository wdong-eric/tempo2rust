"""0142 frozen coefficient-view baseline, independent gates and range execution."""
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch
import integrated
from common import ROOT
import test_allocation_wiring as allocation_controls


class CoefficientWiring(unittest.TestCase):
    def test_pair_reuses_recurrence_and_finishes_compilation_before_measurements(self):
        with tempfile.TemporaryDirectory() as directory,patch.object(integrated.platform,'system',return_value='Linux'):
            run=allocation_controls.AllocationWiring().matrix(Path(directory),{})
            commands={r['label']:r['argv'] for r in run.commands};names=list(commands)
            self.assertEqual(integrated.COEFFICIENT_REVISIONS,{
                'before':'e24622d5210fdfc8abe10f9f58a21b7e4a2a1aca',
                'after':'a1851c55fcc0b1ac733b064e631bfbbd2e0118d5'})
            self.assertEqual(integrated.COEFFICIENT_REVISIONS['before'],integrated.RECURRENCE_REVISIONS['after'])
            for name in ['coefficient-build-before','coefficient-direct-build-before','coefficient-qualify-before']:
                self.assertNotIn(name,commands)
            before=run.out/'predictor-performance/recurrence'
            for name in ['coefficient-pair-verify','coefficient-pair-consumers','coefficient-witness']:
                self.assertIn(str(before/'after/prefix'),commands[name])
            self.assertIn(str(before/'qualify-after'),commands['coefficient-pair-verify'])
            self.assertIn(str(before/'after-direct'),commands['coefficient-direct-verify'])
            self.assertIn(str(before/'after-direct/plain'),commands['coefficient-direct-measure'])
            self.assertIn(str(before/'qualify-after/native-getPeriod'),commands['coefficient-pair-measure'])
            self.assertIn(str(run.out/'candidate/source/validation/predictor_performance/coefficient_verify.py'),commands['coefficient-witness'])
            for name in ['coefficient-direct-verify','coefficient-pair-verify','coefficient-direct-measure','coefficient-pair-measure']:
                self.assertIn(str(run.out/'workers/performance/cheby-native-1-2/t2pred.dat'),commands[name])
            for name in ['predictor-pair-measure','evaluator-direct-measure','evaluator-pair-measure',
                         'recurrence-direct-measure','recurrence-pair-measure','coefficient-direct-measure','coefficient-pair-measure']:
                self.assertGreater(names.index(name),names.index('coefficient-object'))
                self.assertGreater(names.index(name),names.index('coefficient-wide-range-release'))
                self.assertIn('100000',commands[name]);self.assertIn('7',commands[name])
            self.assertEqual(run.saved['coefficient_performance']['revisions'],integrated.COEFFICIENT_REVISIONS)
            self.assertIn('("coefficient-performance", COEFFICIENT_REVISIONS)',(ROOT/'.github/workflows/tempo2-rust.yml').read_text())

    def test_missing_prerequisites_and_contract_failures_preserve_independent_routes(self):
        for failure,blocked,independent in [
            ('coefficient-source-after','coefficient-pair-verify','recurrence-pair-measure'),
            ('recurrence-source-after','coefficient-pair-verify','coefficient-pair-tests-release'),
            ('coefficient-witness','coefficient-pair-measure','coefficient-direct-measure'),
            ('coefficient-direct-verify','coefficient-direct-measure','coefficient-pair-measure'),
            ('coefficient-pair-verify','coefficient-pair-measure','coefficient-direct-measure'),
            ('predictor-workload','coefficient-direct-verify','coefficient-witness')]:
            with self.subTest(failure=failure),tempfile.TemporaryDirectory() as directory:
                run=allocation_controls.AllocationWiring().matrix(Path(directory),{failure:1})
                self.assertIsNone(run.saved['stages'][blocked]['exit'])
                self.assertEqual(run.saved['stages'][independent]['exit'],0)

    def test_native_failure_remains_failed_without_hiding_pair_evidence(self):
        with tempfile.TemporaryDirectory() as directory:
            run=allocation_controls.AllocationWiring().matrix(Path(directory),{'coefficient-pair-native-gate':1})
            self.assertIn('coefficient-pair-native-gate',run.saved['failed'])
            for name in ['coefficient-witness','coefficient-pair-concurrency','coefficient-pair-consumers',
                         'coefficient-direct-measure','coefficient-pair-measure']:
                self.assertEqual(run.saved['stages'][name]['exit'],0)

    def test_every_linux_range_witness_must_execute_and_pass(self):
        tests=['cheby2d::evaluation::platform::tests::predictor_recurrence_linux_keeps_wide_inputs',
               'predictor_tests::predictor_owned_operations_preserve_rounding_and_classes',
               'predictor_tests::predictor_native_subnormal_single_rounding']
        with tempfile.TemporaryDirectory() as directory,patch.object(integrated.platform,'system',return_value='Linux'):
            run=allocation_controls.AllocationWiring().matrix(Path(directory),{})
            commands={r['label']:r['argv'] for r in run.commands}
            for profile in ['debug','release']:
                command=commands['coefficient-wide-range-'+profile];path=Path(command[-1])
                lines=['test '+name+' ... ok' for name in tests]
                for text,passes in [('\n'.join(lines),True),('running 0 tests',False),
                    *[('\n'.join(lines[:i]+lines[i+1:]),False) for i in range(3)],
                    *[('\n'.join(lines).replace(name+' ... ok',name+' ... '+state),False)
                      for name in tests for state in ['ignored','FAILED']]]:
                    path.write_text(text)
                    self.assertEqual(subprocess.run(command,capture_output=True).returncode==0,passes)


if __name__=='__main__':unittest.main()
