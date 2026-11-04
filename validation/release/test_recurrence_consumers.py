"""Installed borrow witness runs on both endpoints and compares both streams."""
import contextlib
import io
import json
from pathlib import Path
import runpy
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch
from common import ROOT,sha


class RecurrenceConsumers(unittest.TestCase):
    def execute(self, root, corrupt=None):
        prefixes=[]
        for side in ['before','after']:
            prefix=root/side;installed=prefix/'share/tempo2-rust/rust';installed.mkdir(parents=True)
            (installed/'Cargo.toml').write_text('[dependencies]\nrug="1"\n[lib]\n')
            prefixes.append(prefix)
        out=root/'consumers';script=ROOT/'validation/predictor_performance/consumers.py'
        command=[str(script),'--before',str(prefixes[0]),'--after',str(prefixes[1]),
                 '--out',str(out),'--cargo-home',str(root/'cache')]
        def process(args,**kwargs):
            stdout=b'';stderr=b''
            if args[0]=='cargo':
                work=Path(args[args.index('--manifest-path')+1]).parent
                for source in (work/'src/bin').glob('*.rs'):
                    exe=work/'target/release'/source.stem;exe.parent.mkdir(parents=True,exist_ok=True)
                    exe.write_bytes(b'controlled executable')
            else:
                exe=Path(args[0]);stdout=exe.name.encode()+b' identical\n'
                if 'after' in exe.parts and exe.name=='recurrence_consumer':
                    if corrupt=='stdout':stdout+=b'changed borrow output\n'
                    if corrupt=='stderr':stderr=b'unexpected borrow diagnostic\n'
            return subprocess.CompletedProcess(args,0,stdout,stderr)
        with patch.object(sys,'argv',command),patch.object(subprocess,'run',side_effect=process),contextlib.redirect_stdout(io.StringIO()):
            runpy.run_path(str(script),run_name='__main__')
        return out

    def test_borrow_source_and_both_runs_are_bound(self):
        with tempfile.TemporaryDirectory() as directory:
            out=self.execute(Path(directory));receipt=json.loads((out/'receipt.json').read_text())
            self.assertEqual(len(receipt['records']),6)
            source=ROOT/'validation/predictor_performance/recurrence_consumer.rs'
            self.assertEqual(receipt['inputs'][str(source)],sha(source))
            for side in ['before','after']:
                self.assertEqual((out/side/'src/bin/recurrence_consumer.rs').read_bytes(),source.read_bytes())
                record=next(r for r in receipt['records'] if r['engine']==side and r['consumer']=='recurrence_consumer')
                self.assertEqual(record['stdout_sha256'],sha(out/side/'recurrence_consumer.stdout'))
                self.assertEqual(record['stderr_sha256'],sha(out/side/'recurrence_consumer.stderr'))

    def test_changed_borrow_output_or_diagnostic_keeps_failure_artifact(self):
        for channel in ['stdout','stderr']:
            with self.subTest(channel=channel),tempfile.TemporaryDirectory() as directory:
                root=Path(directory)
                with self.assertRaises(AssertionError):self.execute(root,channel)
                self.assertFalse((root/'consumers/receipt.json').exists())
                self.assertTrue((root/'consumers/after'/('recurrence_consumer.'+channel)).read_bytes())


if __name__=='__main__':unittest.main()
