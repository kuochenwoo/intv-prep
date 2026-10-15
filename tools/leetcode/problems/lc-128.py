import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-128",
    "title": "最长连续序列",
    "difficulty": "medium",
    "tags": ["并查集", "数组", "哈希表"],
    "statement": "给定一个未排序的整数数组 nums，找出其中数值连续（相邻两数相差 1）的最长序列的长度。序列元素不要求在原数组中相邻，重复值只算一次。空数组返回 0。",
    "en": {'title': 'Longest Consecutive Sequence',
     'tags': ['Union Find', 'Array', 'Hash Table'],
     'statement': 'Given an unsorted integer array nums, find the length of the longest sequence of consecutive '
                  'values (adjacent numbers differ by 1). The elements need not be adjacent in the original '
                  'array, and duplicate values count only once. Return 0 for an empty array.',
     'constraints': ['0 ≤ nums.length ≤ 100000; -10^9 ≤ nums[i] ≤ 10^9'],
     'complexity': {'time': 'O(n) average time (official requirement)',
                    'space': 'O(n) extra space (hash set)',
                    'officialRequirement': 'Official requirement: design an algorithm that runs in O(n) time.',
                    'notes': 'n is the array length. Put all numbers in a hash set and count forward only from '
                             'starting points where x-1 is not in the set, so each number is visited a constant '
                             'number of times; hashing is O(1) on average. Sorting is O(n log n): it gives '
                             'correct results but does not meet the official requirement, and local tests cannot '
                             'tell the difference. The largest local test has 30000 elements (the official limit '
                             'is 100000, reduced to keep the problem bank small).'},
     'acmIoSpec': 'stdin: line 1 is n; line 2 has n integers (an empty line when n is 0). stdout: one integer, '
                  'the length of the longest consecutive sequence.'},
    "constraints": ["0 ≤ nums.length ≤ 100000；-10^9 ≤ nums[i] ≤ 10^9"],
    "complexity": {
        "time": "O(n) 平均时间（官方要求）",
        "space": "O(n) 额外空间（哈希集合）",
        "officialRequirement": "官方要求设计 O(n) 时间复杂度的算法。",
        "notes": "n 为数组长度。把所有数放进哈希集合，只从「x-1 不在集合中」的起点向后数，每个数最多被访问常数次；哈希为平均 O(1)。排序做法为 O(n log n)，结果正确但不满足官方要求，本地测试无法区分。本地最大测试为 30000 个元素（官方上限 100000，为控制题库体积而缩小）。",
        "sourceUrl": "https://leetcode.com/problems/longest-consecutive-sequence/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int longestConsecutive(vector<int>& nums) {\n        return 0; // Length of the longest run of consecutive values.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数（n 为 0 时为空行）。stdout：一个整数，最长连续序列的长度。",
    },
    "io": {
        "method": "longestConsecutive",
        "params": [{"name": "nums", "type": "int[]"}],
        "returns": "int",
    },
}


def solve(nums):
    # Sort + scan: independent of the hash-set C++ solutions.
    vals = sorted(set(nums))
    best = cur = 0
    for i, v in enumerate(vals):
        cur = cur + 1 if i and vals[i - 1] == v - 1 else 1
        best = max(best, cur)
    return best


def brute(nums):
    s = set(nums)
    best = 0
    for x in s:
        k = 0
        while x + k in s:
            k += 1
        best = max(best, k)
    return best


r = rng(128)


def runs(total, pieces, lo, hi):
    out = []
    for _ in range(pieces):
        start = r.randint(lo, hi)
        length = r.randint(1, total // pieces * 2)
        out.extend(range(start, min(start + length, hi + 1)))
    out = out[:total]
    r.shuffle(out)
    return out


big_run = list(range(10**9 - 14999, 10**9 + 1)) + [r.randint(-10**9, 10**9) for _ in range(14000)] + [-10**9, -10**9 + 1] * 500
r.shuffle(big_run)

tests = [
    {"id": "t1", "input": [100, 4, 200, 1, 3, 2], "sample": True, "expected": 4},
    {"id": "t2", "input": [0, 3, 7, 2, 5, 8, 4, 6, 0, 1], "sample": True, "expected": 9},
    {"id": "t3", "input": [1, 0, 1, 2], "sample": True, "expected": 3},
    {"id": "empty", "input": [], "sample": False},
    {"id": "single", "input": [-7], "sample": False},
    {"id": "all-equal", "input": [5] * 50, "sample": False},
    {"id": "extremes", "input": [10**9, -10**9, 10**9 - 1, -10**9 + 1, 0], "sample": False},
    {"id": "negatives", "input": [-3, -1, -2, -5, -4, 2, 1], "sample": False},
    {"id": "random-500", "input": runs(500, 20, -200, 200), "sample": False},
    {"id": "worst-case-30000-long-run", "input": big_run, "sample": False},
    {"id": "worst-case-20000-many-runs", "input": runs(20000, 1500, -10**6, 10**6), "sample": False},
]
for t in tests[:9]:
    assert solve(t["input"]) == brute(t["input"])
write(problem, tests, solve)
