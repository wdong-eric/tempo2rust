"""Keep allocation-only history, prerequisites and inherited scientific failures explicit."""
import contextlib
import gzip
import hashlib
import io
import json
from pathlib import Path
from types import SimpleNamespace
import tempfile
import shutil
import unittest
from unittest.mock import patch
from common import ROOT, Run, sha
import integrated
from allocation_verify import lifecycle_inputs, corrected_inputs, corrected_lifecycle, native_failures, verify


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

    def lifecycle_fixture(self, root):
        case='seeded_reinitialise';original=root/'original';corrected=root/'corrected'
        original.mkdir();(corrected/'expected').mkdir(parents=True)
        fields=['ne_sw_ifuncN','nWhite_dm','nchromx','AverageDMResiduals','unrelated']
        event={'phase':'repeat','state':[{'path':'psr.'+f,'runs':[[1,'00000000']]} for f in fields],
               'stdout_position':0,'stderr':''}
        expected={'events':[event],'stdout':''}
        fixed=corrected/'expected'/(case+'.json.gz')
        fixed.write_bytes(gzip.compress(json.dumps(expected).encode()))
        raw=json.loads(json.dumps(expected))
        for row in raw['events'][0]['state'][:4]:row['runs']=[[1,'01000000']]
        old=original/(case+'.json.gz');old.write_bytes(gzip.compress(json.dumps(raw).encode()))
        proof={'status':'pass','producer':sha(ROOT/'validation/release/lifecycle_correction.py'),
               'copy_transform':{'sha256':sha(ROOT/'validation/general_fitting/fitting_corrections.py')},
               'records':[{'case':case,'original_sha256':sha(old),'corrected_sha256':sha(fixed)}]}
        (corrected/'receipt.json').write_text(json.dumps(proof))
        result=root/'same-engine';wd=result/case;wd.mkdir(parents=True)
        inp=wd/'input.json';inp.write_text(json.dumps({'nonce':'challenge','args':['repeat','-4','7']}))
        launch={'phase':'launch','nonce':'challenge','input_sha256':sha(inp)}
        events=(json.dumps(launch)+'\n'+json.dumps(event)+'\n').encode()
        record={'status':'pass_same_engine','lifecycle':[],
                'native_fitting':{'before':{'failures':[]},'after':{'failures':[]}}}
        for engine in ['before','after']:
            (wd/(engine+'.jsonl.gz')).write_bytes(gzip.compress(events))
            record['lifecycle'].append({'case':case,'engine':engine,'native_gate':'original initializer mismatch',
                'reference_sha256':sha(old),'events_sha256':hashlib.sha256(events).hexdigest()})
        (result/'receipt.json').write_text(json.dumps(record))
        return original,corrected,result,record

    def test_full_corrected_contract_preserves_raw_failures_and_rejects_other_changes(self):
        with tempfile.TemporaryDirectory() as directory:
            original,corrected,result,record=self.lifecycle_fixture(Path(directory))
            references,_=corrected_inputs(corrected,original)
            checks=corrected_lifecycle(record,result,references,ROOT)
            self.assertEqual(native_failures(record,checks),[])
            self.assertTrue(all(row['native_gate']=='original initializer mismatch' for row in checks))
            failure={'identity':['white_y',1],'difference':'1e-8'}
            record['native_fitting']['after']['failures']=[failure]
            self.assertEqual(native_failures(record,checks)[0]['failures'],[failure])
            # Equal before/after output can still violate an unrelated native cell.
            for row in record['lifecycle']:
                path=result/row['case']/(row['engine']+'.jsonl.gz')
                events=[json.loads(line) for line in gzip.decompress(path.read_bytes()).splitlines()]
                events[1]['state'][-1]['runs']=[[1,'02000000']]
                data=('\n'.join(map(json.dumps,events))+'\n').encode()
                path.write_bytes(gzip.compress(data));row['events_sha256']=hashlib.sha256(data).hexdigest()
            failures=native_failures(record,corrected_lifecycle(record,result,references,ROOT))
            self.assertEqual([row['scope'] for row in failures],['lifecycle','lifecycle','fitting'])

    def test_missing_failed_or_unbound_proof_is_not_a_waiver(self):
        with tempfile.TemporaryDirectory() as directory:
            original,corrected,result,record=self.lifecycle_fixture(Path(directory))
            receipt=corrected/'receipt.json';proof=json.loads(receipt.read_text())
            for field,value,message in [('status','failure','did not pass'),('producer','wrong','producer')]:
                receipt.write_text(json.dumps({**proof,field:value}))
                with self.assertRaisesRegex(ValueError,message):corrected_inputs(corrected,original)
            receipt.write_text(json.dumps(proof))
            path=corrected/'expected/seeded_reinitialise.json.gz';path.write_bytes(b'changed reference')
            with self.assertRaisesRegex(ValueError,'input differs'):corrected_inputs(corrected,original)
            receipt.unlink()
            with self.assertRaises(FileNotFoundError):corrected_inputs(corrected,original)

    def test_missing_proof_keeps_an_receipt_and_fitting_failure(self):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);original,corrected,result,record=self.lifecycle_fixture(root)
            record['native_fitting']['before']['failures']=['inherited fitting failure']
            (result/'receipt.json').write_text(json.dumps(record));(corrected/'receipt.json').unlink()
            expected=root/'native-corpora/staging/validation/pulsar_lifecycle/expected'
            shutil.copytree(original,expected)
            (root/'native-corpora/receipt.json').write_text(json.dumps({'stages':{
                'pulsar_lifecycle':{'record_exit':0,'directory':str(expected.parent)}}}))
            args=SimpleNamespace(out=root/'verify',before=root/'before',after=root/'after',a2=root/'a2',
                candidate=ROOT,native_corpora=root/'native-corpora',native_fit=root/'fit',lifecycle_corrected=corrected)
            def same_engine(run,*args,**kwargs):
                shutil.copytree(result,run.out/'same-engine');run.commands.append({'status':0})
            with patch.object(Run,'run',same_engine):self.assertEqual(verify(args),1)
            receipt=json.loads((args.out/'receipt.json').read_text())
            self.assertEqual(receipt['status'],'blocked')
            self.assertEqual(receipt['same_engine'],'pass')
            self.assertEqual(len(receipt['original_lifecycle_failures']),2)
            self.assertEqual(receipt['native_failures'][0]['scope'],'fitting')
            self.assertEqual(sha(result/'receipt.json'),sha(args.out/'same-engine/receipt.json'))

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
            self.assertIn(str(run.out/'corpus/lifecycle-corrected-native'),verify)
            self.assertEqual(commands['allocation-build-before']['env']['GMP_MPFR_SYS_CACHE'],commands['allocation-build-after']['env']['GMP_MPFR_SYS_CACHE'])


if __name__=='__main__':unittest.main()
