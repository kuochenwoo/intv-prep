import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-33",
    "title": "搜索旋转排序数组",
    "difficulty": "medium",
    "tags": ["数组", "二分查找"],
    "statement": "一个元素互不相同的升序数组在某个未知下标 k（0 ≤ k < n）处被旋转，变成 [nums[k], …, nums[n-1], nums[0], …, nums[k-1]]。给定旋转后的数组 nums 和整数 target，返回 target 的下标；不存在则返回 -1。",
    "constraints": [
        "1 ≤ nums.length ≤ 5000",
        "-10^4 ≤ nums[i] ≤ 10^4，元素互不相同",
        "nums 是某个升序数组旋转（可能旋转 0 次）后的结果",
        "-10^4 ≤ target ≤ 10^4",
    ],
    "complexity": {
        "time": "O(log n)",
        "space": "O(1) 额外空间",
        "officialRequirement": "官方要求 O(log n) 时间复杂度。",
        "notes": "n 为数组长度。二分时 [lo, mid] 与 [mid, hi] 至少一半有序，判断 target 是否落在有序的那一半；也可先二分找旋转点再在对应段二分。本地判题需 O(n) 读入，无法从耗时上区分二分与线性扫描。",
        "sourceUrl": "https://leetcode.com/problems/search-in-rotated-sorted-array/",
    },
    "en": {
        "title": "Search in Rotated Sorted Array",
        "tags": [
            "Array",
            "Binary Search"
        ],
        "statement": "An ascending array of distinct elements has been rotated at some unknown index k (0 ≤ k < n), becoming [nums[k], …, nums[n-1], nums[0], …, nums[k-1]]. Given the rotated array nums and an integer target, return the index of target, or -1 if it is not present.",
        "constraints": [
            "1 ≤ nums.length ≤ 5000",
            "-10^4 ≤ nums[i] ≤ 10^4, all elements are distinct",
            "nums is an ascending array after rotation (possibly rotated 0 times)",
            "-10^4 ≤ target ≤ 10^4"
        ],
        "complexity": {
            "time": "O(log n)",
            "space": "O(1) extra space",
            "officialRequirement": "Official requirement: O(log n) time complexity.",
            "notes": "n is the array length. During binary search, at least one of [lo, mid] and [mid, hi] is sorted; check whether target falls within the sorted half. Alternatively, binary search for the rotation point first, then binary search in the corresponding segment. The local judge needs O(n) input reading, so timing cannot distinguish binary search from a linear scan."
        },
        "acmIoSpec": "stdin: line 1 is n; line 2 has n integers nums; line 3 is target. stdout: one integer, the index of target or -1."
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int search(vector<int>& nums, int target) {\n        return -1; // Index of target, or -1.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数 nums；第三行 target。stdout：一个整数，target 的下标或 -1。",
    },
    "io": {
        "method": "search",
        "params": [{"name": "nums", "type": "int[]"}, {"name": "target", "type": "int"}],
        "returns": "int",
    },
}


def solve(d):
    return d["nums"].index(d["target"]) if d["target"] in d["nums"] else -1


def rotated(r, n, lo, hi, k=None):
    a = sorted(r.sample(range(lo, hi + 1), n))
    k = r.randrange(n) if k is None else k
    return a[k:] + a[:k]


def absent(r, a, lo, hi):
    s = set(a)
    while True:
        v = r.randint(lo, hi)
        if v not in s:
            return v


r = rng(33)
mid = rotated(r, 400, -10**4, 10**4)
big = rotated(r, 5000, -10**4, 10**4, 3217)
big0 = rotated(r, 5000, -10**4, 10**4, 0)
tests = [
    {"id": "t1", "input": {"nums": [4, 5, 6, 7, 0, 1, 2], "target": 0}, "sample": True, "expected": 4},
    {"id": "t2", "input": {"nums": [4, 5, 6, 7, 0, 1, 2], "target": 3}, "sample": True, "expected": -1},
    {"id": "t3", "input": {"nums": [1], "target": 0}, "sample": True, "expected": -1},
    {"id": "single-hit", "input": {"nums": [1], "target": 1}, "sample": False},
    {"id": "two-rotated", "input": {"nums": [3, 1], "target": 1}, "sample": False},
    {"id": "two-rotated-first", "input": {"nums": [3, 1], "target": 3}, "sample": False},
    {"id": "not-rotated", "input": {"nums": [1, 3, 5, 7, 9], "target": 7}, "sample": False},
    {"id": "pivot-last", "input": {"nums": [2, 3, 4, 5, 1], "target": 1}, "sample": False},
    {"id": "pivot-first-half", "input": {"nums": [6, 7, 1, 2, 3, 4, 5], "target": 7}, "sample": False},
    {"id": "left-half-miss", "input": {"nums": [5, 6, 7, 8, 9, 1, 2], "target": 4}, "sample": False},
    {"id": "random-hit", "input": {"nums": mid, "target": mid[17]}, "sample": False},
    {"id": "random-miss", "input": {"nums": mid, "target": absent(r, mid, -10**4, 10**4)}, "sample": False},
    {"id": "max-size-5000", "input": {"nums": big, "target": big[4999]}, "sample": False},
    {"id": "max-size-5000-unrotated-miss", "input": {"nums": big0, "target": absent(r, big0, -10**4, 10**4)}, "sample": False},
]
write(problem, tests, solve)
