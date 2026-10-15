import sys, pathlib
from itertools import permutations
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-31",
    "title": "下一个排列",
    "difficulty": "medium",
    "tags": ["数组", "双指针", "原地修改"],
    "statement": "把整数数组 nums 原地改为其字典序中的下一个更大排列；若已是字典序最大的排列，则改为最小的排列（升序）。只允许使用常数额外空间。核心函数不返回值，系统检查修改后的数组。",
    "constraints": ["1 ≤ nums.length ≤ 100；0 ≤ nums[i] ≤ 100（可能有重复元素）"],
    "complexity": {
        "time": "O(n)（本地训练目标）",
        "space": "O(1) 额外空间，原地修改",
        "officialRequirement": "官方要求原地修改，且只使用常数额外空间。",
        "notes": "n 为数组长度。从右找第一个 nums[i] < nums[i+1] 的位置 i，再从右找第一个大于 nums[i] 的元素与之交换，最后翻转 i 之后的后缀；找不到 i 时整体翻转。判题只检查结果，无法验证是否原地或常数空间。",
        "sourceUrl": "https://leetcode.com/problems/next-permutation/",
    },
    "en": {
        "title": "Next Permutation",
        "tags": [
            "Array",
            "Two Pointers",
            "In-Place"
        ],
        "statement": "Rearrange the integer array nums in place into its next lexicographically greater permutation; if it is already the lexicographically largest permutation, rearrange it into the smallest one (ascending order). Only constant extra space is allowed. The core function returns nothing; the system checks the modified array.",
        "constraints": [
            "1 ≤ nums.length ≤ 100; 0 ≤ nums[i] ≤ 100 (duplicates possible)"
        ],
        "complexity": {
            "time": "O(n) (local training target)",
            "space": "O(1) extra space, modified in place",
            "officialRequirement": "Official requirement: modify in place using only constant extra space.",
            "notes": "n is the array length. From the right, find the first position i with nums[i] < nums[i+1]; then from the right, find the first element greater than nums[i] and swap them; finally reverse the suffix after i. If no such i exists, reverse the whole array. The judge only checks the result and cannot verify in-place operation or constant space."
        },
        "acmIoSpec": "stdin: line 1 is n; line 2 has n integers nums. stdout: one line with the n transformed integers, separated by spaces."
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    void nextPermutation(vector<int>& nums) {\n        // Rearrange nums in place into the next lexicographic permutation.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数 nums。stdout：一行，变换后的 n 个整数，以空格分隔。",
    },
    "io": {
        "method": "nextPermutation",
        "params": [{"name": "nums", "type": "int[]"}],
        "returns": "void",
        "output": "nums",
    },
}


def solve(a):
    a = list(a)
    i = len(a) - 2
    while i >= 0 and a[i] >= a[i + 1]:
        i -= 1
    if i < 0:
        return a[::-1]
    j = len(a) - 1
    while a[j] <= a[i]:
        j -= 1
    a[i], a[j] = a[j], a[i]
    return a[:i + 1] + a[i + 1:][::-1]


def brute(a):
    perms = sorted(set(permutations(a)))
    k = perms.index(tuple(a))
    return list(perms[(k + 1) % len(perms)])


r = rng(31)
tests = [
    {"id": "t1", "input": [1, 2, 3], "sample": True, "expected": [1, 3, 2]},
    {"id": "t2", "input": [3, 2, 1], "sample": True, "expected": [1, 2, 3]},
    {"id": "t3", "input": [1, 1, 5], "sample": True, "expected": [1, 5, 1]},
    {"id": "single", "input": [7], "sample": False},
    {"id": "all-equal", "input": [4, 4, 4, 4], "sample": False},
    {"id": "duplicates-pivot", "input": [1, 5, 1], "sample": False},
    {"id": "swap-with-rightmost-larger", "input": [2, 3, 1, 3, 3], "sample": False},
    {"id": "random-7", "input": [r.randint(0, 3) for _ in range(7)], "sample": False},
    {"id": "random-8", "input": [r.randint(0, 100) for _ in range(8)], "sample": False},
    {"id": "max-size-100", "input": [r.randint(0, 100) for _ in range(100)], "sample": False},
    # Pivot at index 0 forces a full-length suffix reversal.
    {"id": "max-size-100-pivot-first", "input": [0] + list(range(100, 1, -1)), "sample": False},
    {"id": "max-size-100-descending", "input": list(range(100, 0, -1)), "sample": False},
]
for t in tests:
    if len(t["input"]) <= 8:
        assert brute(t["input"]) == solve(t["input"]), t["id"]
write(problem, tests, solve)
