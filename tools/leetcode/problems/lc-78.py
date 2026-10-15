import sys, pathlib, itertools
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-78",
    "title": "子集",
    "difficulty": "medium",
    "tags": ["位运算", "数组", "回溯"],
    "statement": "给定一个元素互不相同的整数数组 nums，返回它的所有子集（幂集），包括空集。结果中不能有重复子集；子集之间的顺序和每个子集内部元素的顺序都可以任意。",
    "en": {
        "title": "Subsets",
        "tags": [
            "Bit Manipulation",
            "Array",
            "Backtracking"
        ],
        "statement": "Given an integer array nums of distinct elements, return all of its subsets (the power set), including the empty set. The result must not contain duplicate subsets; the order of subsets and the order of elements within each subset are both arbitrary.",
        "constraints": [
            "1 ≤ nums.length ≤ 10",
            "-10 ≤ nums[i] ≤ 10",
            "All elements of nums are distinct"
        ],
        "complexity": {
            "time": "O(n · 2^n) (local training target)",
            "space": "O(n) extra space (backtracking recursion stack and current path, recursion stack counted; output not counted)",
            "officialRequirement": "No official complexity requirement.",
            "notes": "n is the array length. There are 2^n subsets with average length n/2; either bitmask enumeration or backtracking works. The judge ignores both the order between subsets and the order within each subset. The empty set is printed as an empty line."
        },
        "acmIoSpec": "stdin: first line n; second line n distinct integers nums. stdout: first line the number of subsets; then one subset per line, elements separated by spaces, with the empty set printed as an empty line (order between and within subsets is arbitrary)."
    },
    "constraints": [
        "1 ≤ nums.length ≤ 10",
        "-10 ≤ nums[i] ≤ 10",
        "nums 中的所有元素互不相同",
    ],
    "complexity": {
        "time": "O(n · 2^n)（本地训练目标）",
        "space": "O(n) 额外空间（回溯递归栈与当前路径，计入递归栈；输出不计入）",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为数组长度。共 2^n 个子集，平均长度 n/2；二进制枚举或回溯均可。判题同时忽略子集之间及子集内部的顺序。空集输出为一个空行。",
        "sourceUrl": "https://leetcode.com/problems/subsets/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    vector<vector<int>> subsets(vector<int>& nums) {\n        return {}; // Every subset, in any order.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个互不相同的整数 nums。stdout：第一行子集个数；随后每行一个子集，元素用空格分隔，空集输出空行（子集之间及内部顺序任意）。",
    },
    "io": {
        "method": "subsets",
        "params": [{"name": "nums", "type": "int[]"}],
        "returns": "int[][]",
        "checker": "unordered_nested",
    },
}


def solve(nums):
    return [list(c) for k in range(len(nums) + 1) for c in itertools.combinations(nums, k)]


def check(inp, expected, solved):
    norm = lambda v: sorted(tuple(sorted(x)) for x in v)
    return norm(expected) == norm(solved)


r = rng(78)
tests = [
    {"id": "t1", "sample": True, "input": [1, 2, 3],
     "expected": [[], [1], [2], [1, 2], [3], [1, 3], [2, 3], [1, 2, 3]]},
    {"id": "t2", "sample": True, "input": [0], "expected": [[], [0]]},
    {"id": "negatives", "sample": False, "input": [-10, 10, -1, 1]},
    {"id": "random-6", "sample": False, "input": r.sample(range(-10, 11), 6)},
    {"id": "max-size-10", "sample": False, "input": r.sample(range(-10, 11), 10)},
]
write(problem, tests, solve, check=check)
