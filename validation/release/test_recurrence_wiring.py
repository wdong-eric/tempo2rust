"""0137 recurrence reuse, independent failure routes and real range witnesses."""
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch
import integrated
from common import ROOT
from test_allocation_wiring import AllocationWiring


class RecurrenceWiring(unittest.TestCase):
    def test_frozen_pair_reuses_bound_baseline_and_compiles_before_measurements(self):
        with tempfile.TemporaryDirectory() as directory,patch.object(integrated.platform,'system',return_value='Linux'):
            run=AllocationWiring().matrix(Path(directory),{})
            commands={r['label']:r['argv'] for r in run.commands};names=list(commands)
            self.assertEqual(integrated.RECURRENCE_REVISIONS,{
                'before':'8716d1dc026641f3a51c2ba88850c5599293cbdd',
                'after':'e24622d5210fdfc8abe10f9f58a21b7e4a2a1aca'})
            self.assertEqual(integrated.RECURRENCE_REVISIONS['before'],integrated.EVALUATOR_REVISIONS['after'])
            self.assertNotIn('recurrence-build-before',commands)
            self.assertNotIn('recurrence-direct-build-before',commands)
            base=run.out/'predictor-performance/evaluator'
            for name in ['recurrence-pair-verify','recurrence-pair-consumers']:
                self.assertIn(str(base/'after/prefix'),commands[name])
            self.assertIn(str(base/'qualify-after'),commands['recurrence-pair-verify'])
            self.assertIn(str(base/'after-direct'),commands['recurrence-direct-verify'])
            self.assertIn(str(base/'after-direct/plain'),commands['recurrence-direct-measure'])
            self.assertIn(str(base/'qualify-after/native-getPeriod'),commands['recurrence-pair-measure'])
            for name in ['recurrence-direct-verify','recurrence-pair-verify','recurrence-direct-measure','recurrence-pair-measure']:
                self.assertIn(str(run.out/'workers/performance/cheby-native-1-2/t2pred.dat'),commands[name])
            for name in ['predictor-pair-measure','evaluator-direct-measure','evaluator-pair-measure',
                         'recurrence-direct-measure','recurrence-pair-measure']:
                self.assertGreater(names.index(name),names.index('recurrence-object'))
                self.assertGreater(names.index(name),names.index('recurrence-wide-range-release'))
                self.assertIn('100000',commands[name]);self.assertIn('7',commands[name])
            self.assertEqual(run.saved['recurrence_performance']['revisions'],integrated.RECURRENCE_REVISIONS)
            workflow=(ROOT/'.github/workflows/tempo2-rust.yml').read_text()
            self.assertIn('("recurrence-performance", RECURRENCE_REVISIONS)',workflow)

    def test_missing_sources_and_contract_failures_only_block_dependents(self):
        for failure,blocked,independent in [
            ('recurrence-source-after','recurrence-pair-verify','evaluator-pair-measure'),
            ('evaluator-source-after','recurrence-pair-verify','recurrence-pair-tests-release'),
            ('recurrence-direct-verify','recurrence-direct-measure','recurrence-pair-measure'),
            ('recurrence-pair-verify','recurrence-pair-measure','recurrence-direct-measure'),
            ('predictor-workload','recurrence-direct-verify','recurrence-pair-consumers')]:
            with self.subTest(failure=failure),tempfile.TemporaryDirectory() as directory:
                run=AllocationWiring().matrix(Path(directory),{failure:1})
                self.assertIsNone(run.saved['stages'][blocked]['exit'])
                self.assertEqual(run.saved['stages'][independent]['exit'],0)

    def test_native_gate_failure_does_not_erase_parity_checks_or_measurements(self):
        with tempfile.TemporaryDirectory() as directory:
            run=AllocationWiring().matrix(Path(directory),{'recurrence-pair-native-gate':1})
            self.assertIn('recurrence-pair-native-gate',run.saved['failed'])
            for name in ['recurrence-pair-consumers','recurrence-pair-concurrency',
                         'recurrence-direct-measure','recurrence-pair-measure']:
                self.assertEqual(run.saved['stages'][name]['exit'],0)

    def test_wide_range_gate_rejects_absent_ignored_and_failed_tests(self):
        with tempfile.TemporaryDirectory() as directory,patch.object(integrated.platform,'system',return_value='Linux'):
            run=AllocationWiring().matrix(Path(directory),{})
            commands={r['label']:r['argv'] for r in run.commands}
            for profile in ['debug','release']:
                command=commands['recurrence-wide-range-'+profile];path=Path(command[-1])
                test='test cheby2d::evaluation::platform::tests::predictor_recurrence_linux_keeps_wide_inputs ... '
                for text,passes in [(test+'ok\n',True),('running 0 tests\n',False),
                                    (test+'ignored\n',False),(test+'FAILED\n',False)]:
                    path.write_text(text)
                    result=subprocess.run(command,capture_output=True)
                    self.assertEqual(result.returncode==0,passes)


if __name__=='__main__':unittest.main()
