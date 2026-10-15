import sys, pathlib
from collections import Counter
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-169",
    "title": "多数元素",
    "difficulty": "easy",
    "tags": ["数组", "哈希表", "分治", "计数", "排序"],
    "statement": "给定长度为 n 的整数数组 nums，返回其中的多数元素，即出现次数严格大于 ⌊n/2⌋ 的元素。数据保证多数元素一定存在。",
    "constraints": ["1 ≤ n ≤ 5 × 10^4；-10^9 ≤ nums[i] ≤ 10^9", "多数元素一定存在"],
    "complexity": {
        "time": "O(n)（本地训练目标）",
        "space": "O(1) 额外空间（Boyer-Moore 投票，不含输入数组）",
        "officialRequirement": "官方 follow-up 要求 O(n) 时间与 O(1) 空间；题目本身未强制。",
        "notes": "n 为数组长度。投票法：计数为 0 时更换候选，相同加一、不同减一，最后的候选即为多数元素。排序取中位数 O(n log n)，哈希计数 O(n) 额外空间，都能通过本地测试但不满足 follow-up。",
        "sourceUrl": "https://leetcode.com/problems/majority-element/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int majorityElement(vector<int>& nums) {\n        return 0; // The element occurring more than n / 2 times.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数 nums。stdout：一个整数，多数元素。",
    },
    "io": {
        "method": "majorityElement",
        "params": [{"name": "nums", "type": "int[]"}],
        "returns": "int",
    },
}


def solve(a):
    value, count = Counter(a).most_common(1)[0]
    assert count > len(a) // 2, "test has no majority element"
    return value


def gen(r, n, major, lo=-10**9, hi=10**9, tail=False):
    k = n // 2 + 1
    others = [r.randint(lo, hi) for _ in range(n - k)]
    if tail:
        # Majority packed at the end: the vote candidate changes many times first.
        return others + [major] * k
    a = others + [major] * k
    r.shuffle(a)
    return a


r = rng(169)
tests = [
    {"id": "t1", "input": [3, 2, 3], "sample": True, "expected": 3},
    {"id": "t2", "input": [2, 2, 1, 1, 1, 2, 2], "sample": True, "expected": 2},
    {"id": "single", "input": [-1000000000], "sample": False},
    {"id": "all-same", "input": [7] * 9, "sample": False},
    {"id": "barely-majority", "input": [1, 2, 1, 3, 1, 4, 1], "sample": False},
    {"id": "extremes", "input": [1000000000, -1000000000, 1000000000], "sample": False},
    {"id": "random-1001", "input": gen(r, 1001, -42, -5, 5), "sample": False},
    # 5e4 values of up to 11 characters stay under ~400 KiB because half are the short majority value.
    {"id": "max-size-50000", "input": gen(r, 50000, 777, tail=True), "sample": False},
]
write(problem, tests, solve)
