import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-5",
    "title": "最长回文子串",
    "difficulty": "medium",
    "tags": ["双指针", "字符串", "动态规划"],
    "statement": "给定字符串 s，返回 s 中最长的回文连续子串。若有多个长度相同的最长回文子串，返回任意一个均可。",
    "constraints": ["1 ≤ s.length ≤ 1000；s 仅由数字和英文字母组成"],
    "complexity": {
        "time": "O(n^2)（本地训练目标，中心扩展）；Manacher 算法可达 O(n)",
        "space": "中心扩展 O(1) 额外空间（不含返回值）；区间 DP 为 O(n^2)",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为字符串长度。以每个字符和每个字符间隙为中心向两侧扩展。答案可能不唯一，判题只检查返回串是 s 的子串、是回文且长度最长。",
        "sourceUrl": "https://leetcode.com/problems/longest-palindromic-substring/",
    },
    "en": {
        "title": "Longest Palindromic Substring",
        "tags": [
            "Two Pointers",
            "String",
            "Dynamic Programming"
        ],
        "statement": "Given a string s, return the longest contiguous substring of s that is a palindrome. If several longest palindromic substrings have the same length, any one of them may be returned.",
        "constraints": [
            "1 ≤ s.length ≤ 1000; s consists only of digits and English letters"
        ],
        "complexity": {
            "time": "O(n^2) (local training target, expand around center); Manacher's algorithm achieves O(n)",
            "space": "Expand around center: O(1) extra space (excluding the return value); interval DP: O(n^2)",
            "officialRequirement": "No official complexity requirement.",
            "notes": "n is the string length. Use each character and each gap between characters as a center and expand outward. The answer may not be unique; the judge only checks that the returned string is a substring of s, is a palindrome, and has the maximum length."
        },
        "acmIoSpec": "stdin: one line, the string s. stdout: one line, any one longest palindromic substring."
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    string longestPalindrome(string s) {\n        return \"\"; // Any longest palindromic substring of s.\n    }\n};",
        "acmIoSpec": "stdin：一行字符串 s。stdout：一行，任意一个最长回文子串。",
    },
    "io": {
        "method": "longestPalindrome",
        "params": [{"name": "s", "type": "string"}],
        "returns": "string",
        "checker": "longest_palindrome",
    },
}


def solve(s):
    # Interval DP over all substrings (independent of both C++ solutions).
    n = len(s)
    pal = [[False] * n for _ in range(n)]
    best = (0, 1)
    for i in range(n - 1, -1, -1):
        for j in range(i, n):
            if s[i] == s[j] and (j - i < 2 or pal[i + 1][j - 1]):
                pal[i][j] = True
                if j - i + 1 > best[1] - best[0]:
                    best = (i, j + 1)
    return s[best[0]:best[1]]


def check(s, expected, solved):
    return len(expected) == len(solved) and expected in s and expected == expected[::-1]


r = rng(5)
alnum = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
half = "".join(r.choice("ab") for _ in range(240))
tests = [
    {"id": "t1", "input": "babad", "sample": True, "expected": "bab"},
    {"id": "t2", "input": "cbbd", "sample": True, "expected": "bb"},
    {"id": "single", "input": "Z", "sample": False},
    {"id": "all-distinct", "input": "abc123XYZ", "sample": False},
    {"id": "whole-string", "input": "racecar", "sample": False},
    {"id": "even-at-end", "input": "xyzabba", "sample": False},
    {"id": "case-sensitive", "input": "Aa1aA", "sample": False},
    {"id": "random-200", "input": "".join(r.choice("abc") for _ in range(200)), "sample": False},
    {"id": "max-size-1000", "input": "".join(r.choice(alnum) for _ in range(1000)), "sample": False},
    {"id": "max-size-1000-same", "input": "a" * 1000, "sample": False},
    {"id": "max-size-1000-buried", "input": "".join(r.choice("ab") for _ in range(260)) + half + "c" + half[::-1] + "".join(r.choice("ab") for _ in range(259)), "sample": False},
]
write(problem, tests, solve, check=lambda s, e, got: check(s, e, got))
