import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-55",
    "title": "跳跃游戏",
    "difficulty": "medium",
    "tags": ["贪心", "数组", "动态规划"],
    "statement": "给定非负整数数组 nums，你从下标 0 出发，nums[i] 表示站在下标 i 时一次最多能向右跳的距离（可以跳 0 到 nums[i] 步中任意步）。判断能否到达最后一个下标。",
    "constraints": ["1 ≤ nums.length ≤ 10000；0 ≤ nums[i] ≤ 100000"],
    "complexity": {
        "time": "O(n)（本地训练目标）",
        "space": "O(1) 额外空间",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为数组长度。贪心维护当前能到达的最远下标；对每个位置枚举所有落点的 DP 最坏为 O(n·max(nums))。",
        "sourceUrl": "https://leetcode.com/problems/jump-game/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    bool canJump(vector<int>& nums) {\n        return false; // Whether the last index is reachable from index 0.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数 nums。stdout：true 或 false。",
    },
    "io": {
        "method": "canJump",
        "params": [{"name": "nums", "type": "int[]"}],
        "returns": "bool",
    },
}


def brute(nums):
    n = len(nums)
    ok = [False] * n
    ok[0] = True
    for i in range(n):
        if ok[i]:
            for j in range(i + 1, min(n, i + nums[i] + 1)):
                ok[j] = True
    return ok[-1]


def solve(nums):
    # Interval sweep with a difference array of reachable segments.
    n = len(nums)
    diff = [0] * (n + 1)
    diff[0], diff[1] = 1, -1
    cover = 0
    for i in range(n):
        cover += diff[i]
        if cover > 0:
            diff[i + 1] += 1
            diff[min(n, i + nums[i] + 1)] -= 1
        elif i:
            return False
    return True


def solve_checked(nums):
    ans = solve(nums)
    if len(nums) <= 500:
        assert ans == brute(nums)
    return ans


r = rng(55)
trap = [1] * 10000
trap[9000] = 0
tests = [
    {"id": "t1", "input": [2, 3, 1, 1, 4], "sample": True, "expected": True},
    {"id": "t2", "input": [3, 2, 1, 0, 4], "sample": True, "expected": False},
    {"id": "single-zero", "input": [0], "sample": False},
    {"id": "stuck-at-start", "input": [0, 1], "sample": False},
    {"id": "zero-crossed", "input": [2, 0, 0], "sample": False},
    {"id": "zero-blocks", "input": [1, 0, 1], "sample": False},
    {"id": "random-small-0", "input": [r.randint(0, 3) for _ in range(40)], "sample": False},
    {"id": "random-small-1", "input": [r.randint(0, 2) for _ in range(300)], "sample": False},
    {"id": "max-size-10000-ones", "input": [1] * 10000, "sample": False},
    {"id": "max-size-10000-trap", "input": trap, "sample": False},
    {"id": "max-size-10000-big", "input": [100000 - i for i in range(10000)], "sample": False},
]
write(problem, tests, solve_checked)
