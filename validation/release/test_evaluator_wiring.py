"""0128 frozen baseline reuse, independent gates and separate timing scopes."""
from pathlib import Path
import tempfile
import unittest
import integrated
import test_allocation_wiring as allocation_controls


class EvaluatorWiring(unittest.TestCase):
    def test_exact_pair_reuses_install_and_workload_with_plain_timing_clients(self):
        with tempfile.TemporaryDirectory() as directory:
            run=allocation_controls.AllocationWiring().matrix(Path(directory),{})
            commands={row['label']:row['argv'] for row in run.commands}
            self.assertEqual(integrated.EVALUATOR_REVISIONS,{
                'before':'0bd8e2cf2ef54e9da6feace122acd8aa96aa3fbe',
                'after':'8716d1dc026641f3a51c2ba88850c5599293cbdd'})
            self.assertEqual(integrated.EVALUATOR_REVISIONS['before'],integrated.PREDICTOR_REVISIONS['after'])
            self.assertNotIn('evaluator-build-before',commands)
            self.assertIn(integrated.EVALUATOR_REVISIONS['after'],commands['evaluator-build-after'])
            before=str(run.out/'predictor-performance/after/prefix')
            for name in ['evaluator-pair-verify','evaluator-direct-build-before','evaluator-pair-consumers']:
                self.assertIn(before,commands[name])
            verify=commands['evaluator-pair-verify']
            self.assertEqual(verify[verify.index('--before-qualification')+1],str(run.out/'predictor-performance/qualify-after'))
            names=list(commands)
            for name in ['evaluator-direct-verify','evaluator-pair-verify','evaluator-direct-measure','evaluator-pair-measure']:
                self.assertIn(str(run.out/'workers/performance/cheby-native-1-2/t2pred.dat'),commands[name])
            for name in ['predictor-pair-measure','evaluator-direct-measure','evaluator-pair-measure']:
                self.assertGreater(names.index(name),names.index('evaluator-pair-tests-release'))
                self.assertIn('100000',commands[name]);self.assertIn('7',commands[name])
            direct=commands['evaluator-direct-measure']
            for label in ['before','after']:
                self.assertEqual(direct[direct.index('--'+label)+1],str(run.out/f'predictor-performance/evaluator/{label}-direct/plain'))
            full=commands['evaluator-pair-measure']
            self.assertEqual(full[full.index('--native-period')+1],str(run.out/'predictor-performance/qualify-after/native-getPeriod'))
            self.assertEqual(run.saved['evaluator_performance']['revisions'],integrated.EVALUATOR_REVISIONS)

    def test_missing_after_blocks_only_its_dependents(self):
        with tempfile.TemporaryDirectory() as directory:
            run=allocation_controls.AllocationWiring().matrix(Path(directory),{'evaluator-source-after':128})
            stages=run.saved['stages']
            for name in ['evaluator-build-after','evaluator-direct-verify','evaluator-pair-verify',
                         'evaluator-direct-measure','evaluator-pair-measure','evaluator-pair-consumers']:
                self.assertIsNone(stages[name]['exit'],name)
            for name in ['evaluator-direct-build-before','predictor-pair-measure','allocation-verify','c-default']:
                self.assertEqual(stages[name]['exit'],0,name)

    def test_native_gate_failure_stays_failed_without_hiding_other_checks(self):
        with tempfile.TemporaryDirectory() as directory:
            run=allocation_controls.AllocationWiring().matrix(Path(directory),{'evaluator-pair-native-gate':1})
            self.assertIn('evaluator-pair-native-gate',run.saved['failed'])
            for name in ['evaluator-direct-verify','evaluator-direct-measure','evaluator-pair-measure',
                         'evaluator-pair-consumers','evaluator-pair-concurrency',
                         'evaluator-pair-tests-debug','evaluator-pair-tests-release']:
                self.assertEqual(run.saved['stages'][name]['exit'],0,name)

    def test_direct_and_complete_c_verification_block_their_own_measurements(self):
        cases=[('evaluator-direct-verify','evaluator-direct-measure','evaluator-pair-measure'),
               ('evaluator-pair-verify','evaluator-pair-measure','evaluator-direct-measure'),
               ('predictor-qualify-after','evaluator-pair-measure','evaluator-direct-measure')]
        for failure,blocked,independent in cases:
            with self.subTest(failure=failure),tempfile.TemporaryDirectory() as directory:
                run=allocation_controls.AllocationWiring().matrix(Path(directory),{failure:1})
                self.assertIsNone(run.saved['stages'][blocked]['exit'])
                self.assertEqual(run.saved['stages'][independent]['exit'],0)

    def test_failed_bound_workload_blocks_both_scopes_but_not_consumers(self):
        with tempfile.TemporaryDirectory() as directory:
            run=allocation_controls.AllocationWiring().matrix(Path(directory),{'predictor-workload':1})
            for name in ['evaluator-direct-verify','evaluator-pair-verify','evaluator-direct-measure','evaluator-pair-measure']:
                self.assertIsNone(run.saved['stages'][name]['exit'],name)
            self.assertEqual(run.saved['stages']['evaluator-pair-consumers']['exit'],0)


if __name__=='__main__':unittest.main()
