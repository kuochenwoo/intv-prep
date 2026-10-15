"""Shared writer for content/leetcode/problems/<id>.json.

Each tools/leetcode/problems/lc-<n>.py builds a problem dict, a list of test
inputs and an independent Python reference `solve`, then calls `write(...)`.
Expected values are computed by the reference and later re-verified by real
C++ solutions in both modes (see README.md in this folder). Never type an
expected value by hand unless it is a published example; even then the
reference must agree, which `write` asserts.
"""
import json
import pathlib
import random

ROOT = pathlib.Path(__file__).resolve().parents[2]
PROBLEMS = ROOT / "content" / "leetcode" / "problems"

# macOS clang/libc++ has no <bits/stdc++.h>; templates and solutions use real headers.
ACM_TEMPLATE = (
    "#include <iostream>\n#include <vector>\n#include <string>\n#include <algorithm>\n"
    "#include <unordered_map>\nusing namespace std;\n\n"
    "int main() {\n"
    "    // Read stdin and print the answer according to the problem protocol.\n"
    "    return 0;\n}\n"
)
LIMITS = {"timeLimitMs": 1000, "memoryLimitMb": 128, "outputLimitKb": 64}


def rng(seed):
    """Deterministic random source so regenerated files are identical."""
    return random.Random(seed)


def _compact(value):
    return json.dumps(value, ensure_ascii=False, separators=(", ", ": "))


def write(problem, tests, solve, examples=None, check=None):
    """problem: every field except tests/examples (see lc-42.py).
    tests: list of dicts {id, input, sample, expected?}. `expected` may be given
    for samples copied from the published examples; it must equal solve(input)
    unless `check(input, expected, solved)` accepts it (multiple valid answers).
    examples: optional override; default = sample tests rendered as JSON text.
    """
    assert problem["id"].startswith("lc-")
    ids = [t["id"] for t in tests]
    assert len(ids) == len(set(ids)), "duplicate test ids"
    assert any(t["sample"] for t in tests), "need a sample"
    assert any(not t["sample"] and t["id"].startswith(("max-size-", "worst-case-")) for t in tests), \
        "need a max-size-* or worst-case-* hidden test"
    rows = []
    for t in tests:
        solved = solve(t["input"])
        expected = t.get("expected", solved)
        if expected != solved and not (check and check(t["input"], expected, solved)):
            raise AssertionError(f"{t['id']}: given expected {expected!r} != reference {solved!r}")
        rows.append({"id": t["id"], "logicalInput": t["input"], "expected": expected, "isSample": t["sample"]})
    if examples is None:
        examples = [{"input": _compact(r["logicalInput"]), "output": _compact(r["expected"])}
                    for r in rows if r["isSample"]]
    doc = dict(problem)
    doc.setdefault("limits", LIMITS)
    doc["cpp"].setdefault("acmTemplate", ACM_TEMPLATE)
    doc["examples"] = examples
    # Human-readable metadata, one compact line per test (max-size tests are long).
    head = json.dumps({k: v for k, v in doc.items()}, ensure_ascii=False, indent=2)
    body = ",\n".join("    " + _compact(r) for r in rows)
    text = head[:-2] + ',\n  "tests": [\n' + body + "\n  ]\n}\n"
    json.loads(text)
    out = PROBLEMS / f"{problem['id']}.json"
    out.write_text(text, encoding="utf-8")
    print(f"wrote {out.relative_to(ROOT)}: {len(rows)} tests, {len(text) // 1024} KiB")
