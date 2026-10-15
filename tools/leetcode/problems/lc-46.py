import sys, pathlib, itertools
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-46",
    "title": "全排列",
    "difficulty": "medium",
    "tags": ["数组", "回溯"],
    "statement": "给定一个元素互不相同的整数数组 nums，返回它的所有全排列。结果中各排列的先后顺序任意，但每个排列内部的元素顺序即为该排列本身。",
    "constraints": [
        "1 ≤ nums.length ≤ 6",
        "-10 ≤ nums[i] ≤ 10",
        "nums 中的所有整数互不相同",
    ],
    "complexity": {
        "time": "O(n · n!)（本地训练目标）",
        "space": "O(n) 额外空间（递归栈与标记数组，计入递归栈；输出的 n! 个排列不计入）",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为数组长度。共 n! 个排列，每个长度 n，输出本身就需要 Θ(n · n!)。判题忽略排列之间的顺序。",
        "sourceUrl": "https://leetcode.com/problems/permutations/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    vector<vector<int>> permute(vector<int>& nums) {\n        return {}; // All permutations, in any order.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个互不相同的整数 nums。stdout：第一行排列个数；随后每行一个排列，元素用空格分隔（排列之间顺序任意）。",
    },
    "io": {
        "method": "permute",
        "params": [{"name": "nums", "type": "int[]"}],
        "returns": "int[][]",
        "checker": "unordered",
    },
}


def solve(nums):
    return [list(p) for p in itertools.permutations(nums)]


def check(inp, expected, solved):
    return sorted(map(tuple, expected)) == sorted(map(tuple, solved))


r = rng(46)
tests = [
    {"id": "t1", "sample": True, "input": [1, 2, 3],
     "expected": [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]},
    {"id": "t2", "sample": True, "input": [0, 1], "expected": [[0, 1], [1, 0]]},
    {"id": "t3", "sample": True, "input": [1], "expected": [[1]]},
    {"id": "negatives", "sample": False, "input": [-10, 10, -3]},
    {"id": "random-5", "sample": False, "input": r.sample(range(-10, 11), 5)},
    {"id": "max-size-6", "sample": False, "input": r.sample(range(-10, 11), 6)},
]
write(problem, tests, solve, check=check)
