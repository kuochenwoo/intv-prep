import sys, pathlib
from collections import Counter
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-438",
    "title": "找到字符串中所有字母异位词",
    "difficulty": "medium",
    "tags": ["哈希表", "字符串", "滑动窗口"],
    "statement": "给定两个字符串 s 和 p，找出 s 中所有是 p 的字母异位词（字母种类与每种字母的出现次数都和 p 相同）的子串，返回这些子串的起始下标（0-based），顺序不限。",
    "constraints": ["1 ≤ s.length, p.length ≤ 30000", "s 和 p 只包含小写英文字母"],
    "complexity": {
        "time": "O(n + m)（本地训练目标；字母表大小视为常数 26）",
        "space": "O(1) 额外空间（26 个计数）；输出不计入",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n = s.length，m = p.length。维护长度为 m 的滑动窗口的字母计数，并记录与 p 计数不一致的字母个数。逐窗口排序比较为 O(n·m log m)，在 m 较大的测试上会超时。",
        "sourceUrl": "https://leetcode.com/problems/find-all-anagrams-in-a-string/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    vector<int> findAnagrams(string s, string p) {\n        return {}; // Start indices, any order.\n    }\n};",
        "acmIoSpec": "stdin：第一行字符串 s；第二行字符串 p。stdout：一行，所有起始下标，以空格分隔，顺序不限；没有时输出空行。",
    },
    # The all-matching max-size test prints ~30000 indices, above the 64 KiB default.
    "limits": {"timeLimitMs": 1000, "memoryLimitMb": 128, "outputLimitKb": 256},
    "io": {
        "method": "findAnagrams",
        "params": [{"name": "s", "type": "string"}, {"name": "p", "type": "string"}],
        "returns": "int[]",
        "checker": "unordered",
    },
}


def solve(d):
    s, p = d["s"], d["p"]
    n, m = len(s), len(p)
    if m > n:
        return []
    if n * m <= 200000:
        key = sorted(p)
        return [i for i in range(n - m + 1) if sorted(s[i:i + m]) == key]
    target = Counter(p)
    win = Counter(s[:m])
    out = [0] if win == target else []
    for i in range(m, n):
        win[s[i]] += 1
        win[s[i - m]] -= 1
        if win[s[i - m]] == 0:
            del win[s[i - m]]
        if win == target:
            out.append(i - m + 1)
    return out


r = rng(438)


def rs(k, alpha="abc"):
    return "".join(r.choice(alpha) for _ in range(k))


big_p = rs(10000, "ab")
big_s = list(rs(30000, "ab"))
tests = [
    {"id": "t1", "input": {"s": "cbaebabacd", "p": "abc"}, "sample": True, "expected": [0, 6]},
    {"id": "t2", "input": {"s": "abab", "p": "ab"}, "sample": True, "expected": [0, 1, 2]},
    {"id": "p-longer", "input": {"s": "ab", "p": "abc"}, "sample": False},
    {"id": "single-char", "input": {"s": "a", "p": "a"}, "sample": False},
    {"id": "no-match", "input": {"s": "aaaaaa", "p": "b"}, "sample": False},
    {"id": "equal-length", "input": {"s": "listen", "p": "silent"}, "sample": False},
    {"id": "random-2000", "input": {"s": rs(2000), "p": rs(4)}, "sample": False},
    {"id": "max-size-30000", "input": {"s": "ab" * 15000, "p": "ba"}, "sample": False},
    {"id": "worst-case-long-p", "input": {"s": "".join(big_s), "p": big_p}, "sample": False},
    {"id": "worst-case-all-same", "input": {"s": "z" * 30000, "p": "z" * 15000}, "sample": False},
]
write(problem, tests, solve)
