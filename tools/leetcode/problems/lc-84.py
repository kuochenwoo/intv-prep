import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-84",
    "title": "柱状图中最大的矩形",
    "difficulty": "hard",
    "tags": ["栈", "数组", "单调栈"],
    "statement": "给定 n 个非负整数 heights，表示柱状图中相邻排列、宽度均为 1 的柱子高度。求在该柱状图中能勾勒出的矩形的最大面积（矩形必须完全落在柱子内部）。",
    "constraints": [
        "1 ≤ heights.length ≤ 10^5",
        "0 ≤ heights[i] ≤ 10^4",
    ],
    "complexity": {
        "time": "O(n)",
        "space": "O(n) 额外空间（单调栈或左右边界数组）",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为柱子数。对每根柱子求左右两侧第一个更矮柱子的位置，以它为高的最大矩形宽度即两者之间的距离；单调递增栈可一次扫描完成。面积最大为 10^4 × 10^5 = 10^9，仍在 int 范围内。枚举左右端点的 O(n²) 做法会超时；分治在有序输入上退化为 O(n²)。本地最大测试为 n = 10^5（官方上限），高度取全 1 以控制题库体积；另有 n = 30000 的非递减（阶梯上升）测试。",
        "sourceUrl": "https://leetcode.com/problems/largest-rectangle-in-histogram/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int largestRectangleArea(vector<int>& heights) {\n        return 0; // Largest rectangle area.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数 heights。stdout：一个整数，最大矩形面积。",
    },
    "io": {
        "method": "largestRectangleArea",
        "params": [{"name": "heights", "type": "int[]"}],
        "returns": "int",
    },
}


def solve(h):
    # Process bars from tallest to shortest, merging neighbouring active
    # intervals with union-find; the current bar is the minimum of its interval.
    n = len(h)
    parent = list(range(n))
    left = list(range(n))
    right = list(range(n))
    active = [False] * n

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    best = 0
    for i in sorted(range(n), key=lambda i: -h[i]):
        active[i] = True
        root = i
        for j in (i - 1, i + 1):
            if 0 <= j < n and active[j]:
                a, b = find(root), find(j)
                parent[b] = a
                left[a] = min(left[a], left[b])
                right[a] = max(right[a], right[b])
                root = a
        root = find(root)
        best = max(best, h[i] * (right[root] - left[root] + 1))
    return best


def brute(h):
    best = 0
    for i in range(len(h)):
        low = h[i]
        for j in range(i, len(h)):
            low = min(low, h[j])
            best = max(best, low * (j - i + 1))
    return best


r = rng(84)
tests = [
    {"id": "t1", "input": [2, 1, 5, 6, 2, 3], "sample": True, "expected": 10},
    {"id": "t2", "input": [2, 4], "sample": True, "expected": 4},
    {"id": "single-zero", "input": [0], "sample": False},
    {"id": "single-max", "input": [10000], "sample": False},
    {"id": "zeros-split", "input": [3, 3, 0, 2, 2, 2, 0, 1], "sample": False},
    {"id": "increasing", "input": [1, 2, 3, 4, 5], "sample": False},
    {"id": "decreasing", "input": [5, 4, 3, 2, 1], "sample": False},
    {"id": "equal-heights", "input": [6, 6, 6, 6], "sample": False},
    {"id": "valley", "input": [6, 7, 1, 7, 6], "sample": False},
    {"id": "random-small-0", "input": [r.randint(0, 10) for _ in range(60)], "sample": False},
    {"id": "random-small-1", "input": [r.randint(0, 10000) for _ in range(300)], "sample": False},
    {"id": "random-3000", "input": [r.randint(0, 10000) for _ in range(3000)], "sample": False},
    {"id": "worst-case-ascending-30000", "input": [i // 3 for i in range(30000)], "sample": False},
    {"id": "max-size-100000-all-ones", "input": [1] * 10**5, "sample": False},
    {"id": "tall-10000x10000", "input": [10000] * 10000, "sample": False},
]
for t in tests[:11]:
    assert brute(t["input"]) == solve(t["input"]), t["id"]
write(problem, tests, solve)
