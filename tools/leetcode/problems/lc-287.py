import sys, pathlib
from collections import Counter
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-287",
    "title": "寻找重复数",
    "difficulty": "medium",
    "tags": ["位运算", "数组", "双指针", "二分查找"],
    "statement": "数组 nums 含 n + 1 个整数，每个都在 [1, n] 范围内，其中恰有一个数值是重复的（可能重复两次或更多次）。在不修改数组、只使用常数额外空间的前提下找出这个重复的数。",
    "constraints": [
        "1 ≤ n ≤ 10^5；nums.length = n + 1；1 ≤ nums[i] ≤ n",
        "只有一个数值出现两次或更多次，其余数值至多出现一次",
        "本地最大测试 n = 50000（官方上限 10^5，为控制题库体积而缩小）",
    ],
    "complexity": {
        "time": "O(n)（本地训练目标，Floyd 判圈）；对值域二分计数为 O(n log n)",
        "space": "O(1) 额外空间（官方要求），且不得修改 nums",
        "officialRequirement": "官方要求不修改数组且只使用常数额外空间；follow-up 要求证明重复数必然存在，并给出线性时间的解法。",
        "notes": "n 为数组最大取值，数组长度为 n + 1。把 i → nums[i] 看成链表，重复数就是环的入口。判题无法检测是否修改了数组或使用了额外空间（排序、标记取负、哈希集合都能得到正确输出），需自行检查代码。本地最大测试为随机 n = 50000 与长链 n = 30000（官方 10^5，完整规模会使题目文件超过约 600 KiB）。",
        "sourceUrl": "https://leetcode.com/problems/find-the-duplicate-number/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int findDuplicate(vector<int>& nums) {\n        return 0; // The repeated value; do not modify nums.\n    }\n};",
        "acmIoSpec": "stdin：第一行数组长度 n + 1；第二行 n + 1 个整数 nums。stdout：一个整数，重复的数。",
    },
    "io": {
        "method": "findDuplicate",
        "params": [{"name": "nums", "type": "int[]"}],
        "returns": "int",
    },
}


def solve(a):
    n = len(a) - 1
    assert n >= 1 and all(1 <= x <= n for x in a), "values out of range"
    rep = [x for x, c in Counter(a).items() if c >= 2]
    assert len(rep) == 1, "need exactly one repeated value"
    return rep[0]


def gen(r, n, extra=0):
    # A permutation of 1..n plus copies of `dup`; `extra` extra copies replace other values.
    dup = r.randint(1, n)
    a = list(range(1, n + 1))
    victims = r.sample([v for v in a if v != dup], extra)
    gone = set(victims)
    a = [dup if v in gone else v for v in a] + [dup]
    r.shuffle(a)
    return a


r = rng(287)
tests = [
    {"id": "t1", "input": [1, 3, 4, 2, 2], "sample": True, "expected": 2},
    {"id": "t2", "input": [3, 1, 3, 4, 2], "sample": True, "expected": 3},
    {"id": "t3", "input": [3, 3, 3, 3, 3], "sample": True, "expected": 3},
    {"id": "minimum", "input": [1, 1], "sample": False},
    {"id": "dup-is-n", "input": [4, 1, 4, 2, 3], "sample": False},
    {"id": "random-50", "input": gen(r, 50), "sample": False},
    {"id": "random-many-copies", "input": gen(r, 200, extra=120), "sample": False},
    {"id": "worst-case-50000", "input": gen(r, 50000), "sample": False},
    # i -> i + 1 chain: the cycle entrance is at the end of a path of length n.
    {"id": "worst-case-30000-long-tail", "input": list(range(1, 30001)) + [30000], "sample": False},
]
write(problem, tests, solve)
