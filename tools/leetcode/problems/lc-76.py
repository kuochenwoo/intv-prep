import sys, pathlib
from collections import Counter
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-76",
    "title": "最小覆盖子串",
    "difficulty": "hard",
    "tags": ["哈希表", "字符串", "滑动窗口"],
    "statement": "给定字符串 s 和 t，返回 s 中最短的连续子串，使 t 中的每个字符（包括重复字符，按出现次数计）都包含在该子串中。如果不存在这样的子串，返回空字符串 \"\"。测试数据保证答案唯一。",
    "en": {
        "title": "Minimum Window Substring",
        "tags": [
            "Hash Table",
            "String",
            "Sliding Window"
        ],
        "statement": "Given strings s and t, return the shortest contiguous substring of s that contains every character of t (including duplicates, counted by multiplicity). If no such substring exists, return the empty string \"\". The test data guarantees the answer is unique.",
        "constraints": [
            "1 ≤ s.length, t.length ≤ 100000",
            "s and t consist of uppercase and lowercase English letters"
        ],
        "complexity": {
            "time": "O(m + n) (official follow-up)",
            "space": "O(|Σ|) extra space (character counts, |Σ| = 52)",
            "officialRequirement": "Official follow-up asks for an algorithm that runs in O(m + n) time.",
            "notes": "m = s.length, n = t.length. Sliding window: expand the right end until t is covered, then shrink the left end as far as possible, tracking how many characters are still missing; each index enters and leaves the window at most once. Comparing all 52 counts on every coverage check is O(52·m), which also passes but is not optimal."
        },
        "acmIoSpec": "stdin: first line string s; second line string t. stdout: one line, the minimum window substring; print an empty line if none exists."
    },
    "constraints": [
        "1 ≤ s.length, t.length ≤ 100000",
        "s 和 t 由大小写英文字母组成",
    ],
    "complexity": {
        "time": "O(m + n)（官方 follow-up）",
        "space": "O(|Σ|) 额外空间（字符计数，|Σ| = 52）",
        "officialRequirement": "官方 follow-up 要求设计 O(m + n) 时间的算法。",
        "notes": "m = s.length，n = t.length。滑动窗口：右端扩张直到覆盖 t，再尽量收缩左端，并记录仍缺的字符数量；每个下标最多进出窗口各一次。每次检查是否覆盖都比较 52 个计数的做法为 O(52·m)，也可通过但不是最优。",
        "sourceUrl": "https://leetcode.com/problems/minimum-window-substring/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    string minWindow(string s, string t) {\n        return \"\"; // Shortest window of s covering t.\n    }\n};",
        "acmIoSpec": "stdin：第一行字符串 s；第二行字符串 t。stdout：一行，最小覆盖子串；不存在时输出空行。",
    },
    "io": {
        "method": "minWindow",
        "params": [{"name": "s", "type": "string"}, {"name": "t", "type": "string"}],
        "returns": "string",
    },
}


def windows(s, t):
    """For each right end, the shortest covering window ending there. Returns (best_len, ends)."""
    need = Counter(t)
    missing = len(t)
    left, best, ends = 0, None, []
    for right, c in enumerate(s):
        if need[c] > 0:
            missing -= 1
        need[c] -= 1
        if missing == 0:
            while need[s[left]] < 0:
                need[s[left]] += 1
                left += 1
            length = right - left + 1
            if best is None or length < best:
                best, ends = length, [right]
            elif length == best:
                ends.append(right)
    return best, ends


def solve(d):
    s, t = d["s"], d["t"]
    best, ends = windows(s, t)
    if best is None:
        return ""
    assert len(ends) == 1, "answer must be unique"
    return s[ends[0] - best + 1: ends[0] + 1]


def brute(d):
    s, t = d["s"], d["t"]
    need = Counter(t)
    for length in range(len(t), len(s) + 1):
        hits = [s[i:i + length] for i in range(len(s) - length + 1) if not need - Counter(s[i:i + length])]
        if hits:
            return hits[0]
    return ""


LETTERS = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"


def unique_case(seed, slen, tlen, alpha):
    r = rng(seed)
    while True:
        s = "".join(r.choice(alpha) for _ in range(slen))
        t = "".join(r.choice(alpha) for _ in range(tlen))
        best, ends = windows(s, t)
        if best is not None and len(ends) == 1:
            return {"s": s, "t": t}


def planted(seed, slen, tlen):
    # Filler from lowercase letters; t uses uppercase letters that appear only in one planted block.
    r = rng(seed)
    t = "".join(r.choice("ABCDEFGHIJKLMNOPQRSTUVWXYZ") for _ in range(tlen))
    block = list(t) + [r.choice("abc") for _ in range(tlen // 2)]
    r.shuffle(block)
    filler = [r.choice("abcdefghijklmnopqrstuvwxyz") for _ in range(slen - len(block))]
    pos = r.randint(0, len(filler))
    return {"s": "".join(filler[:pos] + block + filler[pos:]), "t": t}


tests = [
    {"id": "t1", "input": {"s": "ADOBECODEBANC", "t": "ABC"}, "sample": True, "expected": "BANC"},
    {"id": "t2", "input": {"s": "a", "t": "a"}, "sample": True, "expected": "a"},
    {"id": "t3", "input": {"s": "a", "t": "aa"}, "sample": True, "expected": ""},
    {"id": "t-longer", "input": {"s": "ab", "t": "abc"}, "sample": False},
    {"id": "case-sensitive", "input": {"s": "aAbBa", "t": "AB"}, "sample": False},
    {"id": "duplicates-in-t", "input": {"s": "baaxyzab", "t": "aab"}, "sample": False},
    {"id": "whole-string", "input": {"s": "xyzzy", "t": "yzzyx"}, "sample": False},
    {"id": "random-small", "input": unique_case(1, 60, 4, "abcdef"), "sample": False},
    {"id": "random-medium", "input": unique_case(2, 3000, 30, LETTERS), "sample": False},
    {"id": "max-size-100000", "input": unique_case(3, 100000, 200, LETTERS), "sample": False},
    {"id": "worst-case-planted", "input": planted(4, 100000, 20000), "sample": False},
    {"id": "worst-case-no-answer", "input": {"s": "a" * 100000, "t": "a" * 99999 + "b"}, "sample": False},
]
for t in tests[:8]:
    assert solve(t["input"]) == brute(t["input"]), t["id"]
write(problem, tests, solve)
