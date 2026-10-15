import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng
from math import comb

problem = {
    "id": "lc-118",
    "title": "杨辉三角",
    "difficulty": "easy",
    "tags": ["数组", "动态规划"],
    "statement": "给定非负整数 numRows，返回杨辉三角的前 numRows 行。第 i 行（从 0 开始）有 i+1 个数，首尾都是 1，其余每个数等于上一行中它左上方与正上方两个数之和。",
    "constraints": ["1 ≤ numRows ≤ 30"],
    "complexity": {
        "time": "O(numRows²)（本地训练目标）",
        "space": "O(1) 额外空间（不计输出的 O(numRows²)）",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "numRows 为行数，输出共 numRows(numRows+1)/2 个数。第 29 行最大值 C(29,14) = 77558760，在 int 范围内。",
        "sourceUrl": "https://leetcode.com/problems/pascals-triangle/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    vector<vector<int>> generate(int numRows) {\n        return {}; // First numRows rows of Pascal's triangle.\n    }\n};",
        "acmIoSpec": "stdin：一行整数 numRows。stdout：第一行行数 numRows；随后每行输出杨辉三角的一行，数之间以空格分隔。",
    },
    "io": {
        "method": "generate",
        "params": [{"name": "numRows", "type": "int"}],
        "returns": "int[][]",
    },
}


def solve(n):
    # Closed form via binomial coefficients (the C++ solutions use additions).
    return [[comb(i, j) for j in range(i + 1)] for i in range(n)]


tests = [
    {"id": "t1", "input": 5, "sample": True,
     "expected": [[1], [1, 1], [1, 2, 1], [1, 3, 3, 1], [1, 4, 6, 4, 1]]},
    {"id": "t2", "input": 1, "sample": True, "expected": [[1]]},
    {"id": "two-rows", "input": 2, "sample": False},
    {"id": "rows-13", "input": 13, "sample": False},
    {"id": "max-size-30", "input": 30, "sample": False},
]
write(problem, tests, solve)
