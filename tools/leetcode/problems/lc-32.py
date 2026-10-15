import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-32",
    "title": "最长有效括号",
    "difficulty": "hard",
    "tags": ["栈", "字符串", "动态规划"],
    "statement": "给定只由 '(' 和 ')' 组成的字符串 s，求其中最长的、括号正确匹配的连续子串的长度。",
    "constraints": ["0 ≤ s.length ≤ 3 × 10^4；s[i] 为 '(' 或 ')'"],
    "complexity": {
        "time": "O(n)（本地训练目标）",
        "space": "O(n) 额外空间（栈或 DP 数组）；左右两遍计数法可做到 O(1)",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为字符串长度。栈中保存“最后一个未匹配右括号”的下标作为基准；或从左到右、从右到左各扫一遍，统计左右括号数相等时的长度。空串答案为 0。",
        "sourceUrl": "https://leetcode.com/problems/longest-valid-parentheses/",
    },
    "en": {
        "title": "Longest Valid Parentheses",
        "tags": [
            "Stack",
            "String",
            "Dynamic Programming"
        ],
        "statement": "Given a string s consisting only of '(' and ')', find the length of its longest contiguous substring in which the parentheses are correctly matched.",
        "constraints": [
            "0 ≤ s.length ≤ 3 × 10^4; s[i] is '(' or ')'"
        ],
        "complexity": {
            "time": "O(n) (local training target)",
            "space": "O(n) extra space (stack or DP array); the two-pass left/right counting method achieves O(1)",
            "officialRequirement": "No official complexity requirement.",
            "notes": "n is the string length. Keep the index of the \"last unmatched closing parenthesis\" on the stack as the base; or scan once left to right and once right to left, recording the length whenever the counts of '(' and ')' are equal. The answer for an empty string is 0."
        },
        "acmIoSpec": "stdin: one line, the string s (may be an empty line). stdout: one integer, the length of the longest valid parentheses substring."
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int longestValidParentheses(string s) {\n        return 0; // Length of the longest well-formed substring.\n    }\n};",
        "acmIoSpec": "stdin：一行字符串 s（可能为空行）。stdout：一个整数，最长有效括号子串的长度。",
    },
    "io": {
        "method": "longestValidParentheses",
        "params": [{"name": "s", "type": "string"}],
        "returns": "int",
    },
}


def solve(s):
    # dp[i] = length of the longest valid substring ending at i.
    dp = [0] * len(s)
    for i in range(1, len(s)):
        if s[i] != ")":
            continue
        j = i - dp[i - 1] - 1
        if j >= 0 and s[j] == "(":
            dp[i] = dp[i - 1] + 2 + (dp[j - 1] if j >= 1 else 0)
    return max(dp, default=0)


def valid(t):
    d = 0
    for c in t:
        d += 1 if c == "(" else -1
        if d < 0:
            return False
    return d == 0


def brute(s):
    return max((j - i for i in range(len(s)) for j in range(i, len(s) + 1) if valid(s[i:j])), default=0)


r = rng(32)
tests = [
    {"id": "t1", "input": "(()", "sample": True, "expected": 2},
    {"id": "t2", "input": ")()())", "sample": True, "expected": 4},
    {"id": "t3", "input": "", "sample": True, "expected": 0},
    {"id": "all-open", "input": "((((", "sample": False},
    {"id": "all-close", "input": "))))", "sample": False},
    {"id": "nested-and-chained", "input": "()(())", "sample": False},
    {"id": "broken-middle", "input": "()(()", "sample": False},
    {"id": "random-80", "input": "".join(r.choice("()") for _ in range(80)), "sample": False},
    {"id": "max-size-30000", "input": "".join(r.choice("(()") if i % 7 else r.choice("()") for i in range(30000)), "sample": False},
    {"id": "max-size-30000-nested", "input": "(" * 15000 + ")" * 15000, "sample": False},
    {"id": "max-size-30000-open-prefix", "input": "(" * 10000 + "()" * 10000, "sample": False},
]
for t in tests:
    if len(t["input"]) <= 80:
        assert brute(t["input"]) == solve(t["input"]), t["id"]
write(problem, tests, solve)
