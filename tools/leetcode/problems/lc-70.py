import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-70",
    "title": "爬楼梯",
    "difficulty": "easy",
    "tags": ["记忆化搜索", "数学", "动态规划"],
    "statement": "楼梯共有 n 级台阶，每一步可以向上走 1 级或 2 级。从地面出发恰好走到第 n 级，一共有多少种不同的走法？",
    "en": {
        "title": "Climbing Stairs",
        "tags": [
            "Memoization",
            "Math",
            "Dynamic Programming"
        ],
        "statement": "A staircase has n steps, and each move climbs either 1 or 2 steps. Starting from the ground, how many distinct ways are there to land exactly on step n?",
        "constraints": [
            "1 ≤ n ≤ 45"
        ],
        "complexity": {
            "time": "O(n) (local training target)",
            "space": "O(1) extra space (rolling variables)",
            "officialRequirement": "No official complexity requirement.",
            "notes": "n is the number of steps. f(n) = f(n-1) + f(n-2), i.e. the Fibonacci sequence; f(45) = 1836311903 still fits in a 32-bit signed integer. Fast matrix exponentiation achieves O(log n), but it is unnecessary for n ≤ 45."
        },
        "acmIoSpec": "stdin: one line with integer n. stdout: one integer, the total number of ways."
    },
    "constraints": ["1 ≤ n ≤ 45"],
    "complexity": {
        "time": "O(n)（本地训练目标）",
        "space": "O(1) 额外空间（滚动变量）",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为台阶数。f(n) = f(n-1) + f(n-2)，即斐波那契数列；f(45) = 1836311903 仍在 32 位有符号整数范围内。矩阵快速幂可做到 O(log n)，但 n ≤ 45 时没有必要。",
        "sourceUrl": "https://leetcode.com/problems/climbing-stairs/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int climbStairs(int n) {\n        return 0; // Number of distinct ways to reach step n.\n    }\n};",
        "acmIoSpec": "stdin：一行整数 n。stdout：一个整数，走法总数。",
    },
    "io": {
        "method": "climbStairs",
        "params": [{"name": "n", "type": "int"}],
        "returns": "int",
    },
}


def brute(n):
    # Count step sequences directly (small n only).
    if n <= 1:
        return 1
    return brute(n - 1) + brute(n - 2)


def solve(n):
    a, b = 1, 1  # ways to reach step 0 and step 1
    for _ in range(n - 1):
        a, b = b, a + b
    if n <= 20:
        assert b == brute(n)
    return b


tests = [
    {"id": "t1", "input": 2, "sample": True, "expected": 2},
    {"id": "t2", "input": 3, "sample": True, "expected": 3},
    {"id": "min-1", "input": 1, "sample": False},
    {"id": "n-10", "input": 10, "sample": False},
    {"id": "n-30", "input": 30, "sample": False},
    {"id": "n-44", "input": 44, "sample": False},
    {"id": "max-size-45", "input": 45, "sample": False},
]
write(problem, tests, solve)
