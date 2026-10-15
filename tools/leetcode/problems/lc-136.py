import sys, pathlib
from collections import Counter
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-136",
    "title": "只出现一次的数字",
    "difficulty": "easy",
    "tags": ["位运算", "数组"],
    "statement": "给定非空整数数组 nums，其中恰有一个元素只出现一次，其余每个元素都恰好出现两次，找出那个只出现一次的元素。要求线性时间且只使用常数额外空间。",
    "en": {'title': 'Single Number',
     'tags': ['Bit Manipulation', 'Array'],
     'statement': 'Given a non-empty integer array nums in which exactly one element appears once and every '
                  'other element appears exactly twice, find the element that appears once. Use linear time and '
                  'only constant extra space.',
     'constraints': ['1 ≤ nums.length ≤ 3 × 10^4; -3 × 10^4 ≤ nums[i] ≤ 3 × 10^4',
                     'Every element appears exactly twice except for one element that appears once'],
     'complexity': {'time': 'O(n) (official requirement)',
                    'space': 'O(1) extra space (official requirement, excluding the input array)',
                    'officialRequirement': 'The official statement requires linear time complexity and only '
                                           'constant extra space.',
                    'notes': 'n is the array length. XOR all elements: paired elements cancel out, leaving the '
                             'answer. Hash counting or sorting also produce correct output, but violate the '
                             'constant-space or linear-time requirement respectively; time limits and memory '
                             'measurements cannot prove complexity, so check your code yourself.'},
     'acmIoSpec': 'stdin: line 1 is n; line 2 has n integers nums. stdout: one integer, the element that appears '
                  'once.'},
    "constraints": [
        "1 ≤ nums.length ≤ 3 × 10^4；-3 × 10^4 ≤ nums[i] ≤ 3 × 10^4",
        "除一个元素出现一次外，其余元素均恰好出现两次",
    ],
    "complexity": {
        "time": "O(n)（官方要求）",
        "space": "O(1) 额外空间（官方要求，不含输入数组）",
        "officialRequirement": "官方题面要求线性时间复杂度，且只使用常数额外空间。",
        "notes": "n 为数组长度。所有元素异或：成对元素相互抵消，剩下的就是答案。哈希计数或排序也能得到正确输出，但分别违反常数空间或线性时间要求；限时与内存统计无法证明复杂度，需自行检查代码。",
        "sourceUrl": "https://leetcode.com/problems/single-number/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int singleNumber(vector<int>& nums) {\n        return 0; // The element that appears exactly once.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数 nums。stdout：一个整数，只出现一次的元素。",
    },
    "io": {
        "method": "singleNumber",
        "params": [{"name": "nums", "type": "int[]"}],
        "returns": "int",
    },
}


def solve(a):
    once = [x for x, c in Counter(a).items() if c == 1]
    assert len(once) == 1 and all(c in (1, 2) for c in Counter(a).values()), "invalid test"
    return once[0]


def gen(r, pairs, lo=-30000, hi=30000):
    vals = r.sample(range(lo, hi + 1), pairs + 1)
    a = vals[:pairs] * 2 + [vals[pairs]]
    r.shuffle(a)
    return a


r = rng(136)
tests = [
    {"id": "t1", "input": [2, 2, 1], "sample": True, "expected": 1},
    {"id": "t2", "input": [4, 1, 2, 1, 2], "sample": True, "expected": 4},
    {"id": "t3", "input": [1], "sample": True, "expected": 1},
    {"id": "negative-single", "input": [-3, 7, 7], "sample": False},
    {"id": "zero-single", "input": [5, -5, 0, -5, 5], "sample": False},
    {"id": "extremes", "input": [30000, -30000, -30000], "sample": False},
    {"id": "random-101", "input": gen(r, 50), "sample": False},
    {"id": "max-size-29999", "input": gen(r, 14999), "sample": False},
]
write(problem, tests, solve)
