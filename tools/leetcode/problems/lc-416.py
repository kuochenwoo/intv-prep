import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-416",
    "title": "分割等和子集",
    "difficulty": "medium",
    "tags": ["数组", "动态规划"],
    "statement": "给定只含正整数的非空数组 nums，判断能否把它分成两个子集，使两个子集的元素和相等。",
    "en": {
        "title": "Partition Equal Subset Sum",
        "tags": [
            "Array",
            "Dynamic Programming"
        ],
        "statement": "Given a non-empty array nums containing only positive integers, determine whether it can be partitioned into two subsets whose element sums are equal.",
        "constraints": [
            "1 ≤ nums.length ≤ 200; 1 ≤ nums[i] ≤ 100"
        ],
        "complexity": {
            "time": "O(n · S) (local training target), where S is half of the total sum",
            "space": "O(S) extra space (1D 0/1 knapsack)",
            "officialRequirement": "No official complexity requirement.",
            "notes": "n is the array length, S = sum(nums) / 2 ≤ 10000. If the total is odd, the answer is false; otherwise use 0/1 knapsack to check whether S is reachable. The 1D array must be updated in reverse order; bitset shifting also works."
        },
        "acmIoSpec": "stdin: line 1 is n; line 2 has n positive integers nums. stdout: true or false."
    },
    "constraints": ["1 ≤ nums.length ≤ 200；1 ≤ nums[i] ≤ 100"],
    "complexity": {
        "time": "O(n · S)（本地训练目标），S 为元素总和的一半",
        "space": "O(S) 额外空间（一维 0/1 背包）",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为数组长度，S = sum(nums) / 2 ≤ 10000。总和为奇数直接为 false；否则 0/1 背包判断能否凑出 S，一维数组需倒序更新，也可用 bitset 移位。",
        "sourceUrl": "https://leetcode.com/problems/partition-equal-subset-sum/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    bool canPartition(vector<int>& nums) {\n        return false; // Whether nums splits into two equal-sum subsets.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个正整数 nums。stdout：true 或 false。",
    },
    "io": {
        "method": "canPartition",
        "params": [{"name": "nums", "type": "int[]"}],
        "returns": "bool",
    },
}


def solve(a):
    # Set of reachable sums as a Python big-int bitmask.
    total = sum(a)
    if total % 2:
        return False
    reach = 1
    for x in a:
        reach |= reach << x
    return bool(reach >> (total // 2) & 1)


def brute(a):
    total = sum(a)
    return any(2 * sum(x for i, x in enumerate(a) if m >> i & 1) == total for m in range(1 << len(a)))


r = rng(416)
tests = [
    {"id": "t1", "input": [1, 5, 11, 5], "sample": True, "expected": True},
    {"id": "t2", "input": [1, 2, 3, 5], "sample": True, "expected": False},
    {"id": "single", "input": [7], "sample": False},
    {"id": "pair-equal", "input": [100, 100], "sample": False},
    {"id": "even-sum-impossible", "input": [2, 2, 3, 5], "sample": False},
    {"id": "random-12", "input": [r.randint(1, 30) for _ in range(12)], "sample": False},
    {"id": "random-14", "input": [r.randint(1, 100) for _ in range(14)], "sample": False},
    {"id": "max-size-200", "input": [r.randint(1, 100) for _ in range(200)], "sample": False},
    {"id": "max-size-200-all-100", "input": [100] * 200, "sample": False},
    {"id": "max-size-200-impossible", "input": [100] * 199 + [98], "sample": False},  # even total, odd half
    {"id": "max-size-200-odd-heavy", "input": [99] * 198 + [1, 3], "sample": False},
]
for t in tests:
    if len(t["input"]) <= 14:
        assert brute(t["input"]) == solve(t["input"]), t["id"]
write(problem, tests, solve)
