"""GUI checks use one fresh export and do not suppress scientific stages."""
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
import integrated
import test_allocation_wiring as allocation_controls


class GuiWiring(unittest.TestCase):
    def matrix(self,root,failures):
        with patch.object(integrated.platform,'system',return_value='Linux'):
            return allocation_controls.AllocationWiring().matrix(root,failures)

    def test_locked_install_explicit_runtime_and_graphics_precede_measurement(self):
        with tempfile.TemporaryDirectory() as directory:
            run=self.matrix(Path(directory),{});commands={r['label']:r for r in run.commands}
            gui=run.out/'gui';candidate=run.out/'candidate/source';names=list(commands)
            fetch=commands['gui-dependencies']['argv'];self.assertIn('--locked',fetch)
            self.assertNotIn('--offline',fetch);self.assertIn(str(candidate/'gui/Cargo.toml'),fetch)
            install=commands['gui-install']['argv']
            self.assertEqual(install[0:4],['cargo','install','--offline','--locked'])
            self.assertNotIn('--debug',install);self.assertIn(str(gui/'prefix'),install)
            for name in ['gui-install','gui-tests-build','gui-library-tests','gui-examples-build','gui-control-render','gui-native']:
                env=commands[name]['env']
                self.assertEqual(env['TEMPO2'],str(run.out/'a2/runtime'))
                self.assertEqual(env['CARGO_TARGET_DIR'],str(gui/'target'))
                self.assertEqual(env['XDG_RUNTIME_DIR'],str(gui/'xdg-runtime'))
            self.assertEqual((gui/'xdg-runtime').stat().st_mode&0o777,0o700)
            self.assertEqual(commands['gui-libraries']['argv'],['ldd',str(gui/'prefix/bin/tempo2-gui')])
            native=commands['gui-native'];self.assertEqual(native['argv'][:3],['timeout','60s','xvfb-run'])
            self.assertIn('native_test::native_x11_load_render_close',native['argv'])
            self.assertIn('--exact',native['argv']);self.assertEqual(native['env']['LIBGL_ALWAYS_SOFTWARE'],'1')
            self.assertEqual(native['env']['TEMPO2_GUI_TEST_OUTPUT'],str(gui/'native'))
            self.assertEqual(commands['gui-control-render']['env']['TEMPO2_GUI_TEST_OUTPUT'],str(gui/'render'))
            for expected in ['--include-ignored','--test-threads=1']:
                self.assertIn(expected,commands['gui-library-tests']['argv'])
            parity=commands['gui-parity']['argv']
            for path in [candidate/'gui/verify.py',run.out/'install-default/bin/tempo2-rust',
                         gui/'target/debug/examples/export_timing',gui/'target/debug/examples/session_workflow',run.out/'a2/runtime']:
                self.assertIn(str(path),parity)
            self.assertTrue(run.saved['gui']['enabled'])
            for name in ['predictor-pair-measure','evaluator-direct-measure','evaluator-pair-measure',
                         'recurrence-direct-measure','recurrence-pair-measure','coefficient-direct-measure','coefficient-pair-measure',
                         'temporary-direct-measure','temporary-pair-measure']:
                self.assertGreater(names.index(name),names.index('gui-evidence'))

    def test_gui_failure_blocks_only_dependent_checks(self):
        for failure,blocked,independent in [
            ('gui-dependencies','gui-install','coefficient-pair-measure'),
            ('gui-tests-build','gui-native','gui-parity'),
            ('gui-install','gui-libraries','gui-native'),
            ('gui-control-render','gui-evidence','gui-native'),
            ('gui-native','gui-evidence','gui-parity'),
            ('gui-library-tests','gui-evidence','gui-control-render'),
            ('gui-parity','gui-evidence','gui-native'),
            ('install-default','gui-parity','gui-native')]:
            with self.subTest(failure=failure),tempfile.TemporaryDirectory() as directory:
                run=self.matrix(Path(directory),{failure:1});stages=run.saved['stages']
                self.assertIsNone(stages[blocked]['exit']);self.assertEqual(stages[independent]['exit'],0)
                self.assertEqual(stages['coefficient-pair-measure']['exit'],0)
                self.assertEqual(stages['source-audit']['exit'],0)
                self.assertIn(failure,run.saved['failed'])

    def test_no_linux_native_claim_on_macos(self):
        with tempfile.TemporaryDirectory() as directory,patch.object(integrated.platform,'system',return_value='Darwin'):
            run=allocation_controls.AllocationWiring().matrix(Path(directory),{})
            self.assertFalse(run.saved['gui']['enabled'])
            self.assertNotIn('gui-native',run.saved['stages'])


if __name__=='__main__':unittest.main()
