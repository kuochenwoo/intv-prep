import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from math import comb
from gen import write, rng

problem = {
    "id": "lc-62",
    "title": "不同路径",
    "difficulty": "medium",
    "tags": ["数学", "动态规划", "组合数学"],
    "statement": "机器人位于 m × n 网格的左上角，每步只能向右或向下移动一格，问到达右下角共有多少条不同的路径。",
    "en": {
        "title": "Unique Paths",
        "tags": [
            "Math",
            "Dynamic Programming",
            "Combinatorics"
        ],
        "statement": "A robot is at the top-left corner of an m × n grid and can only move one cell right or down at each step. How many distinct paths lead to the bottom-right corner?",
        "constraints": [
            "1 ≤ m, n ≤ 100",
            "The answer is guaranteed not to exceed 2 × 10^9"
        ],
        "complexity": {
            "time": "O(m · n) (local training target); O(min(m, n)) with the binomial formula",
            "space": "O(min(m, n)) extra space (rolling array)",
            "officialRequirement": "No official complexity requirement.",
            "notes": "The answer is the binomial coefficient C(m + n - 2, m - 1). Computing it by multiplying term by term and then dividing exactly needs 64-bit intermediates. Because the answer is capped at 2 × 10^9, m = n = 100 never occurs; the largest local tests are grids such as m = 100, n = 7 and m = 18, n = 17 whose answers are close to the cap."
        },
        "acmIoSpec": "stdin: first line integer m; second line integer n. stdout: one integer, the number of distinct paths."
    },
    "constraints": ["1 ≤ m, n ≤ 100", "数据保证答案不超过 2 × 10^9"],
    "complexity": {
        "time": "O(m · n)（本地训练目标）；组合数公式 O(min(m, n))",
        "space": "O(min(m, n)) 额外空间（滚动数组）",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "答案为组合数 C(m + n - 2, m - 1)。用组合数逐项相乘再整除时需 64 位中间值。由于答案上限 2 × 10^9，m = n = 100 不会出现；本地最大测试为 m = 100, n = 7 与 m = 18, n = 17 等答案接近上限的网格。",
        "sourceUrl": "https://leetcode.com/problems/unique-paths/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int uniquePaths(int m, int n) {\n        return 0; // Number of right/down paths in an m x n grid.\n    }\n};",
        "acmIoSpec": "stdin：第一行整数 m；第二行整数 n。stdout：一个整数，不同路径数。",
    },
    "io": {
        "method": "uniquePaths",
        "params": [{"name": "m", "type": "int"}, {"name": "n", "type": "int"}],
        "returns": "int",
    },
}


def solve(d):
    # Grid DP with Python big ints (independent of the closed form used in the assert).
    m, n = d["m"], d["n"]
    row = [1] * n
    for _ in range(1, m):
        for j in range(1, n):
            row[j] += row[j - 1]
    assert row[-1] <= 2 * 10**9, "answer exceeds the official guarantee"
    assert row[-1] == comb(m + n - 2, m - 1)
    return row[-1]


def case(i, m, n):
    return {"id": i, "input": {"m": m, "n": n}, "sample": False}


tests = [
    {"id": "t1", "input": {"m": 3, "n": 7}, "sample": True, "expected": 28},
    {"id": "t2", "input": {"m": 3, "n": 2}, "sample": True, "expected": 3},
    case("one-by-one", 1, 1),
    case("single-row-100", 1, 100),
    case("single-column-100", 100, 1),
    case("square-10", 10, 10),
    case("near-limit-17x18", 17, 18),
    case("near-limit-18x17", 18, 17),
    case("max-size-100x7", 100, 7),
    case("max-size-7x100", 7, 100),
]
write(problem, tests, solve)
