import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-41",
    "title": "缺失的第一个正数",
    "difficulty": "hard",
    "tags": ["数组", "哈希表"],
    "statement": "给定一个未排序的整数数组 nums，返回其中没有出现的最小正整数。要求 O(n) 时间，并且只使用常数级别的额外空间（可以修改输入数组）。",
    "constraints": ["1 ≤ nums.length ≤ 100000；-2^31 ≤ nums[i] ≤ 2^31 - 1"],
    "complexity": {
        "time": "O(n)（官方要求）",
        "space": "O(1) 额外空间（官方要求；原地修改输入数组）",
        "officialRequirement": "官方要求 O(n) 时间复杂度，并且只使用常数级别的额外空间。",
        "notes": "n 为数组长度，答案必在 [1, n+1] 内。原地哈希：把每个值 v ∈ [1, n] 交换到下标 v-1 处，每次交换都让一个值归位，总交换次数 ≤ n；再找第一个 nums[i] ≠ i+1 的位置。使用哈希集合或标记数组为 O(n) 额外空间，排序为 O(n log n)，结果正确但不满足官方要求，本地测试无法区分，需自行检查。本地最大测试为 50000 个元素（官方上限 100000，为控制题库体积而缩小）。",
        "sourceUrl": "https://leetcode.com/problems/first-missing-positive/",
    },
    "en": {
        "title": "First Missing Positive",
        "tags": [
            "Array",
            "Hash Table"
        ],
        "statement": "Given an unsorted integer array nums, return the smallest positive integer that does not appear in it. The solution must run in O(n) time and use only constant extra space (modifying the input array is allowed).",
        "constraints": [
            "1 ≤ nums.length ≤ 100000; -2^31 ≤ nums[i] ≤ 2^31 - 1"
        ],
        "complexity": {
            "time": "O(n) (official requirement)",
            "space": "O(1) extra space (official requirement; the input array is modified in place)",
            "officialRequirement": "Official requirement: O(n) time complexity using only constant extra space.",
            "notes": "n is the array length; the answer must lie in [1, n+1]. In-place hashing: swap each value v ∈ [1, n] to index v-1; each swap puts one value in its place, so there are at most n swaps in total; then find the first position with nums[i] ≠ i+1. A hash set or marker array uses O(n) extra space and sorting is O(n log n); these give correct results but do not meet the official requirement, and the local tests cannot tell the difference, so check this yourself. The largest local test has 50000 elements (the official limit is 100000, reduced to keep the problem bank small)."
        },
        "acmIoSpec": "stdin: line 1 is n; line 2 has n integers. stdout: one integer, the smallest missing positive integer."
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int firstMissingPositive(vector<int>& nums) {\n        return 1; // Smallest positive integer not in nums.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数。stdout：一个整数，缺失的最小正整数。",
    },
    "io": {
        "method": "firstMissingPositive",
        "params": [{"name": "nums", "type": "int[]"}],
        "returns": "int",
    },
}


def solve(nums):
    present = set(nums)
    k = 1
    while k in present:
        k += 1
    return k


r = rng(41)
perm = list(range(1, 50001))
missing = perm.pop(31337)
perm.append(-5)
r.shuffle(perm)

full = list(range(1, 15001))
r.shuffle(full)

tests = [
    {"id": "t1", "input": [1, 2, 0], "sample": True, "expected": 3},
    {"id": "t2", "input": [3, 4, -1, 1], "sample": True, "expected": 2},
    {"id": "t3", "input": [7, 8, 9, 11, 12], "sample": True, "expected": 1},
    {"id": "single-one", "input": [1], "sample": False},
    {"id": "single-negative", "input": [-2147483648], "sample": False},
    {"id": "duplicates", "input": [1, 1, 2, 2, 3, 3], "sample": False},
    {"id": "extremes", "input": [2147483647, -2147483648, 2, 1, 0], "sample": False},
    {"id": "random-500", "input": [r.randint(-50, 400) for _ in range(500)], "sample": False},
    {"id": "worst-case-50000-perm-missing", "input": perm, "sample": False},
    {"id": "worst-case-15000-complete", "input": full, "sample": False},
    {"id": "worst-case-10000-large-values", "input": [r.randint(-2**31, 2**31 - 1) for _ in range(10000)], "sample": False},
]
write(problem, tests, solve)
