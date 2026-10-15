import sys, pathlib
from itertools import combinations
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-198",
    "title": "打家劫舍",
    "difficulty": "medium",
    "tags": ["数组", "动态规划"],
    "statement": "一条街上有 n 间房屋排成一排，第 i 间存有现金 nums[i]。相邻的两间房屋不能在同一晚都被偷（否则触发警报）。在不触发警报的前提下，最多能偷到多少现金？",
    "en": {'title': 'House Robber',
     'tags': ['Array', 'Dynamic Programming'],
     'statement': 'There are n houses in a row on a street; house i holds nums[i] in cash. Two '
                  'adjacent houses cannot both be robbed on the same night (that triggers the alarm). '
                  'What is the maximum amount of cash you can rob without triggering the alarm?',
     'constraints': ['1 ≤ n ≤ 100; 0 ≤ nums[i] ≤ 400'],
     'complexity': {'time': 'O(n) (local training target)',
                    'space': 'O(1) extra space (rolling variables; a dp array is O(n))',
                    'officialRequirement': 'No official complexity requirement.',
                    'notes': 'n is the number of houses. dp[i] = max(dp[i-1], dp[i-2] + nums[i]). '
                             'Recursion without memoization is exponential.'},
     'acmIoSpec': 'stdin: line 1 is n; line 2 has n integers nums. stdout: one integer, the maximum '
                  'amount.'},
    "constraints": ["1 ≤ n ≤ 100；0 ≤ nums[i] ≤ 400"],
    "complexity": {
        "time": "O(n)（本地训练目标）",
        "space": "O(1) 额外空间（滚动变量；dp 数组为 O(n)）",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为房屋数。dp[i] = max(dp[i-1], dp[i-2] + nums[i])。不带记忆化的递归是指数级。",
        "sourceUrl": "https://leetcode.com/problems/house-robber/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int rob(vector<int>& nums) {\n        return 0; // Maximum amount without robbing two adjacent houses.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数 nums。stdout：一个整数，最大金额。",
    },
    "io": {
        "method": "rob",
        "params": [{"name": "nums", "type": "int[]"}],
        "returns": "int",
    },
}


def brute(nums):
    # Enumerate every subset with no two adjacent indices.
    n, best = len(nums), 0
    for mask in range(1 << n):
        if mask & (mask >> 1) == 0:
            best = max(best, sum(nums[i] for i in range(n) if mask >> i & 1))
    return best


def solve(nums):
    take, skip = 0, 0
    for x in nums:
        take, skip = skip + x, max(take, skip)
    ans = max(take, skip)
    if len(nums) <= 16:
        assert ans == brute(nums)
    return ans


r = rng(198)
tests = [
    {"id": "t1", "input": [1, 2, 3, 1], "sample": True, "expected": 4},
    {"id": "t2", "input": [2, 7, 9, 3, 1], "sample": True, "expected": 12},
    {"id": "single", "input": [7], "sample": False},
    {"id": "two", "input": [3, 9], "sample": False},
    {"id": "all-zero", "input": [0] * 10, "sample": False},
    {"id": "skip-two", "input": [2, 1, 1, 2], "sample": False},
    {"id": "random-small", "input": [r.randint(0, 400) for _ in range(15)], "sample": False},
    {"id": "max-size-100", "input": [r.randint(0, 400) for _ in range(100)], "sample": False},
    {"id": "max-size-100-equal", "input": [400] * 100, "sample": False},
]
write(problem, tests, solve)
