import sys, pathlib, itertools
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write

problem = {
    "id": "lc-22",
    "title": "括号生成",
    "difficulty": "medium",
    "tags": ["字符串", "动态规划", "回溯"],
    "statement": "给定括号对数 n，生成所有由 n 对圆括号组成的合法（有效配对）括号字符串，顺序任意。",
    "constraints": ["1 ≤ n ≤ 8"],
    "complexity": {
        "time": "O(4^n / √n)（本地训练目标；即第 n 个卡特兰数乘以长度 2n）",
        "space": "O(n) 额外空间（回溯递归栈与当前字符串，计入递归栈；输出不计入）",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为括号对数，结果数为卡特兰数 C(n)（n = 8 时为 1430）。回溯时保证左括号数 ≤ n 且右括号数 ≤ 左括号数即可只生成合法串。判题忽略结果之间的顺序。",
        "sourceUrl": "https://leetcode.com/problems/generate-parentheses/",
    },
    "en": {
        "title": "Generate Parentheses",
        "tags": [
            "String",
            "Dynamic Programming",
            "Backtracking"
        ],
        "statement": "Given the number of pairs n, generate all well-formed (validly matched) strings made of n pairs of parentheses, in any order.",
        "constraints": [
            "1 ≤ n ≤ 8"
        ],
        "complexity": {
            "time": "O(4^n / √n) (local training target; i.e. the n-th Catalan number times the length 2n)",
            "space": "O(n) extra space (backtracking recursion stack and current string, recursion stack included; output not counted)",
            "officialRequirement": "No official complexity requirement.",
            "notes": "n is the number of pairs; the number of results is the Catalan number C(n) (1430 for n = 8). During backtracking, keeping the count of '(' ≤ n and the count of ')' ≤ the count of '(' generates only valid strings. The judge ignores the order of results."
        },
        "acmIoSpec": "stdin: one line, the integer n. stdout: line 1 is the number of results k; then k lines, each with one valid parentheses string (any order)."
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    vector<string> generateParenthesis(int n) {\n        return {}; // All well-formed strings, in any order.\n    }\n};",
        "acmIoSpec": "stdin：一行整数 n。stdout：第一行结果个数 k；随后 k 行，每行一个合法括号串（顺序任意）。",
    },
    "io": {
        "method": "generateParenthesis",
        "params": [{"name": "n", "type": "int"}],
        "returns": "string[]",
        "checker": "unordered",
    },
}


def solve(n):
    # Brute force: every string of length 2n, keep the balanced ones.
    out = []
    for bits in itertools.product("()", repeat=2 * n):
        depth = 0
        for c in bits:
            depth += 1 if c == "(" else -1
            if depth < 0:
                break
        if depth == 0:
            out.append("".join(bits))
    return out


def check(inp, expected, solved):
    return sorted(expected) == sorted(solved)


tests = [
    {"id": "t1", "sample": True, "input": 3, "expected": ["((()))", "(()())", "(())()", "()(())", "()()()"]},
    {"id": "t2", "sample": True, "input": 1, "expected": ["()"]},
    {"id": "n2", "sample": False, "input": 2},
    {"id": "n5", "sample": False, "input": 5},
    {"id": "n7", "sample": False, "input": 7},
    {"id": "max-size-8", "sample": False, "input": 8},
]
write(problem, tests, solve, check=check)
