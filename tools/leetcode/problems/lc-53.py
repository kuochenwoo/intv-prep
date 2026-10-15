import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-53",
    "title": "最大子数组和",
    "difficulty": "medium",
    "tags": ["数组", "分治", "动态规划"],
    "statement": "给定整数数组 nums，找出和最大的连续非空子数组，返回这个最大和。",
    "en": {
        "title": "Maximum Subarray",
        "tags": [
            "Array",
            "Divide and Conquer",
            "Dynamic Programming"
        ],
        "statement": "Given an integer array nums, find the contiguous non-empty subarray with the largest sum and return that sum.",
        "constraints": [
            "1 ≤ nums.length ≤ 100000; -10^4 ≤ nums[i] ≤ 10^4"
        ],
        "complexity": {
            "time": "O(n) (local training target)",
            "space": "O(1) extra space (Kadane); divide and conquer uses an O(log n) recursion stack",
            "officialRequirement": "No official complexity requirement; the follow-up suggests also trying a divide-and-conquer solution beyond the O(n) one (no complexity specified).",
            "notes": "n is the array length. Kadane: the best sum ending at i = max(nums[i], result at the previous position + nums[i]). Divide and conquer is O(n log n) (or O(n) when merging segment info). For an all-negative array the answer is the largest single element. The maximum sum does not exceed 10^9, within 32-bit integer range. The largest local test has 60000 elements (official limit 100000, reduced to keep the problem bank small)."
        },
        "acmIoSpec": "stdin: first line n; second line n integers. stdout: one integer, the maximum subarray sum."
    },
    "constraints": ["1 ≤ nums.length ≤ 100000；-10^4 ≤ nums[i] ≤ 10^4"],
    "complexity": {
        "time": "O(n)（本地训练目标）",
        "space": "O(1) 额外空间（Kadane）；分治为 O(log n) 递归栈",
        "officialRequirement": "官方未强制复杂度要求；follow-up 建议在 O(n) 解法之外再尝试分治解法（未规定复杂度）。",
        "notes": "n 为数组长度。Kadane：以 i 结尾的最大和 = max(nums[i], 前一位置结果 + nums[i])。分治 O(n log n)（或合并区间信息的 O(n)）。全负数组时答案为最大的单个元素。最大和不超过 10^9，在 32 位整数范围内。本地最大测试为 60000 个元素（官方上限 100000，为控制题库体积而缩小）。",
        "sourceUrl": "https://leetcode.com/problems/maximum-subarray/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int maxSubArray(vector<int>& nums) {\n        return 0; // Largest subarray sum.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数。stdout：一个整数，最大子数组和。",
    },
    "io": {
        "method": "maxSubArray",
        "params": [{"name": "nums", "type": "int[]"}],
        "returns": "int",
    },
}


def solve(nums):
    if len(nums) <= 200:
        return max(sum(nums[i:j]) for i in range(len(nums)) for j in range(i + 1, len(nums) + 1))
    # Prefix sum minus the smallest earlier prefix.
    best, pre, low = None, 0, 0
    for x in nums:
        pre += x
        cand = pre - low
        best = cand if best is None else max(best, cand)
        low = min(low, pre)
    return best


r = rng(53)
tests = [
    {"id": "t1", "input": [-2, 1, -3, 4, -1, 2, 1, -5, 4], "sample": True, "expected": 6},
    {"id": "t2", "input": [1], "sample": True, "expected": 1},
    {"id": "t3", "input": [5, 4, -1, 7, 8], "sample": True, "expected": 23},
    {"id": "single-negative", "input": [-10000], "sample": False},
    {"id": "all-negative", "input": [-5, -2, -8, -3, -2], "sample": False},
    {"id": "zeros-and-negatives", "input": [-1, 0, -2, 0, -3], "sample": False},
    {"id": "random-200", "input": [r.randint(-100, 100) for _ in range(200)], "sample": False},
    {"id": "worst-case-60000", "input": [r.randint(-10000, 10000) for _ in range(60000)], "sample": False},
    {"id": "worst-case-all-max-10000", "input": [10000] * 10000, "sample": False},
]
write(problem, tests, solve)
