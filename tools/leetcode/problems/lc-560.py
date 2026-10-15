import sys, pathlib
from collections import defaultdict
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-560",
    "title": "和为 K 的子数组",
    "difficulty": "medium",
    "tags": ["数组", "哈希表", "前缀和"],
    "statement": "给定整数数组 nums 和整数 k，统计和恰好等于 k 的子数组个数。子数组是数组中连续、非空的一段。",
    "constraints": ["1 ≤ nums.length ≤ 20000；-1000 ≤ nums[i] ≤ 1000；-10^7 ≤ k ≤ 10^7"],
    "complexity": {
        "time": "O(n) 平均时间（本地训练目标）",
        "space": "O(n) 额外空间（前缀和计数哈希表）",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为数组长度。子数组 (i, j] 的和为 pre[j] - pre[i]，遍历时在哈希表中查询 pre[j] - k 出现的次数；哈希为平均 O(1)。数组含负数，不能用滑动窗口。全 0 且 k = 0 时答案达 n(n+1)/2 ≈ 2×10^8，仍在 32 位整数范围内。",
        "sourceUrl": "https://leetcode.com/problems/subarray-sum-equals-k/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int subarraySum(vector<int>& nums, int k) {\n        return 0; // Number of subarrays summing to k.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n 和 k；第二行 n 个整数 nums。stdout：一个整数，和为 k 的子数组个数。",
    },
    "io": {
        "method": "subarraySum",
        "params": [{"name": "nums", "type": "int[]"}, {"name": "k", "type": "int"}],
        "returns": "int",
        "stdin": ["nums.length k", "nums"],
    },
}


def solve(d):
    nums, k = d["nums"], d["k"]
    if len(nums) <= 300:
        n = len(nums)
        return sum(1 for i in range(n) for j in range(i + 1, n + 1) if sum(nums[i:j]) == k)
    seen = defaultdict(int)
    seen[0] = 1
    pre = total = 0
    for x in nums:
        pre += x
        total += seen[pre - k]
        seen[pre] += 1
    return total


r = rng(560)
small = [r.randint(-3, 3) for _ in range(20000)]
tests = [
    {"id": "t1", "input": {"nums": [1, 1, 1], "k": 2}, "sample": True, "expected": 2},
    {"id": "t2", "input": {"nums": [1, 2, 3], "k": 3}, "sample": True, "expected": 2},
    {"id": "single-hit", "input": {"nums": [5], "k": 5}, "sample": False},
    {"id": "single-miss", "input": {"nums": [5], "k": -5}, "sample": False},
    {"id": "negatives", "input": {"nums": [1, -1, 0, 1, -1, 2, -2], "k": 0}, "sample": False},
    {"id": "k-out-of-reach", "input": {"nums": [1000] * 50, "k": 10000000}, "sample": False},
    {"id": "random-300", "input": {"nums": [r.randint(-10, 10) for _ in range(300)], "k": 7}, "sample": False},
    {"id": "max-size-20000", "input": {"nums": [r.randint(-1000, 1000) for _ in range(20000)], "k": 1500}, "sample": False},
    {"id": "worst-case-all-zero", "input": {"nums": [0] * 20000, "k": 0}, "sample": False},
    {"id": "worst-case-small-values", "input": {"nums": small, "k": -2}, "sample": False},
]
write(problem, tests, solve)
