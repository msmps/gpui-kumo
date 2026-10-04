"""Regression checks for the unpatched CI failure policy."""

from pathlib import Path
import runpy
import unittest

classify = runpy.run_path(str(Path(__file__).with_name("check-unpatched.py")))["classify"]


class UnpatchedGateTests(unittest.TestCase):
    known = {"known::test": "dependency defect"}
    baseline = "test known::test ... FAILED\ntest normal::test ... ok\ntest result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out;\n"

    def test_known_failure_is_reported_without_blocking(self):
        result = classify(self.baseline, 101, self.known)
        self.assertEqual((result["blocked"], result["known_failures"]), (False, ["known::test"]))

    def test_new_failure_blocks(self):
        self.assertTrue(classify(self.baseline.replace("normal::test ... ok", "normal::test ... FAILED"), 101, self.known)["blocked"])

    def test_removed_baseline_test_blocks(self):
        self.assertTrue(classify(self.baseline.replace("known::test", "renamed::test"), 101, self.known)["blocked"])

    def test_build_failure_blocks(self):
        self.assertTrue(classify("error: could not compile", 101, self.known)["blocked"])

    def test_ignored_test_blocks(self):
        self.assertTrue(classify(self.baseline.replace("normal::test ... ok", "normal::test ... ignored"), 101, self.known)["blocked"])

    def test_abnormal_process_exit_blocks(self):
        self.assertTrue(classify(self.baseline, -9, self.known)["blocked"])

    def test_unexpected_should_panic_failure_blocks(self):
        output = self.baseline.replace("normal::test ... ok", "normal::test - should panic ... FAILED")
        self.assertTrue(classify(output, 101, self.known)["blocked"])

    def test_incomplete_result_capture_blocks(self):
        output = self.baseline.replace("test normal::test ... ok\n", "")
        self.assertTrue(classify(output, 101, self.known)["blocked"])

    def test_known_pass_is_reported_for_review(self):
        output = self.baseline.replace("FAILED", "ok").replace("1 passed; 1 failed", "2 passed; 0 failed")
        result = classify(output, 0, self.known)
        self.assertEqual((result["blocked"], result["known_passes"]), (False, ["known::test"]))


if __name__ == "__main__":
    unittest.main()
