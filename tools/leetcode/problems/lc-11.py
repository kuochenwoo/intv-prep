import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-11",
    "title": "盛最多水的容器",
    "difficulty": "medium",
    "tags": ["贪心", "数组", "双指针"],
    "statement": "给定长度为 n 的非负整数数组 height，第 i 条竖线的两个端点是 (i, 0) 和 (i, height[i])。任选两条竖线与 x 轴构成一个容器（容器不能倾斜），容量为两线间距乘以较短线的高度。返回容器能容纳的最大水量。",
    "constraints": ["2 ≤ n ≤ 100000；0 ≤ height[i] ≤ 10000"],
    "complexity": {
        "time": "O(n)（本地训练目标）",
        "space": "O(1) 额外空间",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为数组长度，输入数组不计入额外空间。双指针从两端出发，每次移动较矮的一侧：较矮一侧作为短板时，向内的任何搭配都不可能更大。O(n^2) 枚举在最大测试上会超时。",
        "sourceUrl": "https://leetcode.com/problems/container-with-most-water/",
    },
    "en": {
        "title": "Container With Most Water",
        "tags": [
            "Greedy",
            "Array",
            "Two Pointers"
        ],
        "statement": "Given a non-negative integer array height of length n, the i-th vertical line has endpoints (i, 0) and (i, height[i]). Any two lines together with the x-axis form a container (which may not be tilted); its capacity is the distance between the lines times the height of the shorter line. Return the maximum amount of water a container can hold.",
        "constraints": [
            "2 ≤ n ≤ 100000; 0 ≤ height[i] ≤ 10000"
        ],
        "complexity": {
            "time": "O(n) (local training target)",
            "space": "O(1) extra space",
            "officialRequirement": "No official complexity requirement.",
            "notes": "n is the array length; the input array does not count toward extra space. Two pointers start at both ends and the shorter side moves each time: when the shorter side is the limiting wall, no pairing further inward with it can be larger. O(n^2) enumeration times out on the largest test."
        },
        "acmIoSpec": "stdin: line 1 is n; line 2 has n integers height. stdout: one integer, the maximum amount of water."
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int maxArea(vector<int>& height) {\n        return 0; // Largest container area.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数 height。stdout：一个整数，最大水量。",
    },
    "io": {
        "method": "maxArea",
        "params": [{"name": "height", "type": "int[]"}],
        "returns": "int",
    },
}


def solve(h):
    if len(h) <= 300:
        return max(min(h[i], h[j]) * (j - i) for i in range(len(h)) for j in range(i + 1, len(h)))
    i, j, best = 0, len(h) - 1, 0
    while i < j:
        best = max(best, min(h[i], h[j]) * (j - i))
        if h[i] < h[j]:
            i += 1
        else:
            j -= 1
    return best


r = rng(11)
tests = [
    {"id": "t1", "input": [1, 8, 6, 2, 5, 4, 8, 3, 7], "sample": True, "expected": 49},
    {"id": "t2", "input": [1, 1], "sample": True, "expected": 1},
    {"id": "zeros", "input": [0, 0, 0], "sample": False},
    {"id": "one-zero", "input": [0, 10000], "sample": False},
    {"id": "increasing", "input": list(range(1, 201)), "sample": False},
    {"id": "peak-in-middle", "input": [1, 2, 10000, 10000, 2, 1], "sample": False},
    {"id": "random-300", "input": [r.randint(0, 10000) for _ in range(300)], "sample": False},
    {"id": "max-size-100000", "input": [r.randint(0, 10000) for _ in range(100000)], "sample": False},
]
write(problem, tests, solve)
