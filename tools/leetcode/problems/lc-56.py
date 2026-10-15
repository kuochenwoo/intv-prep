import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-56",
    "title": "合并区间",
    "difficulty": "medium",
    "tags": ["数组", "排序"],
    "statement": "给定若干闭区间 intervals[i] = [start_i, end_i]，合并所有有重叠的区间（端点相接，如 [1,4] 与 [4,5]，也算重叠），返回互不重叠且恰好覆盖所有输入区间的区间列表。结果中区间的顺序不限。",
    "en": {
        "title": "Merge Intervals",
        "tags": [
            "Array",
            "Sorting"
        ],
        "statement": "Given closed intervals intervals[i] = [start_i, end_i], merge all overlapping intervals (intervals that touch at an endpoint, such as [1,4] and [4,5], also count as overlapping) and return a list of non-overlapping intervals that exactly covers all input intervals. The intervals in the result may be in any order.",
        "constraints": [
            "1 ≤ intervals.length ≤ 10000; intervals[i].length == 2; 0 ≤ start_i ≤ end_i ≤ 10000"
        ],
        "complexity": {
            "time": "O(n log n) (local training target, dominated by sorting)",
            "space": "O(log n) to O(n) for sorting; output not counted",
            "officialRequirement": "No official complexity requirement.",
            "notes": "n is the number of intervals. Sort by left endpoint, then scan linearly: whenever the current interval's left endpoint ≤ the merged interval's right endpoint, extend the right endpoint. Since the value range is only 10^4, a difference/coverage array solution in O(n + V) also works (V is the value range)."
        },
        "acmIoSpec": "stdin: first line the number of intervals n; then n lines, each \"2 start end\" (the element count 2 comes first). stdout: first line the number of merged intervals m; then m lines, each with two integers start end, intervals in any order."
    },
    "constraints": ["1 ≤ intervals.length ≤ 10000；intervals[i].length == 2；0 ≤ start_i ≤ end_i ≤ 10000"],
    "complexity": {
        "time": "O(n log n)（本地训练目标，排序主导）",
        "space": "O(log n)～O(n) 排序所需空间；输出不计入",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为区间个数。按左端点排序后线性扫描，当前区间左端点 ≤ 已合并区间右端点时就延伸右端点。值域只有 10^4，也可以用差分/覆盖数组 O(n + V) 求解（V 为值域）。",
        "sourceUrl": "https://leetcode.com/problems/merge-intervals/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    vector<vector<int>> merge(vector<vector<int>>& intervals) {\n        return {}; // Merged intervals, any order.\n    }\n};",
        "acmIoSpec": "stdin：第一行区间数 n；随后 n 行，每行 \"2 start end\"（先写元素个数 2）。stdout：第一行合并后的区间数 m；随后 m 行，每行两个整数 start end，区间顺序不限。",
    },
    "io": {
        "method": "merge",
        "params": [{"name": "intervals", "type": "int[][]"}],
        "returns": "int[][]",
        "checker": "unordered",
    },
}


def solve(intervals):
    out = []
    for s, e in sorted(intervals):
        if out and s <= out[-1][1]:
            out[-1][1] = max(out[-1][1], e)
        else:
            out.append([s, e])
    return out


def brute(intervals):
    # Union of closed intervals on the doubled grid, then read off runs.
    cover = [False] * 20003
    for s, e in intervals:
        for x in range(2 * s, 2 * e + 1):
            cover[x] = True
    out, x = [], 0
    while x < len(cover):
        if cover[x]:
            y = x
            while cover[y + 1]:
                y += 1
            out.append([x // 2, y // 2])
            x = y + 1
        else:
            x += 1
    return out


def check(inp, expected, solved):
    return sorted(expected) == sorted(solved)


r = rng(56)


def rand_intervals(n, hi, maxlen):
    out = []
    for _ in range(n):
        s = r.randint(0, hi)
        out.append([s, min(hi, s + r.randint(0, maxlen))])
    return out


tests = [
    {"id": "t1", "input": [[1, 3], [2, 6], [8, 10], [15, 18]], "sample": True, "expected": [[1, 6], [8, 10], [15, 18]]},
    {"id": "t2", "input": [[1, 4], [4, 5]], "sample": True, "expected": [[1, 5]]},
    {"id": "t3", "input": [[4, 7], [1, 4]], "sample": True, "expected": [[1, 7]]},
    {"id": "single", "input": [[0, 0]], "sample": False},
    {"id": "nested", "input": [[1, 10], [2, 3], [4, 5], [0, 0], [10, 10]], "sample": False},
    {"id": "adjacent-not-touching", "input": [[1, 2], [3, 4], [5, 5], [5, 6]], "sample": False},
    {"id": "duplicates", "input": [[2, 3], [2, 3], [2, 3]], "sample": False},
    {"id": "random-300", "input": rand_intervals(300, 2000, 10), "sample": False},
    {"id": "max-size-10000", "input": rand_intervals(10000, 10000, 2), "sample": False},
    {"id": "worst-case-reverse-chain", "input": [[i, i + 1] for i in range(9999, -1, -1)], "sample": False},
]
for t in tests[:9]:
    assert solve(t["input"]) == brute(t["input"]), t["id"]
write(problem, tests, solve, check=check)
