import sys, pathlib, itertools
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-15",
    "title": "三数之和",
    "difficulty": "medium",
    "tags": ["数组", "双指针", "排序"],
    "statement": "给定整数数组 nums，找出所有和为 0 的三元组 [nums[i], nums[j], nums[k]]，要求 i、j、k 互不相同。结果中不能出现重复的三元组（按值的多重集合判断重复）。三元组之间、三元组内部的顺序都不限。",
    "constraints": ["3 ≤ nums.length ≤ 3000；-10^5 ≤ nums[i] ≤ 10^5"],
    "complexity": {
        "time": "O(n^2)（本地训练目标）",
        "space": "排序 + 双指针为 O(log n)～O(n) 排序栈/缓冲；输出不计入",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为数组长度。先排序，固定最小的数后用双指针在其右侧找和为相反数的两数，并跳过重复值去重。O(n^3) 枚举在 n = 3000 时会超时。最大测试 n = 3000（官方上限），取完整值域内的随机数，约产生上万个三元组。",
        "sourceUrl": "https://leetcode.com/problems/3sum/",
    },
    "en": {
        "title": "3Sum",
        "tags": [
            "Array",
            "Two Pointers",
            "Sorting"
        ],
        "statement": "Given an integer array nums, find all triplets [nums[i], nums[j], nums[k]] that sum to 0, where i, j and k are pairwise distinct. The result must not contain duplicate triplets (duplicates are judged by the multiset of values). Neither the order of triplets nor the order within a triplet matters.",
        "constraints": [
            "3 ≤ nums.length ≤ 3000; -10^5 ≤ nums[i] ≤ 10^5"
        ],
        "complexity": {
            "time": "O(n^2) (local training target)",
            "space": "Sort + two pointers: O(log n) to O(n) for the sort's stack/buffer; output not counted",
            "officialRequirement": "No official complexity requirement.",
            "notes": "n is the array length. Sort first, fix the smallest number, then use two pointers to its right to find two numbers summing to its negation, skipping repeated values to deduplicate. O(n^3) enumeration times out at n = 3000. The largest test has n = 3000 (the official limit) with random values over the full range, producing on the order of tens of thousands of triplets."
        },
        "acmIoSpec": "stdin: line 1 is n; line 2 has n integers. stdout: line 1 is the number of triplets m; then m lines, each with three integers. The order of triplets and the order within each triplet are arbitrary; if there is no solution, output only 0."
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    vector<vector<int>> threeSum(vector<int>& nums) {\n        return {}; // Unique zero-sum triplets, any order.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数。stdout：第一行三元组个数 m；随后 m 行，每行三个整数。三元组顺序与三元组内部顺序不限；没有解时只输出 0。",
    },
    # The max-size test prints thousands of triplets, above the 64 KiB default.
    "limits": {"timeLimitMs": 1000, "memoryLimitMb": 128, "outputLimitKb": 512},
    "io": {
        "method": "threeSum",
        "params": [{"name": "nums", "type": "int[]"}],
        "returns": "int[][]",
        "checker": "unordered_nested",
    },
}


def solve(nums):
    # Hash-based O(n^2): for each i, look up -(a+b) among earlier j values.
    found = set()
    n = len(nums)
    for i in range(n):
        seen = set()
        a = nums[i]
        for j in range(i + 1, n):
            b = nums[j]
            c = -a - b
            if c in seen:
                found.add(tuple(sorted((a, b, c))))
            seen.add(b)
    return [list(t) for t in sorted(found)]


def brute(nums):
    return [list(t) for t in sorted({tuple(sorted(c)) for c in itertools.combinations(nums, 3) if sum(c) == 0})]


def canon(v):
    return sorted(sorted(t) for t in v)


def check(inp, expected, solved):
    return canon(expected) == canon(solved)


r = rng(15)
tests = [
    {"id": "t1", "input": [-1, 0, 1, 2, -1, -4], "sample": True, "expected": [[-1, -1, 2], [-1, 0, 1]]},
    {"id": "t2", "input": [0, 1, 1], "sample": True, "expected": []},
    {"id": "t3", "input": [0, 0, 0], "sample": True, "expected": [[0, 0, 0]]},
    {"id": "many-duplicates", "input": [-2, 0, 0, 2, 2, -2, 0, 1, -1, 1, -1, 0], "sample": False},
    {"id": "extremes", "input": [-100000, 50000, 50000, 100000, 0, -100000], "sample": False},
    {"id": "all-positive", "input": [1, 2, 3, 4, 5], "sample": False},
    {"id": "random-150", "input": [r.randint(-50, 50) for _ in range(150)], "sample": False},
    {"id": "all-zero-3000", "input": [0] * 3000, "sample": False},
    {"id": "max-size-3000", "input": [r.randint(-100000, 100000) for _ in range(3000)], "sample": False},
]
for t in tests[:7]:
    assert solve(t["input"]) == brute(t["input"]), t["id"]
write(problem, tests, solve, check=check)
