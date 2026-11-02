"""Frozen AO pair routing and preservation of supplemental native failures."""
import json
from pathlib import Path
from types import SimpleNamespace
import tempfile
import unittest
from predictor_performance_gate import gate
from predictor_workload import workload
from common import sha
import test_allocation_wiring as allocation_controls
import integrated


class PredictorWiring(unittest.TestCase):
    def test_workload_requires_successful_bound_8x4_native_case(self):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);fixture=root/'validation/predictors/fixtures/isolated.par'
            fixture.parent.mkdir(parents=True);fixture.write_text('versioned isolated PAR')
            performance=root/'performance';predictor=performance/'cheby-native-1-2/t2pred.dat'
            predictor.parent.mkdir(parents=True);predictor.write_text('native generated 8x4 predictor')
            command=['native-tempo2','-f','original/isolated.par','-pred','PKS 58000 58000.03 1000 2000 8 4 3600']
            row={'case':'cheby','engine':'native','workers':1,'sample':2,'exit':0,'command':command}
            record={'records':[row],'identities':{'original/isolated.par':sha(fixture),'native-tempo2':'native executable hash'},
                    'files':{'cheby-native-1-2/t2pred.dat':sha(predictor)}}
            receipt=performance/'receipt.json';receipt.write_text(json.dumps(record))
            self.assertEqual(workload(performance,root)['predictor'],{str(predictor):sha(predictor)})
            row['exit']=1;receipt.write_text(json.dumps(record))
            with self.assertRaisesRegex(ValueError,'successfully'):workload(performance,root)
            row['exit']=0;command[-1]='PKS 58000 58000.03 1000 2000 4 2 3600';receipt.write_text(json.dumps(record))
            with self.assertRaisesRegex(ValueError,'8x4'):workload(performance,root)
            command[-1]='PKS 58000 58000.03 1000 2000 8 4 3600';receipt.write_text(json.dumps(record))
            predictor.write_text('changed product')
            with self.assertRaisesRegex(ValueError,'predictor identity'):workload(performance,root)
            receipt.unlink()
            with self.assertRaises(FileNotFoundError):workload(performance,root)

    def test_negative_nan_witness_remains_a_failure(self):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);receipt=root/'verify.json'
            failure={'line':30257,'native':'T1-CLASS 11 nan sign=1 finite=0 nan=1',
                     'rust':'T1-CLASS 11 nan sign=0 finite=0 nan=1'}
            record={'exact_before_after':True,'inherited_native_failures':[failure],
                    'native_finite_AF_floor_passes':[2,4]}
            receipt.write_text(json.dumps(record));original=receipt.read_bytes()
            self.assertEqual(gate(SimpleNamespace(verification=receipt,out=root/'failed')),1)
            saved=json.loads((root/'failed/receipt.json').read_text())
            self.assertEqual(saved['native_failures'],[failure])
            self.assertEqual(receipt.read_bytes(),original)
            record['inherited_native_failures']=[];receipt.write_text(json.dumps(record))
            self.assertEqual(gate(SimpleNamespace(verification=receipt,out=root/'passed')),0)

    def test_missing_after_does_not_block_before_or_other_campaigns(self):
        with tempfile.TemporaryDirectory() as directory:
            run=allocation_controls.AllocationWiring().matrix(Path(directory),{'predictor-source-after':128})
            stages=run.saved['stages']
            for name in ['predictor-build-after','predictor-pair-verify','predictor-pair-measure',
                         'predictor-pair-consumers','predictor-pair-concurrency']:
                self.assertIsNone(stages[name]['exit'],name)
            for name in ['predictor-build-before','predictor-qualify-before','allocation-verify','c-default']:
                self.assertEqual(stages[name]['exit'],0,name)

    def test_native_failure_keeps_independent_checks_and_measurements(self):
        with tempfile.TemporaryDirectory() as directory:
            run=allocation_controls.AllocationWiring().matrix(Path(directory),{'workers':1,'predictor-pair-native-gate':1})
            stages=run.saved['stages'];commands={r['label']:r for r in run.commands}
            self.assertIn('predictor-pair-native-gate',run.saved['failed'])
            for name in ['predictor-pair-measure','predictor-pair-consumers','predictor-pair-concurrency',
                         'predictor-pair-tests-debug','predictor-pair-tests-release','c-default','c-combined']:
                self.assertEqual(stages[name]['exit'],0,name)
            for label,revision in integrated.PREDICTOR_REVISIONS.items():
                build=commands['predictor-build-'+label]['argv']
                self.assertIn(revision,build)
                self.assertEqual(build[build.index('--out')+1],str(run.out/'predictor-performance'/label))
            measurement=commands['predictor-pair-measure']['argv']
            self.assertIn(str(run.out/'workers/performance/cheby-native-1-2/t2pred.dat'),measurement)
            self.assertIn('100000',measurement);self.assertIn('7',measurement)
            names=[r['label'] for r in run.commands]
            self.assertGreater(names.index('predictor-pair-measure'),names.index('predictor-pair-tests-release'))
            self.assertEqual(run.saved['predictor_performance']['revisions'],integrated.PREDICTOR_REVISIONS)

    def test_failed_qualification_blocks_comparison_but_not_built_consumers(self):
        with tempfile.TemporaryDirectory() as directory:
            run=allocation_controls.AllocationWiring().matrix(Path(directory),{'predictor-qualify-before':1})
            stages=run.saved['stages']
            self.assertIsNone(stages['predictor-pair-verify']['exit'])
            self.assertIsNone(stages['predictor-pair-measure']['exit'])
            self.assertEqual(stages['predictor-pair-consumers']['exit'],0)
            self.assertEqual(stages['predictor-pair-concurrency']['exit'],0)

    def test_missing_workload_blocks_pair_not_abi_or_independent_checks(self):
        with tempfile.TemporaryDirectory() as directory:
            run=allocation_controls.AllocationWiring().matrix(Path(directory),{'workers':1,'predictor-workload':1})
            stages=run.saved['stages']
            self.assertEqual(stages['predictor-workload']['exit'],1)
            self.assertIsNone(stages['predictor-pair-verify']['exit'])
            self.assertIsNone(stages['predictor-pair-measure']['exit'])
            self.assertEqual(stages['predictor-qualify-after']['exit'],0)
            self.assertEqual(stages['predictor-pair-consumers']['exit'],0)


if __name__=='__main__':unittest.main()
