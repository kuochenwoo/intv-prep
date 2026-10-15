import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-152",
    "title": "乘积最大子数组",
    "difficulty": "medium",
    "tags": ["数组", "动态规划"],
    "statement": "给定整数数组 nums，找出乘积最大的非空连续子数组，返回该乘积。数据保证任意子数组的乘积都在 32 位有符号整数范围内。",
    "en": {'title': 'Maximum Product Subarray',
     'tags': ['Array', 'Dynamic Programming'],
     'statement': 'Given an integer array nums, find the non-empty contiguous subarray with the '
                  'largest product and return that product. The product of any subarray is guaranteed '
                  'to fit in a 32-bit signed integer.',
     'constraints': ['1 ≤ nums.length ≤ 2 × 10^4; -10 ≤ nums[i] ≤ 10',
                     'The product of any prefix or suffix (i.e., any subarray) fits in a 32-bit signed '
                     'integer'],
     'complexity': {'time': 'O(n) (local training target)',
                    'space': 'O(1) extra space (excluding the input array)',
                    'officialRequirement': 'No official complexity requirement.',
                    'notes': 'n is the array length. Track both the maximum and minimum product of '
                             'subarrays ending at the current position (a negative number swaps them); '
                             'alternatively, split at zeros and take the maximum of prefix and suffix '
                             'products within each segment.'},
     'acmIoSpec': 'stdin: line 1 is n; line 2 has n integers nums. stdout: one integer, the maximum '
                  'subarray product.'},
    "constraints": [
        "1 ≤ nums.length ≤ 2 × 10^4；-10 ≤ nums[i] ≤ 10",
        "任意前缀或后缀（即任意子数组）的乘积都能用 32 位有符号整数表示",
    ],
    "complexity": {
        "time": "O(n)（本地训练目标）",
        "space": "O(1) 额外空间（不含输入数组）",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为数组长度。同时维护以当前位置结尾的最大与最小乘积（负数会交换两者）；也可按 0 分段，在每段内取前缀积与后缀积的最大值。",
        "sourceUrl": "https://leetcode.com/problems/maximum-product-subarray/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int maxProduct(vector<int>& nums) {\n        return 0; // Largest product of a non-empty contiguous subarray.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数 nums。stdout：一个整数，最大子数组乘积。",
    },
    "io": {
        "method": "maxProduct",
        "params": [{"name": "nums", "type": "int[]"}],
        "returns": "int",
    },
}

LIMIT = 2**31 - 1


def segments(a):
    seg = []
    for x in a:
        if x == 0:
            if seg:
                yield seg
            seg = []
        else:
            seg.append(x)
    if seg:
        yield seg


def prod(xs):
    p = 1
    for x in xs:
        p *= x
    return p


def solve(a):
    # Zero-split reasoning, independent of the min/max DP in C++.
    cands = [0] if 0 in a else []
    for seg in segments(a):
        assert abs(prod(seg)) <= LIMIT, "test violates the 32-bit product guarantee"
        p = prod(seg)
        cands.append(p)
        if p < 0:
            negs = [i for i, x in enumerate(seg) if x < 0]
            if negs[0] + 1 < len(seg):
                cands.append(prod(seg[negs[0] + 1:]))
            if negs[-1] > 0:
                cands.append(prod(seg[:negs[-1]]))
    return max(cands)


def brute(a):
    return max(prod(a[i:j]) for i in range(len(a)) for j in range(i + 1, len(a) + 1))


def gen(r, n):
    # Mostly ±1 with occasional larger factors; zeros keep every segment's product bounded.
    out, mag = [], 1
    while len(out) < n:
        x = r.choice([1, -1, 1, -1, 2, -2, 3, -3, 10, -10, 0, 5, -1, 1])
        if x != 0 and mag * abs(x) > LIMIT:
            x = 0
        mag = 1 if x == 0 else mag * abs(x)
        out.append(x)
    return out


r = rng(152)
tests = [
    {"id": "t1", "input": [2, 3, -2, 4], "sample": True, "expected": 6},
    {"id": "t2", "input": [-2, 0, -1], "sample": True, "expected": 0},
    {"id": "single-negative", "input": [-2], "sample": False},
    {"id": "two-negatives", "input": [-2, -3], "sample": False},
    {"id": "negative-then-positive", "input": [-2, 5, -1, 0, 3, -4], "sample": False},
    {"id": "all-zero", "input": [0, 0, 0], "sample": False},
    {"id": "neg-zero-neg", "input": [-3, 0, -5], "sample": False},
    {"id": "random-60", "input": gen(r, 60), "sample": False},
    {"id": "max-size-20000", "input": gen(r, 20000), "sample": False},
    # One long zero-free run whose product needs 31 bits: 2^30 times sign flips.
    {"id": "max-size-20000-long-run", "input": [1, -1] * 4985 + [-2] * 30 + [-1, 1] * 5000, "sample": False},
]
for t in tests:
    if len(t["input"]) <= 60:
        assert brute(t["input"]) == solve(t["input"]), t["id"]
write(problem, tests, solve)
