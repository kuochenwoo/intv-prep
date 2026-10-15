import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-279",
    "title": "完全平方数",
    "difficulty": "medium",
    "tags": ["广度优先搜索", "数学", "动态规划"],
    "statement": "给定正整数 n，把它表示成若干个完全平方数（1, 4, 9, 16, …，可重复使用）之和，求所需完全平方数的最少个数。",
    "constraints": ["1 ≤ n ≤ 10000"],
    "complexity": {
        "time": "O(n√n)（本地训练目标，动态规划）",
        "space": "O(n) 额外空间",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为输入整数。dp[i] = 1 + min(dp[i - j²])。利用四平方和定理与 Legendre 三平方和定理可做到 O(√n)：答案只可能是 1~4，n 形如 4^a(8b+7) 时为 4。",
        "sourceUrl": "https://leetcode.com/problems/perfect-squares/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int numSquares(int n) {\n        return 0; // Fewest perfect squares summing to n.\n    }\n};",
        "acmIoSpec": "stdin：一行整数 n。stdout：一个整数，最少个数。",
    },
    "io": {
        "method": "numSquares",
        "params": [{"name": "n", "type": "int"}],
        "returns": "int",
    },
}


def bfs(n):
    # Breadth-first search over remainders: level = number of squares used.
    squares = [k * k for k in range(1, int(n ** 0.5) + 2) if k * k <= n]
    seen, frontier, level = {n}, [n], 0
    while True:
        level += 1
        nxt = []
        for v in frontier:
            for sq in squares:
                if sq > v:
                    break
                if sq == v:
                    return level
                if v - sq not in seen:
                    seen.add(v - sq)
                    nxt.append(v - sq)
        frontier = nxt


tests = [
    {"id": "t1", "input": 12, "sample": True, "expected": 3},
    {"id": "t2", "input": 13, "sample": True, "expected": 2},
    {"id": "min-1", "input": 1, "sample": False},
    {"id": "square-9801", "input": 9801, "sample": False},
    {"id": "needs-four-7", "input": 7, "sample": False},
    {"id": "needs-four-9980", "input": 9980, "sample": False},
    {"id": "n-6015", "input": 6015, "sample": False},
    {"id": "n-9997", "input": 9997, "sample": False},
    {"id": "max-size-10000", "input": 10000, "sample": False},
    {"id": "max-size-9999", "input": 9999, "sample": False},
]
write(problem, tests, bfs)
