"""Regression evidence for H's diagnostic comparator; no TEMPO2 counterpart."""
import copy
import json
import unittest

from run import HERE, compare, isolated_contract


class IsolatedDiagnosticTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.fixtures=json.loads((HERE/"diagnostic_fixtures.json").read_text())["observations"]
        cls.expected=json.loads((HERE/"outputs/isolated.json").read_text())

    def test_recorded_symbolized_and_unsymbolized_presentations_are_equivalent(self):
        for label,observed in self.fixtures.items():
            with self.subTest(presentation=label):
                compare(isolated_contract(observed),self.expected)

    def test_wrong_failure_kind_is_rejected(self):
        for label,observed in self.fixtures.items():
            with self.subTest(presentation=label):
                changed=copy.deepcopy(observed)
                case=changed["cases"]["coefficient_offset"]
                case["summary"]=[s.replace("AddressSanitizer: BUS","AddressSanitizer: heap-buffer-overflow") for s in case["summary"]]
                with self.assertRaises(AssertionError):
                    compare(isolated_contract(changed),self.expected)

    def test_wrong_completed_stage_is_rejected(self):
        for label,observed in self.fixtures.items():
            with self.subTest(presentation=label):
                changed=copy.deepcopy(observed)
                changed["cases"]["coefficient_offset"]["last_completed_op"]="open"
                with self.assertRaises(AssertionError):
                    compare(isolated_contract(changed),self.expected)

    def test_wrong_symbolized_source_location_is_rejected(self):
        for replacement in ("jpleph.c:403","other.c:373"):
            with self.subTest(location=replacement):
                changed=copy.deepcopy(self.fixtures["symbolized"])
                case=changed["cases"]["coefficient_offset"]
                case["summary"]=[s.replace("jpleph.c:373",replacement) for s in case["summary"]]
                with self.assertRaises(AssertionError):
                    isolated_contract(changed)

    def test_missing_ubsan_source_location_is_rejected(self):
        changed=copy.deepcopy(self.fixtures["unsymbolized"])
        changed["cases"]["u2_packed_au"]["summary"]=["runtime error: load of misaligned address <address>"]
        with self.assertRaises(AssertionError):
            isolated_contract(changed)

    def test_unclassified_diagnostic_is_rejected(self):
        changed=copy.deepcopy(self.fixtures["unsymbolized"])
        changed["cases"]["coefficient_offset"]["summary"]=["process stopped without a sanitizer classification"]
        with self.assertRaises(AssertionError):
            isolated_contract(changed)

    def test_changed_return_status_is_rejected(self):
        for code in (0,-11):
            with self.subTest(returncode=code):
                changed=copy.deepcopy(self.fixtures["unsymbolized"])
                changed["cases"]["coefficient_offset"]["returncode"]=code
                with self.assertRaises(AssertionError):
                    compare(isolated_contract(changed),self.expected)

    def test_dropped_failure_case_is_rejected(self):
        changed=copy.deepcopy(self.fixtures["unsymbolized"])
        del changed["cases"]["coefficient_offset"]
        with self.assertRaises(AssertionError):
            compare(isolated_contract(changed),self.expected)

    def test_canonical_assertion_condition_is_gated(self):
        changed=copy.deepcopy(self.fixtures["unsymbolized"])
        case=changed["cases"]["max_cheby_assertion"]
        case["summary"]=[s.replace("ncf < MAX_CHEBY","ncf <= MAX_CHEBY") for s in case["summary"]]
        with self.assertRaises(AssertionError):
            compare(isolated_contract(changed),self.expected)


if __name__=="__main__": unittest.main()
