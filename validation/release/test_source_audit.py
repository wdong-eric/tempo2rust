import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

from common import Run
import source_audit


class ShallowBaseline(unittest.TestCase):
    def test_unavailable_baseline_is_not_a_source_change(self):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);upstream=root/'upstream';checkout=root/'checkout'
            def git(path,*args):
                return subprocess.check_output(['git','-C',str(path),*args],stderr=subprocess.PIPE).decode().strip()
            upstream.mkdir();git(upstream,'init')
            git(upstream,'config','user.name','Audit regression')
            git(upstream,'config','user.email','audit@example.invalid')
            src=upstream/'rust/src';src.mkdir(parents=True)
            for name in ['same','changed','removed']:(src/(name+'.rs')).write_text('pub fn original() {}\n')
            git(upstream,'add','.');git(upstream,'commit','-m','baseline')
            baseline=git(upstream,'rev-parse','HEAD')
            (src/'changed.rs').write_text('pub fn changed() {}\n')
            (src/'removed.rs').unlink();(src/'added.rs').write_text('pub fn added() {}\n')
            git(upstream,'add','.');git(upstream,'commit','-m','candidate')
            candidate=git(upstream,'rev-parse','HEAD')
            checkout.mkdir();git(checkout,'init');git(checkout,'remote','add','origin',str(upstream))
            git(checkout,'fetch','--depth=1','origin',candidate);git(checkout,'checkout','--detach','FETCH_HEAD')
            with patch.object(source_audit,'ROOT',checkout),patch.object(source_audit,'BASELINE',baseline):
                missing=Run(root/'missing')
                with self.assertRaises(SystemExit):source_audit.compare_bodies(missing)
                receipt=json.loads((missing.out/'receipt.json').read_text())
                self.assertEqual(receipt['status'],'failure')
                self.assertEqual(receipt['baseline_status'],'unavailable')
                self.assertNotIn('changed_rust_bodies',receipt)
                # Match the workflow's narrow fetch, retaining candidate checkout.
                git(checkout,'fetch','--no-tags','--depth=1','origin',baseline)
                self.assertEqual(git(checkout,'rev-parse','HEAD'),candidate)
                self.assertEqual(git(checkout,'rev-parse','--is-shallow-repository'),'true')
                result=source_audit.compare_bodies(Run(root/'available'))
                self.assertEqual(list(result['unchanged_rust_bodies']),['rust/src/same.rs'])
                self.assertEqual(result['changed_rust_bodies'],['rust/src/added.rs','rust/src/changed.rs','rust/src/removed.rs'])
                self.assertEqual(result['added_rust_bodies'],['rust/src/added.rs'])
                self.assertEqual(result['removed_rust_bodies'],['rust/src/removed.rs'])


if __name__=='__main__':unittest.main()
