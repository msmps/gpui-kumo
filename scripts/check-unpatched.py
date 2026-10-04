#!/usr/bin/env python3
"""Test the packaged RC against registry dependencies, allowing named known defects."""

import argparse
import json
import os
from pathlib import Path
import re
import subprocess
import tarfile
import tempfile

ROOT = Path(__file__).resolve().parent.parent
BASELINE = ROOT / "scripts/unpatched-known-failures.json"


def classify(output, returncode, known):
    results = dict(re.findall(r"^test (\S+)(?: - should panic)? \.\.\. (ok|FAILED|ignored)$", output, re.MULTILINE))
    missing = sorted(set(known) - results.keys())
    unexpected = sorted(name for name, status in results.items() if status != "ok" and name not in known)
    failures = sorted(name for name, status in results.items() if status == "FAILED")
    totals = re.search(r"^test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; 0 ignored; 0 measured; 0 filtered out;", output, re.MULTILINE)
    complete = bool(totals) and tuple(map(int, totals.groups())) == (
        sum(status == "ok" for status in results.values()), len(failures)
    )
    valid_exit = returncode == (101 if failures else 0)
    return {
        "known_failures": sorted(set(failures) & known.keys()),
        "known_passes": sorted(name for name in known if results.get(name) == "ok"),
        "missing_known_tests": missing,
        "unexpected_failures": unexpected,
        "passed": sum(status == "ok" for status in results.values()),
        "blocked": bool(missing or unexpected or not complete or not valid_exit),
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, default=ROOT / "target/unpatched-ci")
    args = parser.parse_args()
    args.output_dir.mkdir(parents=True, exist_ok=True)
    known = json.loads(BASELINE.read_text())
    subprocess.run(["cargo", "package", "-p", "gpui-kumo", "--no-verify", "--locked", "--allow-dirty"], cwd=ROOT, check=True)
    metadata = json.loads(subprocess.check_output(["cargo", "metadata", "--no-deps", "--format-version", "1"], cwd=ROOT))
    package = next(p for p in metadata["packages"] if p["name"] == "gpui-kumo")
    archive = Path(metadata["target_directory"]) / "package" / f"gpui-kumo-{package['version']}.crate"
    with tempfile.TemporaryDirectory(prefix="kumo-unpatched-ci-") as temporary:
        with tarfile.open(archive) as source:
            source.extractall(temporary, filter="data")
        extracted = Path(temporary) / f"gpui-kumo-{package['version']}"
        assert "[patch." not in (extracted / "Cargo.toml").read_text(), "Packaged manifest has patches"
        graph = json.loads(subprocess.check_output(["cargo", "metadata", "--locked", "--format-version", "1"], cwd=extracted))
        for name in ("gpui-kit", "gpui-pre", "gpui-base", "accesskit_atspi_common"):
            packages = [p for p in graph["packages"] if p["name"] == name]
            assert len(packages) == 1 and packages[0]["source"].startswith("registry+"), f"Unexpected source for {name}"
        (args.output_dir / "dependency-graph.json").write_text(json.dumps(graph, indent=2))
        result = subprocess.run(["cargo", "test", "--lib", "--all-features", "--locked", "--", "--color", "never"], cwd=extracted, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
        print(result.stdout, end="")
        (args.output_dir / "tests.log").write_text(result.stdout)
        report = classify(result.stdout, result.returncode, known)
        (args.output_dir / "results.json").write_text(json.dumps(report, indent=2) + "\n")
    lines = ["## Unpatched packaged library", "", f"Passing tests: {report['passed']}", "", "| Known dependency regression | Result | Cause |", "| --- | --- | --- |"]
    for name, cause in known.items():
        status = "KNOWN FAILURE" if name in report["known_failures"] else "PASS (review baseline)" if name in report["known_passes"] else "MISSING (blocks)"
        lines.append(f"| `{name}` | {status} | {cause} |")
    lines += ["", f"Unexpected failures: {report['unexpected_failures']}", f"Missing known tests: {report['missing_known_tests']}", f"Gate: {'BLOCKED' if report['blocked'] else 'PASS'}", "", "Known passes are reported for baseline review; clock-dependent defects can vary. Compilation/harness failures, ignored tests, removed baseline tests and unexpected test failures block."]
    summary = "\n".join(lines) + "\n"
    print(summary)
    (args.output_dir / "summary.md").write_text(summary)
    if os.environ.get("GITHUB_STEP_SUMMARY"):
        with open(os.environ["GITHUB_STEP_SUMMARY"], "a") as destination:
            destination.write(summary)
    return int(report["blocked"])


if __name__ == "__main__":
    raise SystemExit(main())
