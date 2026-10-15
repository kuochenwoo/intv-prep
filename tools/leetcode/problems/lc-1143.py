import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-1143",
    "title": "最长公共子序列",
    "difficulty": "medium",
    "tags": ["字符串", "动态规划"],
    "statement": "给定两个字符串 text1 和 text2，返回它们最长公共子序列的长度；不存在公共子序列时返回 0。子序列由原串删去若干字符且保持剩余字符相对顺序得到，不要求连续。",
    "constraints": ["1 ≤ text1.length, text2.length ≤ 1000；两串仅含小写英文字母"],
    "complexity": {
        "time": "O(m · n)（本地训练目标）",
        "space": "O(m · n) 二维 DP；滚动数组可降为 O(min(m, n)) 额外空间",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "m、n 为两串长度。dp[i][j] 为 text1 前 i 个字符与 text2 前 j 个字符的 LCS 长度：末字符相同取 dp[i-1][j-1] + 1，否则取 max(dp[i-1][j], dp[i][j-1])。",
        "sourceUrl": "https://leetcode.com/problems/longest-common-subsequence/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int longestCommonSubsequence(string text1, string text2) {\n        return 0; // Length of the longest common subsequence.\n    }\n};",
        "acmIoSpec": "stdin：第一行字符串 text1；第二行字符串 text2。stdout：一个整数，最长公共子序列的长度。",
    },
    "io": {
        "method": "longestCommonSubsequence",
        "params": [{"name": "text1", "type": "string"}, {"name": "text2", "type": "string"}],
        "returns": "int",
    },
}


def solve(d):
    # Bottom-up table indexed on suffixes (i..end), not prefixes as in the C++ solutions.
    a, b = d["text1"], d["text2"]
    nxt = [0] * (len(b) + 1)
    for i in range(len(a) - 1, -1, -1):
        cur = [0] * (len(b) + 1)
        for j in range(len(b) - 1, -1, -1):
            cur[j] = nxt[j + 1] + 1 if a[i] == b[j] else max(nxt[j], cur[j + 1])
        nxt = cur
    return nxt[0]


def brute(d):
    from itertools import combinations
    a, b = d["text1"], d["text2"]
    def is_sub(t, s):
        it = iter(s)
        return all(c in it for c in t)
    for k in range(len(a), 0, -1):
        if any(is_sub(c, b) for c in combinations(a, k)):
            return k
    return 0


r = rng(1143)
word = lambda n, al="abcdefghijklmnopqrstuvwxyz": "".join(r.choice(al) for _ in range(n))
case = lambda i, a, b: {"id": i, "input": {"text1": a, "text2": b}, "sample": False}
tests = [
    {"id": "t1", "input": {"text1": "abcde", "text2": "ace"}, "sample": True, "expected": 3},
    {"id": "t2", "input": {"text1": "abc", "text2": "abc"}, "sample": True, "expected": 3},
    {"id": "t3", "input": {"text1": "abc", "text2": "def"}, "sample": True, "expected": 0},
    case("single-chars-equal", "q", "q"),
    case("single-vs-long", "z", "abcdefghijklmnopqrstuvwxyz"),
    case("reversed", "abcdef", "fedcba"),
    case("random-small", word(10, "abc"), word(12, "abc")),
    case("random-200", word(200, "abcd"), word(150, "abcd")),
    case("max-size-1000", word(1000, "ab"), word(1000, "ab")),
    case("max-size-1000-full-alphabet", word(1000), word(1000)),
    case("max-size-1000-disjoint", "a" * 1000, "b" * 1000),
]
for t in tests:
    if len(t["input"]["text1"]) <= 12:
        assert brute(t["input"]) == solve(t["input"]), t["id"]
write(problem, tests, solve)
