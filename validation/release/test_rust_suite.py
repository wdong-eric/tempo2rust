"""Keep partial libtest execution distinct from build failure and skipped tests."""
import unittest
from rust_suite import test_counts


class RustSuiteTests(unittest.TestCase):
    def test_partial_failure_reports_executed_tests(self):
        counts = test_counts('test result: FAILED. 94 passed; 2 failed; 36 ignored; 0 measured; 0 filtered out; finished in 53.93s')
        self.assertTrue(counts['executed'])
        self.assertEqual((counts['passing_tests'], counts['failing_tests'], counts['ignored_tests']), (94, 2, 36))
        self.assertEqual(counts['completed_targets'], 1)

    def test_build_failure_and_ignored_tests_are_not_execution(self):
        for stdout in ['', 'test result: ok. 0 passed; 0 failed; 36 ignored; 0 measured; 96 filtered out; finished in 0.00s']:
            self.assertFalse(test_counts(stdout)['executed'])


if __name__ == '__main__':
    unittest.main()
