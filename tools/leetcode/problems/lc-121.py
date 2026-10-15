import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-121",
    "title": "买卖股票的最佳时机",
    "difficulty": "easy",
    "tags": ["数组", "动态规划"],
    "statement": "数组 prices 给出某支股票每天的价格。你只能选择某一天买入一次，并在之后的某一天卖出一次。求能获得的最大利润；如果无法获利，返回 0。",
    "constraints": ["1 ≤ prices.length ≤ 100000；0 ≤ prices[i] ≤ 10000"],
    "complexity": {
        "time": "O(n)（本地训练目标）",
        "space": "O(1) 额外空间",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为天数。一次遍历维护此前的最低价，用当日价格减去它更新答案；枚举所有买卖日组合的 O(n²) 做法在 n = 100000 时会超时。",
        "sourceUrl": "https://leetcode.com/problems/best-time-to-buy-and-sell-stock/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int maxProfit(vector<int>& prices) {\n        return 0; // Best profit from one buy followed by one sell.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数 prices。stdout：一个整数，最大利润。",
    },
    "io": {
        "method": "maxProfit",
        "params": [{"name": "prices", "type": "int[]"}],
        "returns": "int",
    },
}


def brute(p):
    return max([0] + [p[j] - p[i] for i in range(len(p)) for j in range(i + 1, len(p))])


def solve(p):
    # Suffix maximum scan (the C++ solutions scan prefix minima / day-to-day gains).
    best, hi = 0, -1
    for x in reversed(p):
        hi = max(hi, x)
        best = max(best, hi - x)
    if len(p) <= 300:
        assert best == brute(p)
    return best


r = rng(121)
tests = [
    {"id": "t1", "input": [7, 1, 5, 3, 6, 4], "sample": True, "expected": 5},
    {"id": "t2", "input": [7, 6, 4, 3, 1], "sample": True, "expected": 0},
    {"id": "single", "input": [5], "sample": False},
    {"id": "all-equal", "input": [3] * 20, "sample": False},
    {"id": "min-later", "input": [2, 4, 1, 3], "sample": False},
    {"id": "random-300", "input": [r.randint(0, 10000) for _ in range(300)], "sample": False},
    # Decreasing then a late spike: a quadratic scan does the full n²/2 work.
    {"id": "max-size-100000", "input": [999 - i // 101 for i in range(99999)] + [10000], "sample": False},
    {"id": "random-5000", "input": [r.randint(0, 10000) for _ in range(5000)], "sample": False},
]
write(problem, tests, solve)
