import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-4",
    "title": "寻找两个正序数组的中位数",
    "difficulty": "hard",
    "tags": ["数组", "二分查找", "分治"],
    "statement": "给定两个分别按非递减顺序排列的整数数组 nums1（长度 m）和 nums2（长度 n），其中一个可以为空。求把两个数组合并后整体的中位数：总长度为奇数时取正中间的数，为偶数时取中间两个数的平均值，结果为浮点数。",
    "constraints": [
        "nums1.length == m，nums2.length == n",
        "0 ≤ m ≤ 1000，0 ≤ n ≤ 1000，1 ≤ m + n ≤ 2000",
        "-10^6 ≤ nums1[i], nums2[i] ≤ 10^6",
    ],
    "complexity": {
        "time": "O(log(min(m, n)))（在较短数组上二分划分位置）；求第 k 小的递归删减为 O(log(m + n))",
        "space": "O(1) 额外空间",
        "officialRequirement": "官方要求整体运行时间复杂度为 O(log(m + n))。",
        "notes": "m、n 为两数组长度。归并到中点的 O(m + n) 做法能通过本地测试，但不满足官方要求；本地规模（m + n ≤ 2000）和 O(m + n) 的读入使耗时无法区分二者。输出与期望值按 1e-5 相对误差比较。",
        "sourceUrl": "https://leetcode.com/problems/median-of-two-sorted-arrays/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    double findMedianSortedArrays(vector<int>& nums1, vector<int>& nums2) {\n        return 0.0; // Median of the merged arrays.\n    }\n};",
        "acmIoSpec": "stdin：第一行 m；第二行 m 个整数 nums1（m = 0 时为空行）；第三行 n；第四行 n 个整数 nums2（n = 0 时为空行）。stdout：一个浮点数，中位数（按 1e-5 相对误差比较）。",
    },
    "io": {
        "method": "findMedianSortedArrays",
        "params": [{"name": "nums1", "type": "int[]"}, {"name": "nums2", "type": "int[]"}],
        "returns": "double",
    },
}


def solve(d):
    a = sorted(d["nums1"] + d["nums2"])
    k = len(a)
    return float(a[k // 2]) if k % 2 else (a[k // 2 - 1] + a[k // 2]) / 2


def arr(r, n, lo=-10**6, hi=10**6):
    return sorted(r.randint(lo, hi) for _ in range(n))


r = rng(4)
tests = [
    {"id": "t1", "input": {"nums1": [1, 3], "nums2": [2]}, "sample": True, "expected": 2.0},
    {"id": "t2", "input": {"nums1": [1, 2], "nums2": [3, 4]}, "sample": True, "expected": 2.5},
    {"id": "first-empty", "input": {"nums1": [], "nums2": [1]}, "sample": False},
    {"id": "second-empty-even", "input": {"nums1": [2, 3], "nums2": []}, "sample": False},
    {"id": "both-single", "input": {"nums1": [-1000000], "nums2": [1000000]}, "sample": False},
    {"id": "half-value", "input": {"nums1": [1], "nums2": [2]}, "sample": False},
    {"id": "negative-half", "input": {"nums1": [-5, -3], "nums2": [-2, 0]}, "sample": False},
    {"id": "disjoint-low-high", "input": {"nums1": [1, 2, 3, 4, 5], "nums2": [100, 200]}, "sample": False},
    {"id": "disjoint-high-low", "input": {"nums1": [100, 200, 300], "nums2": [1, 2, 3, 4]}, "sample": False},
    {"id": "all-equal", "input": {"nums1": [7] * 6, "nums2": [7] * 9}, "sample": False},
    {"id": "duplicates", "input": {"nums1": [1, 1, 3, 3], "nums2": [1, 1, 3, 3]}, "sample": False},
    {"id": "short-vs-long", "input": {"nums1": [500], "nums2": arr(r, 30, 0, 1000)}, "sample": False},
    {"id": "random-odd", "input": {"nums1": arr(r, 37), "nums2": arr(r, 64)}, "sample": False},
    {"id": "random-even", "input": {"nums1": arr(r, 120), "nums2": arr(r, 80)}, "sample": False},
    {"id": "max-size-1000-0", "input": {"nums1": arr(r, 1000), "nums2": []}, "sample": False},
    {"id": "max-size-1000-1000", "input": {"nums1": arr(r, 1000), "nums2": arr(r, 1000)}, "sample": False},
]
write(problem, tests, solve)
