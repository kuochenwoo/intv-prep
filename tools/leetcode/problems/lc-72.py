import sys, pathlib
from functools import lru_cache
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-72",
    "title": "编辑距离",
    "difficulty": "medium",
    "tags": ["字符串", "动态规划"],
    "statement": "给定两个单词 word1 和 word2，每次操作可以在 word1 中插入一个字符、删除一个字符或替换一个字符。求把 word1 变成 word2 所需的最少操作次数。",
    "constraints": ["0 ≤ word1.length, word2.length ≤ 500；两串仅含小写英文字母"],
    "complexity": {
        "time": "O(m · n)（本地训练目标）",
        "space": "O(m · n) 二维 DP；滚动数组可降为 O(min(m, n)) 额外空间",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "m、n 为两串长度。dp[i][j] 为 word1 前 i 个字符变为 word2 前 j 个字符的最少操作数；任一串为空时答案为另一串的长度（空串输入为空行）。",
        "sourceUrl": "https://leetcode.com/problems/edit-distance/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int minDistance(string word1, string word2) {\n        return 0; // Minimum number of insert/delete/replace operations.\n    }\n};",
        "acmIoSpec": "stdin：第一行字符串 word1；第二行字符串 word2（任一行都可能为空行）。stdout：一个整数，最少操作次数。",
    },
    "io": {
        "method": "minDistance",
        "params": [{"name": "word1", "type": "string"}, {"name": "word2", "type": "string"}],
        "returns": "int",
    },
}


def solve(d):
    # Suffix-indexed DP (C++ solutions index prefixes).
    a, b = d["word1"], d["word2"]
    m, n = len(a), len(b)
    nxt = list(range(n, -1, -1))  # row i = m: distance from "" to b[j:]
    for i in range(m - 1, -1, -1):
        cur = [0] * (n + 1)
        cur[n] = m - i
        for j in range(n - 1, -1, -1):
            if a[i] == b[j]:
                cur[j] = nxt[j + 1]
            else:
                cur[j] = 1 + min(nxt[j + 1], nxt[j], cur[j + 1])
        nxt = cur
    return nxt[0]


def brute(d):
    a, b = d["word1"], d["word2"]

    @lru_cache(None)
    def go(i, j):
        if i == len(a) or j == len(b):
            return len(a) - i + len(b) - j
        if a[i] == b[j]:
            return go(i + 1, j + 1)
        return 1 + min(go(i + 1, j + 1), go(i + 1, j), go(i, j + 1))
    return go(0, 0)


r = rng(72)
word = lambda n, al="abcdefghijklmnopqrstuvwxyz": "".join(r.choice(al) for _ in range(n))
case = lambda i, a, b: {"id": i, "input": {"word1": a, "word2": b}, "sample": False}
tests = [
    {"id": "t1", "input": {"word1": "horse", "word2": "ros"}, "sample": True, "expected": 3},
    {"id": "t2", "input": {"word1": "intention", "word2": "execution"}, "sample": True, "expected": 5},
    case("both-empty", "", ""),
    case("first-empty", "", "abc"),
    case("second-empty", "hello", ""),
    case("identical", "same", "same"),
    case("single-replace", "a", "b"),
    case("random-small", word(15, "abc"), word(18, "abc")),
    case("random-120", word(120, "abcde"), word(100, "abcde")),
    case("max-size-500", word(500, "abc"), word(500, "abc")),
    case("max-size-500-full-alphabet", word(500), word(500)),
    case("max-size-500-vs-empty", word(500), ""),
]
for t in tests:
    if len(t["input"]["word1"]) + len(t["input"]["word2"]) <= 250:
        assert brute(t["input"]) == solve(t["input"]), t["id"]
write(problem, tests, solve)
