import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-35",
    "title": "搜索插入位置",
    "difficulty": "easy",
    "tags": ["数组", "二分查找"],
    "statement": "给定一个元素互不相同、严格升序的整数数组 nums 和整数 target。若 target 在数组中，返回它的下标；否则返回把它按顺序插入后所处的下标（即第一个大于 target 的元素位置，若没有则为 n）。",
    "constraints": [
        "1 ≤ nums.length ≤ 10^4",
        "-10^4 ≤ nums[i] ≤ 10^4，nums 元素互不相同且升序",
        "-10^4 ≤ target ≤ 10^4",
    ],
    "complexity": {
        "time": "O(log n)",
        "space": "O(1) 额外空间",
        "officialRequirement": "官方要求 O(log n) 时间复杂度。",
        "notes": "n 为数组长度。答案就是第一个 ≥ target 的位置（lower_bound）。本地判题需 O(n) 读入，无法从耗时上区分二分与线性扫描。",
        "sourceUrl": "https://leetcode.com/problems/search-insert-position/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int searchInsert(vector<int>& nums, int target) {\n        return 0; // Index of target or its insertion point.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数 nums；第三行 target。stdout：一个整数，下标或插入位置。",
    },
    "io": {
        "method": "searchInsert",
        "params": [{"name": "nums", "type": "int[]"}, {"name": "target", "type": "int"}],
        "returns": "int",
    },
}


def solve(d):
    return sum(1 for v in d["nums"] if v < d["target"])


r = rng(35)
big = sorted(r.sample(range(-10**4, 10**4 + 1), 10**4))
mid = sorted(r.sample(range(-500, 501), 300))
missing = next(v for v in range(big[6000], 10**4) if v not in set(big))
tests = [
    {"id": "t1", "input": {"nums": [1, 3, 5, 6], "target": 5}, "sample": True, "expected": 2},
    {"id": "t2", "input": {"nums": [1, 3, 5, 6], "target": 2}, "sample": True, "expected": 1},
    {"id": "t3", "input": {"nums": [1, 3, 5, 6], "target": 7}, "sample": True, "expected": 4},
    {"id": "front", "input": {"nums": [1, 3, 5, 6], "target": 0}, "sample": False},
    {"id": "single-equal", "input": {"nums": [4], "target": 4}, "sample": False},
    {"id": "single-less", "input": {"nums": [4], "target": -10000}, "sample": False},
    {"id": "single-greater", "input": {"nums": [4], "target": 10000}, "sample": False},
    {"id": "negatives", "input": {"nums": [-9, -4, -2, 0, 3], "target": -3}, "sample": False},
    {"id": "random-hit", "input": {"nums": mid, "target": mid[123]}, "sample": False},
    {"id": "random-gap", "input": {"nums": mid, "target": next(v for v in range(mid[200], 501) if v not in mid)}, "sample": False},
    {"id": "max-size-10000", "input": {"nums": big, "target": missing}, "sample": False},
    {"id": "max-size-10000-end", "input": {"nums": list(range(-10**4, 0)), "target": 10**4}, "sample": False},
]
write(problem, tests, solve)
