"""Actual executable identity is required; unrelated inspection failures continue."""
import ast
import json
from pathlib import Path
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch
from common import Run, sha
from time_ephemeris_objects import capture


class TimeEphemerisObjects(unittest.TestCase):
    def test_corpus_dispatch_captures_failed_science_before_the_next_build(self):
        # Execute the real dispatch loop with its process boundary replaced;
        # historical native staging is unrelated to this ordering contract.
        source=Path(__file__).with_name('corpus_compare.py')
        function=next(n for n in ast.parse(source.read_text()).body
                      if isinstance(n,ast.FunctionDef) and n.name=='execute')
        loop=[n for n in function.body if isinstance(n,ast.For) and isinstance(n.target,ast.Name)
              and n.target.id=='stage' and ast.unparse(n.iter)=='args.stages'][-1]
        code=compile(ast.Module(body=[loop],type_ignores=[]),str(source),'exec')
        for failed in ['time_ephemeris','time-objects','time-corrected-native']:
            with self.subTest(failed=failed),tempfile.TemporaryDirectory() as directory:
                root=Path(directory);(root/'validation/time_ephemeris/outputs/reference-corrected').mkdir(parents=True)
                calls=[]
                def call(label,command,*args,**kwargs):
                    calls.append((label,list(map(str,command))));return label!=failed
                namespace={'args':SimpleNamespace(stages=['time_ephemeris','clocks']),
                    'root':root,'rust':root/'rust','ROOT':root,'run':SimpleNamespace(out=root/'out'),
                    'profile':'release','release':['--release'],'sys':sys,'call':call,
                    'script':lambda port,name,*flags:[sys.executable,root/'validation'/port/name,*flags]}
                exec(code,namespace)
                names=[label for label,_ in calls]
                self.assertEqual(names[-1],'clocks')
                if failed=='time-corrected-native':
                    self.assertNotIn('time_ephemeris',names);self.assertNotIn('time-objects',names)
                else:
                    self.assertEqual(names.index('time-objects'),names.index('time_ephemeris')+1)
                    command=dict(calls)['time-objects']
                    self.assertIn(str(root/'out/time-ephemeris-objects'),command)
                    self.assertIn(str(root/'validation/time_ephemeris_port/outputs/release/direct.json'),command)

    def fixture(self, root):
        native = root/'native'; native.write_bytes(b'native executable')
        rust = root/'rust'; rust.write_bytes(b'executed Rust test binary')
        nr = root/'native.json'; nr.write_text(json.dumps({'profile': {
            'executable_path':str(native), 'executable_sha256':sha(native)}}))
        rr = root/'rust.json'; rr.write_text(json.dumps({'complete':True, 'cases':[{}]*97,
            'failures':[{'id':'inherited scientific failure'}], 'native_receipt_sha256':sha(nr),
            'identity':{'executable':str(rust), 'executable_sha256':sha(rust)}}))
        return nr, rr, native, rust

    def test_both_actual_binaries_captured_despite_scientific_failures(self):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);nr,rr,native,rust=self.fixture(root);run=Run(root/'out')
            with patch('time_ephemeris_objects.platform.system',return_value='Linux'), \
                 patch.object(run,'run',return_value=b'actual disassembly\n') as command:
                self.assertFalse(capture(run,nr,rr))
            self.assertEqual(command.call_args_list[0].args, (['objdump','-Cd',native],'native-disassembly'))
            self.assertEqual(command.call_args_list[1].args, (['objdump','-Cd',rust],'rust-disassembly'))
            receipt=json.loads((run.out/'receipt.json').read_text())
            self.assertEqual(set(receipt['products']),{'native','rust'})
            for name in receipt['products']:
                self.assertEqual(receipt['products'][name]['disassembly_sha256'],sha(run.out/(name+'.s')))

    def test_wrong_or_missing_executable_blocks_only_its_inspection(self):
        for engine,remove in [('native',False),('rust',False),('native',True),('rust',True)]:
            with self.subTest(engine=engine,remove=remove),tempfile.TemporaryDirectory() as directory:
                root=Path(directory);nr,rr,native,rust=self.fixture(root);run=Run(root/'out')
                exe={'native':native,'rust':rust}[engine]
                if remove:exe.unlink()
                else:exe.write_bytes(b'rebuilt after execution')
                with patch.object(run,'run',return_value=b'disassembly') as command:
                    self.assertTrue(capture(run,nr,rr))
                self.assertEqual(command.call_count,1)
                receipt=json.loads((run.out/'receipt.json').read_text())
                self.assertEqual(set(receipt['failures']),{engine})
                self.assertEqual(set(receipt['products']),{'native','rust'}-{engine})

    def test_changed_during_inspection_and_incomplete_or_wrong_receipts_reject(self):
        for fault in ['during','incomplete','count','native-receipt']:
            with self.subTest(fault=fault),tempfile.TemporaryDirectory() as directory:
                root=Path(directory);nr,rr,native,rust=self.fixture(root);run=Run(root/'out')
                record=json.loads(rr.read_text())
                if fault=='incomplete':record['complete']=False
                if fault=='count':record['cases'].pop()
                if fault=='native-receipt':record['native_receipt_sha256']='unbound'
                rr.write_text(json.dumps(record))
                def inspect(command,label):
                    if fault=='during' and label=='rust-disassembly':rust.write_bytes(b'changed')
                    return b'disassembly'
                with patch.object(run,'run',side_effect=inspect):self.assertTrue(capture(run,nr,rr))
                receipt=json.loads((run.out/'receipt.json').read_text())
                self.assertEqual(set(receipt['failures']),{'rust'})
                self.assertEqual(set(receipt['products']),{'native'})


if __name__=='__main__':unittest.main()
