import sys, pathlib, heapq
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-239",
    "title": "滑动窗口最大值",
    "difficulty": "hard",
    "tags": ["队列", "数组", "滑动窗口", "单调队列", "堆（优先队列）"],
    "statement": "给定整数数组 nums 和窗口大小 k。一个长度为 k 的窗口从数组最左端每次向右滑动一位，直到最右端。按顺序返回每个窗口内的最大值（共 n - k + 1 个）。",
    "constraints": ["1 ≤ nums.length ≤ 100000；-10^4 ≤ nums[i] ≤ 10^4；1 ≤ k ≤ nums.length"],
    "complexity": {
        "time": "O(n)（本地训练目标）",
        "space": "O(k) 额外空间（单调队列）；输出不计入",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为数组长度。单调递减双端队列保存下标，每个下标最多入队出队各一次；堆做法 O(n log n) 也能通过。逐窗口扫描 O(n·k) 在最大测试上会超时。max-size 测试 n = 100000、k = 80000（n 为官方上限），但数值取 0～9 以控制题库体积；另有全值域的 10000 规模递减/递增测试。",
        "sourceUrl": "https://leetcode.com/problems/sliding-window-maximum/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    vector<int> maxSlidingWindow(vector<int>& nums, int k) {\n        return {}; // Maximum of each window, left to right.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n 和 k；第二行 n 个整数 nums。stdout：一行 n - k + 1 个整数，依次为每个窗口的最大值。",
    },
    "io": {
        "method": "maxSlidingWindow",
        "params": [{"name": "nums", "type": "int[]"}, {"name": "k", "type": "int"}],
        "returns": "int[]",
        "stdin": ["nums.length k", "nums"],
    },
}


def solve(d):
    nums, k = d["nums"], d["k"]
    if len(nums) * k <= 100000:
        return [max(nums[i:i + k]) for i in range(len(nums) - k + 1)]
    # Max-heap with lazy deletion of indices that left the window.
    heap, out = [], []
    for i, x in enumerate(nums):
        heapq.heappush(heap, (-x, i))
        if i >= k - 1:
            while heap[0][1] <= i - k:
                heapq.heappop(heap)
            out.append(-heap[0][0])
    return out


r = rng(239)
tests = [
    {"id": "t1", "input": {"nums": [1, 3, -1, -3, 5, 3, 6, 7], "k": 3}, "sample": True, "expected": [3, 3, 5, 5, 6, 7]},
    {"id": "t2", "input": {"nums": [1], "k": 1}, "sample": True, "expected": [1]},
    {"id": "k-equals-n", "input": {"nums": [4, -2, 9, 9, -10000], "k": 5}, "sample": False},
    {"id": "k-one", "input": {"nums": [3, -1, 2, 2, -5], "k": 1}, "sample": False},
    {"id": "all-equal", "input": {"nums": [7] * 30, "k": 4}, "sample": False},
    {"id": "random-500", "input": {"nums": [r.randint(-100, 100) for _ in range(500)], "k": 17}, "sample": False},
    {"id": "max-size-100000", "input": {"nums": [r.randint(0, 9) for _ in range(100000)], "k": 80000}, "sample": False},
    {"id": "worst-case-decreasing-10000", "input": {"nums": [10000 - 2 * i for i in range(10000)], "k": 4000}, "sample": False},
    {"id": "worst-case-increasing-10000", "input": {"nums": [-10000 + 2 * i for i in range(10000)], "k": 4000}, "sample": False},
]
write(problem, tests, solve)
