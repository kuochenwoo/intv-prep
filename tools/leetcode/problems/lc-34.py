import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-34",
    "title": "在排序数组中查找元素的第一个和最后一个位置",
    "difficulty": "medium",
    "tags": ["数组", "二分查找"],
    "statement": "给定一个按非递减顺序排列的整数数组 nums（可能有重复元素，也可能为空）和整数 target，返回 target 在数组中第一次和最后一次出现的下标 [start, end]；若 target 不存在，返回 [-1, -1]。",
    "constraints": [
        "0 ≤ nums.length ≤ 10^5",
        "-10^9 ≤ nums[i] ≤ 10^9，nums 非递减",
        "-10^9 ≤ target ≤ 10^9",
    ],
    "complexity": {
        "time": "O(log n)",
        "space": "O(1) 额外空间",
        "officialRequirement": "官方要求 O(log n) 时间复杂度。",
        "notes": "n 为数组长度。分别二分出第一个 ≥ target 和第一个 > target 的位置；找到一个位置后再向两侧线性扩展在大量重复时退化为 O(n)。本地最大测试 n = 10^5（官方上限），但取值压缩到 [0, 99] 以控制题库体积（含大量重复）；另有取值覆盖 ±10^9 的中等测试。本地判题需 O(n) 读入，无法从耗时上区分二分与线性扫描。",
        "sourceUrl": "https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    vector<int> searchRange(vector<int>& nums, int target) {\n        return {-1, -1}; // First and last index of target.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数 nums（n = 0 时为空行）；第三行 target。stdout：一行两个整数 start end。",
    },
    "io": {
        "method": "searchRange",
        "params": [{"name": "nums", "type": "int[]"}, {"name": "target", "type": "int"}],
        "returns": "int[]",
    },
}


def solve(d):
    idx = [i for i, v in enumerate(d["nums"]) if v == d["target"]]
    return [idx[0], idx[-1]] if idx else [-1, -1]


r = rng(34)
wide = sorted(r.randint(-10**9, 10**9) for _ in range(1500)) 
wide[700:760] = [wide[700]] * 60
wide.sort()
big = sorted(r.randint(0, 99) for _ in range(10**5))
tests = [
    {"id": "t1", "input": {"nums": [5, 7, 7, 8, 8, 10], "target": 8}, "sample": True, "expected": [3, 4]},
    {"id": "t2", "input": {"nums": [5, 7, 7, 8, 8, 10], "target": 6}, "sample": True, "expected": [-1, -1]},
    {"id": "t3", "input": {"nums": [], "target": 0}, "sample": True, "expected": [-1, -1]},
    {"id": "single-hit", "input": {"nums": [1], "target": 1}, "sample": False},
    {"id": "single-miss", "input": {"nums": [1], "target": 0}, "sample": False},
    {"id": "all-equal", "input": {"nums": [2] * 50, "target": 2}, "sample": False},
    {"id": "at-ends", "input": {"nums": [3, 3, 4, 5, 9, 9, 9], "target": 9}, "sample": False},
    {"id": "front", "input": {"nums": [3, 3, 4, 5, 9, 9, 9], "target": 3}, "sample": False},
    {"id": "extremes", "input": {"nums": [-10**9, -10**9, 0, 10**9], "target": -10**9}, "sample": False},
    {"id": "above-all", "input": {"nums": [-10**9, -10**9, 0, 10**9], "target": 10**9}, "sample": False},
    {"id": "random-wide-hit", "input": {"nums": wide, "target": wide[730]}, "sample": False},
    {"id": "random-wide-miss", "input": {"nums": wide, "target": wide[100] + 1 if wide[101] != wide[100] + 1 else wide[100] - 1}, "sample": False},
    {"id": "max-size-100000", "input": {"nums": big, "target": 50}, "sample": False},
    {"id": "all-equal-20000", "input": {"nums": [7] * 20000, "target": 7}, "sample": False},
]
write(problem, tests, solve)
