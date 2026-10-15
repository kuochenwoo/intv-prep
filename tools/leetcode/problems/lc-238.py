import sys, pathlib, math
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-238",
    "title": "除自身以外数组的乘积",
    "difficulty": "medium",
    "tags": ["数组", "前缀和"],
    "statement": "给定整数数组 nums，返回数组 answer，其中 answer[i] 等于 nums 中除 nums[i] 之外所有元素的乘积。不允许使用除法，且要在 O(n) 时间内完成。题目保证任意前缀或后缀的乘积都在 32 位有符号整数范围内。",
    "constraints": [
        "2 ≤ nums.length ≤ 100000；-30 ≤ nums[i] ≤ 30",
        "任意前缀或后缀的乘积都在 32 位有符号整数范围内",
    ],
    "complexity": {
        "time": "O(n)（官方要求）",
        "space": "O(1) 额外空间（输出数组不计入，官方 follow-up）；前缀/后缀两个数组为 O(n)",
        "officialRequirement": "官方要求 O(n) 时间且不使用除法；follow-up 要求 O(1) 额外空间（输出数组不计入额外空间）。",
        "notes": "n 为数组长度。answer[i] = 左侧前缀积 × 右侧后缀积；先在输出数组中写入前缀积，再从右向左乘上滚动的后缀积。本地无法检测是否使用了除法，需自行检查程序。本地最大测试为 50000 个元素（官方上限 100000，为控制题库体积而缩小）。",
        "sourceUrl": "https://leetcode.com/problems/product-of-array-except-self/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    vector<int> productExceptSelf(vector<int>& nums) {\n        return {}; // answer[i] = product of all nums except nums[i], no division.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数 nums。stdout：一行 n 个整数 answer。",
    },
    # The largest tests print 50000 values, above the 64 KiB default.
    "limits": {"timeLimitMs": 1000, "memoryLimitMb": 128, "outputLimitKb": 256},
    "io": {
        "method": "productExceptSelf",
        "params": [{"name": "nums", "type": "int[]"}],
        "returns": "int[]",
    },
}


def solve(nums):
    n = len(nums)
    if n <= 300:
        return [math.prod(nums[:i] + nums[i + 1:]) for i in range(n)]
    left = [1] * n
    for i in range(1, n):
        left[i] = left[i - 1] * nums[i - 1]
    right = 1
    out = [0] * n
    for i in reversed(range(n)):
        out[i] = left[i] * right
        right *= nums[i]
    return out


r = rng(238)


def small_product(n, factors, zeros=0):
    nums = [r.choice([1, -1]) for _ in range(n - len(factors) - zeros)] + factors + [0] * zeros
    r.shuffle(nums)
    return nums


def fits(nums):
    p = 1
    for x in nums:
        p *= x
        if not -2**31 <= p < 2**31:
            return False
    p = 1
    for x in reversed(nums):
        p *= x
        if not -2**31 <= p < 2**31:
            return False
    return True


tests = [
    {"id": "t1", "input": [1, 2, 3, 4], "sample": True, "expected": [24, 12, 8, 6]},
    {"id": "t2", "input": [-1, 1, 0, -3, 3], "sample": True, "expected": [0, 0, 9, 0, 0]},
    {"id": "two-elements", "input": [-30, 30], "sample": False},
    {"id": "two-zeros", "input": [0, 4, 0, 5], "sample": False},
    {"id": "one-zero", "input": [2, 3, 0, -4], "sample": False},
    {"id": "large-product", "input": [30, -30, 30, 30, -30, 2], "sample": False},
    {"id": "random-200", "input": small_product(200, [r.randint(-30, 30) or 7 for _ in range(5)]), "sample": False},
    {"id": "worst-case-50000-one-zero", "input": small_product(50000, [2, -3, 5, 7, -11, 13, 17], zeros=1), "sample": False},
    {"id": "worst-case-15000-nonzero", "input": small_product(15000, [2, 3, -5, 7, 30, -30, 29, -2, 3]), "sample": False},
]
for t in tests:
    assert fits(t["input"]), t["id"]
write(problem, tests, solve)
