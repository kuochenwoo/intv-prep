import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-131",
    "title": "分割回文串",
    "difficulty": "medium",
    "tags": ["字符串", "动态规划", "回溯"],
    "statement": "给定字符串 s，把它切分成若干个非空子串，使每个子串都是回文串，返回所有可能的切分方案。每个方案按子串在 s 中的位置从左到右列出；方案之间的顺序任意。",
    "en": {'title': 'Palindrome Partitioning',
     'tags': ['String', 'Dynamic Programming', 'Backtracking'],
     'statement': 'Given a string s, split it into non-empty substrings such that every substring is a '
                  'palindrome, and return all possible partitions. Each partition lists its substrings from left '
                  'to right by their position in s; partitions may be in any order.',
     'constraints': ['1 ≤ s.length ≤ 16', 's consists only of lowercase English letters'],
     'complexity': {'time': 'O(n · 2^n) (local training target)',
                    'space': 'O(n^2) extra space (palindrome precomputation table) + O(n) recursion stack '
                             '(counted); output not counted',
                    'officialRequirement': 'No official complexity requirement.',
                    'notes': 'n is the length of s. The n - 1 cut points give 2^(n-1) ways to cut, all valid '
                             'when every letter is the same; precomputing isPal[i][j] with DP allows O(1) '
                             'palindrome checks. The local all-same-letter test has n = 13 (4096 partitions); '
                             'the official limit n = 16 (32768 partitions) produces about 1 MiB of output and '
                             'was reduced to keep the problem bank small, and there is also a random-string test '
                             'with n = 16. The checker ignores only the order of partitions.'},
     'acmIoSpec': 'stdin: one line with the string s. stdout: line 1 is the number of partitions P; then each '
                  'partition in turn: one line with its substring count k, then k lines with one substring each '
                  '(left to right as in s; partitions in any order).'},
    "constraints": [
        "1 ≤ s.length ≤ 16",
        "s 仅由小写英文字母组成",
    ],
    "complexity": {
        "time": "O(n · 2^n)（本地训练目标）",
        "space": "O(n^2) 额外空间（回文预处理表）+ O(n) 递归栈（计入）；输出不计入",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为 s 的长度。n - 1 个切点共 2^(n-1) 种切法，全为同一字母时每种都合法；用 DP 预处理 isPal[i][j] 可 O(1) 判断回文。本地全同字母测试为 n = 13（4096 个方案）；官方上限 n = 16（32768 个方案）的输出约 1 MiB，为控制题库体积而缩小，另有一组 n = 16 的随机字符串测试。判题只忽略方案之间的顺序。",
        "sourceUrl": "https://leetcode.com/problems/palindrome-partitioning/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    vector<vector<string>> partition(string s) {\n        return {}; // Every palindrome partition, in any order.\n    }\n};",
        "acmIoSpec": "stdin：一行字符串 s。stdout：第一行方案数 P；随后依次输出每个方案：先一行该方案的子串个数 k，再 k 行，每行一个子串（按在 s 中从左到右的顺序；方案之间顺序任意）。",
    },
    # 4096 partitions of "a"*13 print about 100 KiB.
    "limits": {"timeLimitMs": 1000, "memoryLimitMb": 128, "outputLimitKb": 256},
    "io": {
        "method": "partition",
        "params": [{"name": "s", "type": "string"}],
        "returns": "string[][]",
        "checker": "unordered",
    },
}


def solve(s):
    # Enumerate every cut mask and keep those whose pieces are all palindromes.
    n, out = len(s), []
    for mask in range(1 << (n - 1)):
        parts, start = [], 0
        for i in range(n - 1):
            if mask >> i & 1:
                parts.append(s[start:i + 1])
                start = i + 1
        parts.append(s[start:])
        if all(p == p[::-1] for p in parts):
            out.append(parts)
    return out


def check(inp, expected, solved):
    return sorted(expected) == sorted(solved)


r = rng(131)
tests = [
    {"id": "t1", "sample": True, "input": "aab", "expected": [["a", "a", "b"], ["aa", "b"]]},
    {"id": "t2", "sample": True, "input": "a", "expected": [["a"]]},
    {"id": "distinct", "sample": False, "input": "abcde"},
    {"id": "palindrome-whole", "sample": False, "input": "racecar"},
    {"id": "nested", "sample": False, "input": "abaaba"},
    {"id": "random-10", "sample": False, "input": "".join(r.choice("ab") for _ in range(10))},
    {"id": "max-size-16", "sample": False, "input": "".join(r.choice("ab") for _ in range(16))},
    {"id": "max-size-16-distinct", "sample": False, "input": "abcdefghijklmnop"},
    {"id": "worst-case-a13", "sample": False, "input": "a" * 13},
]
write(problem, tests, solve, check=check)
