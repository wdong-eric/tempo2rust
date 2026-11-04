"""0145 baseline reuse, exact gates and actual Linux midpoint/range selection."""
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch
import integrated
from common import ROOT
import test_allocation_wiring as allocation_controls


class TemporaryWiring(unittest.TestCase):
    def test_pair_reuses_coefficient_endpoint_and_compiles_before_all_measurements(self):
        with tempfile.TemporaryDirectory() as directory,patch.object(integrated.platform,'system',return_value='Linux'):
            run=allocation_controls.AllocationWiring().matrix(Path(directory),{})
            commands={r['label']:r['argv'] for r in run.commands};names=list(commands)
            self.assertEqual(integrated.TEMPORARY_REVISIONS,{
                'before':'a1851c55fcc0b1ac733b064e631bfbbd2e0118d5',
                'after':'e1d34d835377451dcc7822161afce96a87948005'})
            self.assertEqual(integrated.TEMPORARY_REVISIONS['before'],integrated.COEFFICIENT_REVISIONS['after'])
            for name in ['temporary-build-before','temporary-direct-build-before','temporary-qualify-before']:
                self.assertNotIn(name,commands)
            before=run.out/'predictor-performance/coefficient-view'
            for name in ['temporary-pair-verify','temporary-pair-consumers','temporary-witness']:
                self.assertIn(str(before/'after/prefix'),commands[name])
            self.assertIn(str(before/'qualify-after'),commands['temporary-pair-verify'])
            self.assertIn(str(before/'after-direct'),commands['temporary-direct-verify'])
            self.assertIn(str(before/'after-direct/plain'),commands['temporary-direct-measure'])
            self.assertIn(str(before/'qualify-after/native-getPeriod'),commands['temporary-pair-measure'])
            self.assertIn(str(run.out/'candidate/source/validation/predictor_performance/coefficient_verify.py'),commands['temporary-witness'])
            self.assertIn(str(ROOT/'validation/release/c_concurrency.py'),commands['temporary-pair-concurrency'])
            self.assertEqual(commands['temporary-object'][0:2],['objdump','-Cd'])
            self.assertEqual(commands['temporary-object'][-1],str(run.out/'predictor-performance/temporary-reuse/after/prefix/lib/libtempo2pred_rust.so'))
            for name in ['temporary-direct-verify','temporary-pair-verify','temporary-direct-measure','temporary-pair-measure']:
                self.assertIn(str(run.out/'workers/performance/cheby-native-1-2/t2pred.dat'),commands[name])
            for name in ['predictor-pair-measure','evaluator-direct-measure','evaluator-pair-measure',
                         'recurrence-direct-measure','recurrence-pair-measure','coefficient-direct-measure','coefficient-pair-measure',
                         'temporary-direct-measure','temporary-pair-measure']:
                for boundary in ['temporary-object','temporary-wide-range-release','gui-evidence']:
                    self.assertGreater(names.index(name),names.index(boundary))
                self.assertIn('100000',commands[name]);self.assertIn('7',commands[name])
            self.assertEqual(run.saved['temporary_performance']['revisions'],integrated.TEMPORARY_REVISIONS)
            self.assertIn('("temporary-performance", TEMPORARY_REVISIONS)',(ROOT/'.github/workflows/tempo2-rust.yml').read_text())

    def test_missing_sources_and_failed_contracts_block_only_dependents(self):
        for failure,blocked,independent in [
            ('temporary-source-after','temporary-pair-verify','coefficient-pair-measure'),
            ('coefficient-source-after','temporary-pair-verify','temporary-pair-tests-release'),
            ('temporary-witness','temporary-pair-measure','temporary-direct-measure'),
            ('temporary-direct-verify','temporary-direct-measure','temporary-pair-measure'),
            ('temporary-pair-verify','temporary-pair-measure','temporary-direct-measure'),
            ('predictor-workload','temporary-direct-verify','temporary-witness')]:
            with self.subTest(failure=failure),tempfile.TemporaryDirectory() as directory:
                run=allocation_controls.AllocationWiring().matrix(Path(directory),{failure:1})
                self.assertIsNone(run.saved['stages'][blocked]['exit'])
                self.assertEqual(run.saved['stages'][independent]['exit'],0)

    def test_native_or_range_failure_remains_failed_without_erasing_other_evidence(self):
        for failure in ['temporary-pair-native-gate','temporary-wide-range-debug','temporary-wide-range-release']:
            with self.subTest(failure=failure),tempfile.TemporaryDirectory() as directory,patch.object(integrated.platform,'system',return_value='Linux'):
                run=allocation_controls.AllocationWiring().matrix(Path(directory),{failure:1})
                self.assertIn(failure,run.saved['failed'])
                for name in ['temporary-witness','temporary-pair-concurrency','temporary-pair-consumers',
                             'temporary-direct-measure','temporary-pair-measure','gui-evidence']:
                    self.assertEqual(run.saved['stages'][name]['exit'],0)

    def test_each_range_and_midpoint_test_must_execute_in_each_profile(self):
        tests=['cheby2d::evaluation::platform::tests::predictor_recurrence_linux_keeps_wide_inputs',
               'predictor_tests::predictor_owned_operations_preserve_rounding_and_classes',
               'predictor_tests::predictor_native_subnormal_single_rounding',
               'predictor_tests::predictor_owned_subnormal_midpoint_keeps_first_rounding_direction']
        with tempfile.TemporaryDirectory() as directory,patch.object(integrated.platform,'system',return_value='Linux'):
            run=allocation_controls.AllocationWiring().matrix(Path(directory),{})
            commands={r['label']:r['argv'] for r in run.commands}
            for profile in ['debug','release']:
                command=commands['temporary-wide-range-'+profile];path=Path(command[-1])
                self.assertEqual(path.name,'temporary-pair-tests-'+profile+'.stdout')
                lines=['test '+name+' ... ok' for name in tests]
                for text,passes in [('\n'.join(lines),True),('running 0 tests',False),
                    *[('\n'.join(lines[:i]+lines[i+1:]),False) for i in range(4)],
                    *[('\n'.join(lines).replace(name+' ... ok',name+' ... '+state),False)
                      for name in tests for state in ['ignored','FAILED']]]:
                    path.write_text(text)
                    self.assertEqual(subprocess.run(command,capture_output=True).returncode==0,passes)


if __name__=='__main__':unittest.main()
