"""Keep allocation-only history, prerequisites and inherited scientific failures explicit."""
import contextlib
import io
import json
from pathlib import Path
from types import SimpleNamespace
import tempfile
import unittest
from unittest.mock import patch
from common import ROOT
import integrated
from allocation_verify import lifecycle_inputs, native_failures


class AllocationWiring(unittest.TestCase):
    def test_s_ordinary_outputs_survive_unrelated_and_coverage_failures(self):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);expected=root/'staging/validation/pulsar_lifecycle/expected'
            expected.mkdir(parents=True);(expected/'matrix_1_1.json.gz').write_bytes(b'bound native record')
            stage={'record_exit':0,'exit':1,'coverage.py':1,'directory':str(expected.parent)}
            receipt=root/'receipt.json'
            receipt.write_text(json.dumps({'status':'failure','stages':{'pulsar_lifecycle':stage,'other':{'exit':1}}}))
            actual,inputs=lifecycle_inputs(root)
            self.assertEqual(actual,expected)
            self.assertIn(str(expected/'matrix_1_1.json.gz'),inputs)
            stage['record_exit']=1
            receipt.write_text(json.dumps({'stages':{'pulsar_lifecycle':stage}}))
            with self.assertRaisesRegex(ValueError,'ordinary recording'):lifecycle_inputs(root)
            receipt.unlink()
            with self.assertRaisesRegex(ValueError,'unavailable'):lifecycle_inputs(root)

    def test_same_engine_equality_does_not_waive_native_gates(self):
        record={'status':'pass_same_engine','lifecycle':[
            {'case':'seeded_reinitialise','engine':engine,'native_gate':'23 != 11'} for engine in ['before','after']],
            'native_fitting':{'before':{'failures':[]},'after':{'failures':[]}}}
        self.assertEqual(len(native_failures(record)),2)
        failure={'identity':['white_y',1],'difference':'1e-8'}
        record['native_fitting']['after']['failures']=[failure]
        self.assertEqual(native_failures(record)[-1]['failures'],[failure])
        for row in record['lifecycle']:row['native_gate']='pass'
        record['native_fitting']['after']['failures']=[]
        self.assertEqual(native_failures(record),[])

    def matrix(self,root,failures):
        class Commands:
            def __init__(self,out):
                self.out=Path(out);self.out.mkdir();self.commands=[];self.saved=None
            def run(self,args,label,**kwargs):
                self.commands.append({'argv':list(map(str,args)),'label':label,
                    'status':failures.get(label,0),'env':kwargs.get('env')})
                return b''
            def save(self,fields):self.saved=fields
        run=Commands(root/'matrix')
        args=SimpleNamespace(out=run.out,revision='candidate',native_repository=None,cmake='cmake',provider_source=None,ephemeris=None)
        with patch.object(integrated,'Run',return_value=run),contextlib.redirect_stdout(io.StringIO()):
            integrated.qualify(args)
        return run

    def test_missing_before_blocks_only_its_dependents(self):
        with tempfile.TemporaryDirectory() as directory:
            run=self.matrix(Path(directory),{'allocation-source-before':128})
            stages=run.saved['stages']
            self.assertIsNone(stages['allocation-build-before']['exit'])
            self.assertIsNone(stages['allocation-measure']['exit'])
            self.assertIsNone(stages['allocation-verify']['exit'])
            for name in ['allocation-build-after','allocation-debug-lifecycle','allocation-providers','predictors','output']:
                self.assertEqual(stages[name]['exit'],0)

    def test_failed_native_gates_keep_baseline_and_later_checks(self):
        with tempfile.TemporaryDirectory() as directory:
            run=self.matrix(Path(directory),{'allocation-verify':1,'allocation-predictor-after':1})
            stages=run.saved['stages'];commands={r['label']:r for r in run.commands}
            self.assertEqual(stages['allocation-predictor-before']['exit'],0)
            self.assertEqual(stages['allocation-debug-lifecycle']['exit'],0)
            self.assertEqual(stages['allocation-providers']['exit'],0)
            self.assertIn('allocation-verify',run.saved['failed'])
            self.assertIn('allocation-predictor-after',run.saved['failed'])
            for label,revision in integrated.ALLOCATION_REVISIONS.items():
                self.assertIn(revision,commands['allocation-build-'+label]['argv'])
            targets={commands[n]['env']['CARGO_TARGET_DIR'] for n in ['allocation-providers','allocation-debug-lifecycle']}
            self.assertEqual(len(targets),2)
            verify=commands['allocation-verify']['argv']
            self.assertIn(str(run.out/'candidate/source'),verify)
            self.assertIn(str(run.out/'native-corpora'),verify)
            self.assertEqual(commands['allocation-build-before']['env']['GMP_MPFR_SYS_CACHE'],commands['allocation-build-after']['env']['GMP_MPFR_SYS_CACHE'])


if __name__=='__main__':unittest.main()
