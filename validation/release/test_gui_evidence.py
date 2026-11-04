"""Reject absent executions/products and any byte difference in native timing."""
import json
from pathlib import Path
import struct
import tempfile
import unittest
from common import Run, sha
from gui_evidence import verify


class GuiEvidence(unittest.TestCase):
    def fixture(self,root):
        gui=root/'gui';candidate=root/'candidate/source'
        def write(path,data):
            path.parent.mkdir(parents=True,exist_ok=True)
            path.write_bytes(data.encode() if isinstance(data,str) else data)
        summary='\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;\n'
        for label,name in [('gui-library-tests','library'),('gui-control-render','tests::load_inspect_and_render'),
                           ('gui-native','native_test::native_x11_load_render_close')]:
            write(root/(label+'.stdout'),'test '+name+' ... ok\n'+summary)
        for path in [candidate.parent/'receipt.json',candidate/'gui/Cargo.toml',candidate/'gui/Cargo.lock',
                     candidate/'rust/Cargo.lock',gui/'prefix/bin/tempo2-gui',
                     gui/'target/debug/examples/export_timing',gui/'target/debug/examples/session_workflow']:
            write(path,'fixture')
        artifacts=[]
        for name in ['tempo2_gui','tempo2-gui']:
            exe=gui/'target/debug'/name;write(exe,name)
            artifacts.append({'executable':str(exe),'profile':{'test':True},'target':{'name':name}})
        write(root/'gui-tests-build.stdout','\n'.join(map(json.dumps,artifacts)))
        write(gui/'parity/bary-unsorted-cli.stdout','{"signed": "-0", "value": 1}\n')
        write(gui/'native/native-timing.json',(gui/'parity/bary-unsorted-cli.stdout').read_bytes())
        write(gui/'native/opengl.txt','4.5\nMesa\nllvmpipe\n')
        write(gui/'parity/receipt.json',json.dumps({'status':'pass','core_dependency_versions_preserved':True,
            'outputs':{'bary-unsorted-cli.stdout':sha(gui/'parity/bary-unsorted-cli.stdout')}}))
        for case in ['bary-unsorted','pks']:
            for mode in ['gui','engine']:
                for product in ['session.par','session.tim','session.json']:
                    write(gui/'parity'/f'{case}-{mode}'/product,'saved product')
        # This gate checks headers/dimensions only, not visual rendering or PNG decoding.
        header=b'\x89PNG\r\n\x1a\n'+struct.pack('>I',13)+b'IHDR'+struct.pack('>II',1100,800)
        for path in ['render/'+name+'.png' for name in
                     ['viewer','excluded','fitted','saved','save-error','input-error']]+['native/native.png']:
            write(gui/path,header)
        library=root/'libexample.so';write(library,'actual library bytes')
        write(root/'gui-libraries.stdout',f'libexample.so => {library} (0x1234)\n')
        return gui,candidate

    def test_complete_products_bind_executables_libraries_and_raw_timing(self):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);gui,candidate=self.fixture(root);run=Run(root/'out')
            verify(run,gui,candidate)
            receipt=json.loads((run.out/'receipt.json').read_text())
            self.assertTrue(receipt['native_timing_exact_cli']);self.assertEqual(len(receipt['images']),7)
            self.assertEqual(receipt['linked_libraries'][0]['sha256'],sha(root/'libexample.so'))
            self.assertEqual(receipt['inputs'][str(gui/'prefix/bin/tempo2-gui')],sha(gui/'prefix/bin/tempo2-gui'))

    def test_empty_ignored_failed_or_wrong_named_tests_are_not_execution(self):
        for label in ['gui-library-tests','gui-control-render','gui-native']:
            for fault in ['empty','failed','ignored','wrong-name']:
                if label=='gui-library-tests' and fault=='wrong-name':continue
                with self.subTest(label=label,fault=fault),tempfile.TemporaryDirectory() as directory:
                    root=Path(directory);gui,candidate=self.fixture(root);path=root/(label+'.stdout')
                    text=path.read_text()
                    if fault=='empty':text='running 0 tests\n'
                    if fault=='failed':text=text.replace('1 passed; 0 failed','0 passed; 1 failed').replace('result: ok','result: FAILED')
                    if fault=='ignored':text=text.replace(' ... ok',' ... ignored')
                    if fault=='wrong-name':text=text.replace('test ','test other::',1)
                    if label=='gui-library-tests' and fault=='ignored':text=text.replace('1 passed','0 passed')
                    path.write_text(text)
                    with self.assertRaises(AssertionError):verify(Run(root/'out'),gui,candidate)

    def test_mismatched_timing_unbound_cli_and_missing_products_reject(self):
        for fault in ['sign','native-bytes','cli-hash','image','context','saved-file','artifact','library']:
            with self.subTest(fault=fault),tempfile.TemporaryDirectory() as directory:
                root=Path(directory);gui,candidate=self.fixture(root)
                if fault=='sign':
                    path=gui/'native/native-timing.json';path.write_text(path.read_text().replace('-0','0'))
                if fault=='native-bytes':(gui/'native/native-timing.json').write_text('{}')
                if fault=='cli-hash':
                    for path in [gui/'native/native-timing.json',gui/'parity/bary-unsorted-cli.stdout']:path.write_text('{}')
                if fault=='image':(gui/'native/native.png').unlink()
                if fault=='context':(gui/'native/opengl.txt').write_text('4.5\n\nllvmpipe\n')
                if fault=='saved-file':(gui/'parity/pks-engine/session.tim').unlink()
                if fault=='artifact':(root/'gui-tests-build.stdout').write_text('')
                if fault=='library':(root/'libexample.so').unlink()
                with self.assertRaises((AssertionError,FileNotFoundError)):verify(Run(root/'out'),gui,candidate)


if __name__=='__main__':unittest.main()
