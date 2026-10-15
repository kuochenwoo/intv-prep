import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-300",
    "title": "最长递增子序列",
    "difficulty": "medium",
    "tags": ["数组", "二分查找", "动态规划"],
    "statement": "给定整数数组 nums，求其中最长的严格递增子序列的长度。子序列由原数组删去若干（可以为零个）元素、且不改变剩余元素相对顺序得到，不要求连续。",
    "constraints": ["1 ≤ nums.length ≤ 2500；-10^4 ≤ nums[i] ≤ 10^4"],
    "complexity": {
        "time": "O(n log n)（本地训练目标，贪心 + 二分）；O(n^2) 动态规划在本地数据规模下也可通过",
        "space": "O(n) 额外空间（不含输入数组）",
        "officialRequirement": "官方 follow-up 要求设计 O(n log n) 时间的算法；题目本身未强制。",
        "notes": "n 为数组长度。tails[k] 记录长度为 k+1 的递增子序列的最小结尾，对每个元素二分替换。最大测试为官方上限 n = 2500，此规模下 O(n^2) 仍能通过，限时无法区分两种复杂度。",
        "sourceUrl": "https://leetcode.com/problems/longest-increasing-subsequence/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int lengthOfLIS(vector<int>& nums) {\n        return 0; // Length of the longest strictly increasing subsequence.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数 nums。stdout：一个整数，最长严格递增子序列的长度。",
    },
    "io": {
        "method": "lengthOfLIS",
        "params": [{"name": "nums", "type": "int[]"}],
        "returns": "int",
    },
}


def brute(a):
    # O(n^2) DP, independent of the patience-sorting C++ solution.
    best = [1] * len(a)
    for i in range(len(a)):
        for j in range(i):
            if a[j] < a[i] and best[j] + 1 > best[i]:
                best[i] = best[j] + 1
    return max(best)


def solve(a):
    return brute(a)


r = rng(300)
tests = [
    {"id": "t1", "input": [10, 9, 2, 5, 3, 7, 101, 18], "sample": True, "expected": 4},
    {"id": "t2", "input": [0, 1, 0, 3, 2, 3], "sample": True, "expected": 4},
    {"id": "t3", "input": [7, 7, 7, 7, 7, 7, 7], "sample": True, "expected": 1},
    {"id": "single", "input": [-5], "sample": False},
    {"id": "decreasing", "input": list(range(100, 0, -1)), "sample": False},
    {"id": "negatives", "input": [-3, -10, -2, -2, -1, -8, 0], "sample": False},
    {"id": "random-300", "input": [r.randint(-50, 50) for _ in range(300)], "sample": False},
    {"id": "max-size-2500", "input": [r.randint(-10000, 10000) for _ in range(2500)], "sample": False},
    {"id": "max-size-2500-increasing", "input": list(range(-10000, -10000 + 2500 * 8, 8)), "sample": False},
    {"id": "max-size-2500-equal", "input": [42] * 2500, "sample": False},
]
write(problem, tests, solve)
