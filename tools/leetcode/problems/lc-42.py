import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-42",
    "title": "接雨水",
    "difficulty": "hard",
    "tags": ["数组", "双指针", "单调栈", "动态规划"],
    "statement": "给定 n 个非负整数表示宽度为 1 的柱子高度，计算下雨后这些柱子之间最多能积存多少单位的雨水。",
    "constraints": ["1 ≤ n ≤ 20000；0 ≤ height[i] ≤ 100000"],
    "complexity": {
        "time": "O(n)（本地训练目标）",
        "space": "双指针 O(1) 额外空间；前后缀数组或单调栈 O(n) 亦可",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为柱子数，输入数组不计入额外空间。每个位置的水量为 min(左侧最高, 右侧最高) - 自身高度；双指针每次移动较矮一侧。",
        "sourceUrl": "https://leetcode.com/problems/trapping-rain-water/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int trap(vector<int>& height) {\n        return 0; // Total trapped water.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数 height。stdout：一个整数，积水总量。",
    },
    "io": {
        "method": "trap",
        "params": [{"name": "height", "type": "int[]"}],
        "returns": "int",
    },
}


def solve(h):
    # Prefix/suffix maxima: independent of the two-pointer C++ solution.
    n = len(h)
    left, right = [0] * n, [0] * n
    for i in range(n):
        left[i] = max(h[i], left[i - 1] if i else 0)
    for i in reversed(range(n)):
        right[i] = max(h[i], right[i + 1] if i + 1 < n else 0)
    return sum(min(left[i], right[i]) - h[i] for i in range(n))


r = rng(42)
tests = [
    {"id": "t1", "input": [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], "sample": True, "expected": 6},
    {"id": "t2", "input": [4, 2, 0, 3, 2, 5], "sample": True, "expected": 9},
    {"id": "single", "input": [5], "sample": False},
    {"id": "monotonic", "input": list(range(50)), "sample": False},
    {"id": "valley", "input": [5, 0, 0, 0, 5], "sample": False},
    {"id": "random-200", "input": [r.randint(0, 50) for _ in range(200)], "sample": False},
    {"id": "max-size-20000", "input": [r.randint(0, 100000) for _ in range(20000)], "sample": False},
]
write(problem, tests, solve)
