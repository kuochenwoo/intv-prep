import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-153",
    "title": "寻找旋转排序数组中的最小值",
    "difficulty": "medium",
    "tags": ["数组", "二分查找"],
    "statement": "一个元素互不相同、长度为 n 的升序数组被旋转了 1 到 n 次（每次把末尾元素移到开头），得到数组 nums。返回 nums 中的最小元素。",
    "constraints": [
        "1 ≤ n ≤ 5000",
        "-5000 ≤ nums[i] ≤ 5000，元素互不相同",
        "nums 是某个升序数组旋转 1 到 n 次后的结果",
    ],
    "complexity": {
        "time": "O(log n)",
        "space": "O(1) 额外空间",
        "officialRequirement": "官方要求 O(log n) 时间复杂度。",
        "notes": "n 为数组长度。比较 nums[mid] 与 nums[hi]：nums[mid] > nums[hi] 说明最小值在右半边，否则在 [lo, mid]。旋转 n 次等价于未旋转。本地判题需 O(n) 读入，无法从耗时上区分二分与线性扫描。",
        "sourceUrl": "https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int findMin(vector<int>& nums) {\n        return 0; // Minimum element.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数 nums。stdout：一个整数，最小值。",
    },
    "io": {
        "method": "findMin",
        "params": [{"name": "nums", "type": "int[]"}],
        "returns": "int",
    },
}


def solve(nums):
    return min(nums)


def rotated(r, n, k=None):
    a = sorted(r.sample(range(-5000, 5001), n))
    k = r.randrange(n) if k is None else k
    return a[k:] + a[:k]


r = rng(153)
tests = [
    {"id": "t1", "input": [3, 4, 5, 1, 2], "sample": True, "expected": 1},
    {"id": "t2", "input": [4, 5, 6, 7, 0, 1, 2], "sample": True, "expected": 0},
    {"id": "t3", "input": [11, 13, 15, 17], "sample": True, "expected": 11},
    {"id": "single", "input": [-5000], "sample": False},
    {"id": "two-rotated", "input": [2, 1], "sample": False},
    {"id": "two-sorted", "input": [1, 2], "sample": False},
    {"id": "min-last", "input": [2, 3, 4, 5, -1], "sample": False},
    {"id": "min-second", "input": [5000, -5000, -10, 0, 7], "sample": False},
    {"id": "random-300", "input": rotated(r, 300), "sample": False},
    {"id": "max-size-5000", "input": rotated(r, 5000, 4321), "sample": False},
    {"id": "max-size-5000-unrotated", "input": rotated(r, 5000, 0), "sample": False},
]
write(problem, tests, solve)
