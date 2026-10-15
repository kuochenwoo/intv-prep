import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-189",
    "title": "轮转数组",
    "difficulty": "medium",
    "tags": ["数组", "数学", "双指针"],
    "statement": "给定整数数组 nums 和非负整数 k，把数组元素整体向右轮转 k 个位置（最右边的元素移到最左边，重复 k 次）。要求原地修改 nums，核心函数不返回值，系统检查修改后的数组。",
    "en": {'title': 'Rotate Array',
     'tags': ['Array', 'Math', 'Two Pointers'],
     'statement': 'Given an integer array nums and a non-negative integer k, rotate the array to the '
                  'right by k positions (move the rightmost element to the leftmost position, repeated '
                  'k times). Modify nums in place; the core function returns nothing, and the system '
                  'checks the modified array.',
     'constraints': ['1 ≤ nums.length ≤ 100000; -2^31 ≤ nums[i] ≤ 2^31 - 1; 0 ≤ k ≤ 100000'],
     'complexity': {'time': 'O(n) (local training target)',
                    'space': 'O(1) extra space (three reversals or cyclic replacement)',
                    'officialRequirement': 'Official follow-up: come up with as many solutions as you '
                                           'can (at least three), and try to do it in place with O(1) '
                                           'extra space.',
                    'notes': 'n is the array length; k may exceed n, so take k mod n first. Three '
                             'reversals: reverse the whole array, then reverse the first k and the '
                             'last n-k elements separately. Copying into a new array (O(n) extra '
                             'space) is correct but does not meet the follow-up; shifting right one '
                             'step at a time, O(n·k), times out on large tests. The largest local test '
                             'has 50000 elements (official limit is 100000, reduced to keep the '
                             'problem bank small, since both input and output are stored in it).'},
     'acmIoSpec': 'stdin: line 1 has n and k; line 2 has n integers nums. stdout: one line with the n '
                  'integers after rotation.'},
    "constraints": ["1 ≤ nums.length ≤ 100000；-2^31 ≤ nums[i] ≤ 2^31 - 1；0 ≤ k ≤ 100000"],
    "complexity": {
        "time": "O(n)（本地训练目标）",
        "space": "O(1) 额外空间（三次翻转或环状替换）",
        "officialRequirement": "官方 follow-up：尽可能想出至少三种解法，并尝试用 O(1) 额外空间原地完成。",
        "notes": "n 为数组长度，k 可能大于 n，先取 k mod n。三次翻转：整体翻转，再分别翻转前 k 个与后 n-k 个。复制到新数组 O(n) 额外空间结果正确但不满足 follow-up；每次右移一位的 O(n·k) 做法在大测试上会超时。本地最大测试为 50000 个元素（官方上限 100000，为控制题库体积而缩小，输入和输出都要写入题库）。",
        "sourceUrl": "https://leetcode.com/problems/rotate-array/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    void rotate(vector<int>& nums, int k) {\n        // Modify nums in place.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n 和 k；第二行 n 个整数 nums。stdout：一行，轮转后的 n 个整数。",
    },
    # The whole rotated array is printed (up to ~110 KiB locally), above the 64 KiB default.
    "limits": {"timeLimitMs": 1000, "memoryLimitMb": 128, "outputLimitKb": 256},
    "io": {
        "method": "rotate",
        "params": [{"name": "nums", "type": "int[]"}, {"name": "k", "type": "int"}],
        "returns": "void",
        "output": "nums",
        "stdin": ["nums.length k", "nums"],
    },
}


def solve(d):
    nums, k = list(d["nums"]), d["k"]
    steps = k % len(nums)
    if len(nums) * steps <= 100000:
        for _ in range(steps):  # literal one-step rotations for small inputs
            nums.insert(0, nums.pop())
        return nums
    return nums[-steps:] + nums[:-steps]


r = rng(189)
I32 = (-2**31, 2**31 - 1)
tests = [
    {"id": "t1", "input": {"nums": [1, 2, 3, 4, 5, 6, 7], "k": 3}, "sample": True, "expected": [5, 6, 7, 1, 2, 3, 4]},
    {"id": "t2", "input": {"nums": [-1, -100, 3, 99], "k": 2}, "sample": True, "expected": [3, 99, -1, -100]},
    {"id": "k-zero", "input": {"nums": [1, 2, 3], "k": 0}, "sample": False},
    {"id": "k-equals-n", "input": {"nums": [1, 2, 3], "k": 3}, "sample": False},
    {"id": "k-larger-than-n", "input": {"nums": [1, 2], "k": 5}, "sample": False},
    {"id": "single", "input": {"nums": [-2147483648], "k": 100000}, "sample": False},
    {"id": "gcd-cycles", "input": {"nums": list(range(12)), "k": 8}, "sample": False},
    {"id": "random-1000", "input": {"nums": [r.randint(*I32) for _ in range(1000)], "k": 357}, "sample": False},
    {"id": "worst-case-50000", "input": {"nums": [r.randint(0, 9) for _ in range(50000)], "k": 24999}, "sample": False},
    {"id": "worst-case-full-range-10000", "input": {"nums": [r.randint(*I32) for _ in range(10000)], "k": 100000}, "sample": False},
]
write(problem, tests, solve)
