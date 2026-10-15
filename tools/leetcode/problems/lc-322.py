import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-322",
    "title": "零钱兑换",
    "difficulty": "medium",
    "tags": ["广度优先搜索", "数组", "动态规划"],
    "statement": "给定若干种硬币面额 coins（每种数量不限）和目标金额 amount，求凑出恰好 amount 所需的最少硬币数；如果无法凑出，返回 -1。",
    "en": {
        "title": "Coin Change",
        "tags": [
            "Breadth-First Search",
            "Array",
            "Dynamic Programming"
        ],
        "statement": "Given coin denominations coins (unlimited supply of each) and a target amount, find the minimum number of coins needed to make exactly amount; if it cannot be made, return -1.",
        "constraints": [
            "1 ≤ coins.length ≤ 12; 1 ≤ coins[i] ≤ 2³¹ - 1; 0 ≤ amount ≤ 10000"
        ],
        "complexity": {
            "time": "O(amount · m) (local training target)",
            "space": "O(amount) extra space",
            "officialRequirement": "No official complexity requirement.",
            "notes": "m is the number of denominations. Unbounded knapsack: dp[x] = 1 + min(dp[x - c]). Greedy (largest coins first) is not always optimal; note that denominations can be as large as 2³¹ - 1, so guard against overflow when computing x + c."
        },
        "acmIoSpec": "stdin: line 1 is the number of denominations m; line 2 has m integers coins; line 3 is the integer amount. stdout: one integer, the minimum number of coins or -1."
    },
    "constraints": ["1 ≤ coins.length ≤ 12；1 ≤ coins[i] ≤ 2³¹ - 1；0 ≤ amount ≤ 10000"],
    "complexity": {
        "time": "O(amount · m)（本地训练目标）",
        "space": "O(amount) 额外空间",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "m 为面额种数。完全背包：dp[x] = 1 + min(dp[x - c])。贪心（先用大面额）不一定最优；注意面额可达 2³¹ - 1，计算 x + c 时防止溢出。",
        "sourceUrl": "https://leetcode.com/problems/coin-change/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int coinChange(vector<int>& coins, int amount) {\n        return -1; // Fewest coins summing to amount, or -1.\n    }\n};",
        "acmIoSpec": "stdin：第一行面额种数 m；第二行 m 个整数 coins；第三行整数 amount。stdout：一个整数，最少硬币数或 -1。",
    },
    "io": {
        "method": "coinChange",
        "params": [{"name": "coins", "type": "int[]"}, {"name": "amount", "type": "int"}],
        "returns": "int",
    },
}


def brute(coins, amount):
    # Exhaustive search over counts of each coin (tiny inputs only).
    best = None

    def go(i, left, used):
        nonlocal best
        if left == 0:
            best = used if best is None else min(best, used)
            return
        if i == len(coins):
            return
        c = coins[i]
        for k in range(left // c + 1):
            go(i + 1, left - k * c, used + k)

    go(0, amount, 0)
    return -1 if best is None else best


def solve(data):
    coins, amount = data["coins"], data["amount"]
    INF = float("inf")
    dp = [0] + [INF] * amount
    for c in coins:  # coin-major order (the C++ DP is amount-major)
        for x in range(c, amount + 1):
            if dp[x - c] + 1 < dp[x]:
                dp[x] = dp[x - c] + 1
    ans = -1 if dp[amount] == INF else dp[amount]
    if amount <= 60 and len(coins) <= 4:
        assert ans == brute(coins, amount)
    return ans


r = rng(322)
BIG = 2**31 - 1
tests = [
    {"id": "t1", "input": {"coins": [1, 2, 5], "amount": 11}, "sample": True, "expected": 3},
    {"id": "t2", "input": {"coins": [2], "amount": 3}, "sample": True, "expected": -1},
    {"id": "t3", "input": {"coins": [1], "amount": 0}, "sample": True, "expected": 0},
    {"id": "greedy-fails", "input": {"coins": [1, 3, 4], "amount": 6}, "sample": False},
    {"id": "greedy-fails-2", "input": {"coins": [9, 6, 5, 1], "amount": 11}, "sample": False},
    {"id": "huge-coin-only", "input": {"coins": [BIG], "amount": 2}, "sample": False},
    {"id": "huge-coin-mixed", "input": {"coins": [BIG, 3, 7], "amount": 25}, "sample": False},
    {"id": "unreachable-even", "input": {"coins": [4, 6, 10], "amount": 47}, "sample": False},
    {"id": "random-small", "input": {"coins": [r.randint(1, 20) for _ in range(4)], "amount": 55}, "sample": False},
    {"id": "max-size-ones", "input": {"coins": [1], "amount": 10000}, "sample": False},
    {"id": "max-size-12-coins", "input": {"coins": [r.randint(100, 3000) for _ in range(11)] + [BIG], "amount": 10000}, "sample": False},
    {"id": "max-size-unreachable", "input": {"coins": [2 * r.randint(1, 500) for _ in range(12)], "amount": 9999}, "sample": False},
    {"id": "max-size-small-coins", "input": {"coins": [1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31], "amount": 10000}, "sample": False},
]
write(problem, tests, solve)
