"""Shallow history construction and failed-preparation publication contracts."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from common import Run
from native_corpora import stage_history
from corpus_compare import execute, STAGES


class HistoryTests(unittest.TestCase):
    def test_shallow_origin_resolution_and_failed_attempt_are_retained(self):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);origin=root/'origin'
            def git(*args):
                return subprocess.check_output(['git',*map(str,args)],stderr=subprocess.DEVNULL).decode().strip()
            git('init',origin)
            pins=[]
            for i in range(3):
                (origin/'value').write_text(str(i))
                git('-C',origin,'add','value')
                git('-C',origin,'-c','user.name=AL test','-c','user.email=al@example.invalid','commit','-m',str(i))
                pins.append(git('-C',origin,'rev-parse','HEAD'))
            checkout=root/'checkout'
            git('clone','--depth=1',origin.as_uri(),checkout)
            failed=Run(root/'failed')
            with self.assertRaises(RuntimeError):
                stage_history(failed,failed.out/'staging',[pins[0],'0'*40],checkout)
            failure=(failed.out/'receipt.json').read_bytes()
            self.assertEqual(json.loads(failure)['failed_command'],'history-fetch-'+'0'*40)
            recovery=Run(root/'recovery')
            stage_history(recovery,recovery.out/'staging',pins,checkout)
            self.assertEqual((failed.out/'receipt.json').read_bytes(),failure)
            for i,pin in enumerate(pins):
                self.assertEqual(git('-C',recovery.out/'staging','show',pin+':value'),str(i))
            self.assertEqual(set((recovery.out/'staging/.git/shallow').read_text().splitlines()),set(pins))
            self.assertEqual(int(git('-C',recovery.out/'staging','rev-list','--all','--count')),3)

    def test_preparation_failure_publishes_every_unavailable_comparison(self):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);native=root/'native';native.mkdir()
            receipt=native/'receipt.json'
            receipt.write_text(json.dumps({'status':'failure','commands':[],
                'failed_command':'history-fetch-7946aeee'}))
            before=receipt.read_bytes()
            out=root/'comparison'
            status=execute(argparse.Namespace(out=out,reference_root=native/'staging',
                candidate=root/'candidate',stages=list(STAGES)))
            result=json.loads((out/'receipt.json').read_text())
            self.assertEqual(status,1)
            self.assertEqual(set(result['blocked']),set(STAGES))
            self.assertEqual(result['stages'],{})
            self.assertEqual(result['native_preparation']['failed_command'],'history-fetch-7946aeee')
            self.assertEqual(receipt.read_bytes(),before)


if __name__=='__main__':unittest.main()
