import sys, pathlib
from collections import Counter
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-347",
    "title": "前 K 个高频元素",
    "difficulty": "medium",
    "tags": ["数组", "哈希表", "分治", "桶排序", "计数", "快速选择", "排序", "堆（优先队列）"],
    "statement": "给定整数数组 nums 和整数 k，返回出现次数最多的 k 个元素，顺序任意。题目保证答案唯一（第 k 多与第 k+1 多的出现次数不同）。",
    "constraints": [
        "1 ≤ nums.length ≤ 100000；-10000 ≤ nums[i] ≤ 10000",
        "k 在 [1, 数组中不同元素的个数] 范围内；保证答案唯一",
    ],
    "complexity": {
        "time": "O(n)（本地训练目标，计数 + 按频次分桶）",
        "space": "O(n) 额外空间",
        "officialRequirement": "官方 follow-up 要求时间复杂度优于 O(n log n)。",
        "notes": "n 为数组长度，u 为不同元素个数。哈希计数 O(n)；之后按频次桶排序 O(n)，或大小为 k 的最小堆 O(u log k)，或快速选择期望 O(u)。输出顺序任意，按无序集合比较。",
        "sourceUrl": "https://leetcode.com/problems/top-k-frequent-elements/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    vector<int> topKFrequent(vector<int>& nums, int k) {\n        return {}; // The k most frequent values, any order.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数 nums；第三行整数 k。stdout：一行 k 个整数，顺序任意，以空格分隔。",
    },
    "io": {
        "method": "topKFrequent",
        "params": [{"name": "nums", "type": "int[]"}, {"name": "k", "type": "int"}],
        "returns": "int[]",
        "checker": "unordered",
    },
}


def solve(data):
    nums, k = data["nums"], data["k"]
    ranked = Counter(nums).most_common()
    assert 1 <= k <= len(ranked)
    assert k == len(ranked) or ranked[k - 1][1] > ranked[k][1], "answer must be unique"
    return sorted(v for v, _ in ranked[:k])


def check(inp, expected, solved):
    return sorted(expected) == solved


r = rng(347)


def skewed(n, lo, hi, want_k):
    """Random values with skewed frequencies; k is the nearest valid cut to want_k."""
    values = list(range(lo, hi + 1))
    r.shuffle(values)
    weights = [1.0 / (i + 1) for i in range(len(values))]
    nums = r.choices(values, weights=weights, k=n)
    freq = sorted(Counter(nums).values(), reverse=True)
    cuts = [k for k in range(1, len(freq) + 1) if k == len(freq) or freq[k - 1] > freq[k]]
    k = min(cuts, key=lambda c: abs(c - want_k))
    return {"nums": nums, "k": k}


tests = [
    {"id": "t1", "input": {"nums": [1, 1, 1, 2, 2, 3], "k": 2}, "sample": True, "expected": [1, 2]},
    {"id": "t2", "input": {"nums": [1], "k": 1}, "sample": True, "expected": [1]},
    {"id": "t3", "input": {"nums": [1, 2, 1, 2, 1, 2, 3, 1, 3, 2], "k": 2}, "sample": True, "expected": [1, 2]},
    {"id": "all-distinct-k-all", "input": {"nums": [5, -3, 9, 0, -10000, 10000], "k": 6}, "sample": False},
    {"id": "negatives", "input": {"nums": [-1, -1, -2, -2, -2, 4], "k": 1}, "sample": False},
    {"id": "random-small", "input": skewed(60, -5, 5, 3), "sample": False},
    {"id": "random-2000", "input": skewed(2000, -10000, 10000, 20), "sample": False},
    {"id": "max-size-100000", "input": skewed(100000, -300, 300, 50), "sample": False},
]
write(problem, tests, solve, check=check)
