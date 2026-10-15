import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-45",
    "title": "跳跃游戏 II",
    "difficulty": "medium",
    "tags": ["贪心", "数组", "动态规划"],
    "statement": "给定非负整数数组 nums，从下标 0 出发，站在下标 i 时一次可以向右跳 1 到 nums[i] 步（不越过数组末尾）。题目保证一定能到达最后一个下标，求到达所需的最少跳跃次数。",
    "en": {
        "title": "Jump Game II",
        "tags": [
            "Greedy",
            "Array",
            "Dynamic Programming"
        ],
        "statement": "Given an array of non-negative integers nums, start at index 0; standing at index i you may jump right by 1 to nums[i] steps in one move (without going past the end of the array). It is guaranteed that the last index is reachable; return the minimum number of jumps needed to reach it.",
        "constraints": [
            "1 ≤ nums.length ≤ 10000; 0 ≤ nums[i] ≤ 1000",
            "It is guaranteed that nums[n-1] is reachable"
        ],
        "complexity": {
            "time": "O(n) (local training target)",
            "space": "O(1) extra space",
            "officialRequirement": "No official complexity requirement.",
            "notes": "n is the array length. Greedy layered by jump count (implicit BFS): track the right boundary of the current layer and the farthest index reachable by the next layer. An O(n·max(nums)) DP takes about 10⁷ operations within these limits and also passes."
        },
        "acmIoSpec": "stdin: first line n; second line n integers nums. stdout: one integer, the minimum number of jumps."
    },
    "constraints": ["1 ≤ nums.length ≤ 10000；0 ≤ nums[i] ≤ 1000", "保证可以到达 nums[n-1]"],
    "complexity": {
        "time": "O(n)（本地训练目标）",
        "space": "O(1) 额外空间",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为数组长度。按跳跃次数分层的贪心（隐式 BFS）：记录当前层的右边界与下一层能到的最远处。O(n·max(nums)) 的 DP 在本题范围内约 10⁷ 次操作，也能通过。",
        "sourceUrl": "https://leetcode.com/problems/jump-game-ii/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int jump(vector<int>& nums) {\n        return 0; // Minimum number of jumps to reach the last index.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数 nums。stdout：一个整数，最少跳跃次数。",
    },
    "io": {
        "method": "jump",
        "params": [{"name": "nums", "type": "int[]"}],
        "returns": "int",
    },
}


def solve(nums):
    # Explicit BFS over indices, each index enqueued once.
    n = len(nums)
    dist = [-1] * n
    dist[0] = 0
    queue, head, frontier = [0], 0, 0  # indices <= frontier are already discovered
    while head < len(queue):
        i = queue[head]; head += 1
        for j in range(frontier + 1, min(n - 1, i + nums[i]) + 1):
            dist[j] = dist[i] + 1
            queue.append(j)
        frontier = max(frontier, min(n - 1, i + nums[i]))
    assert dist[-1] >= 0, "input must be reachable"
    return dist[-1]


r = rng(45)


def reachable(n, hi):
    return [r.randint(1, hi) for _ in range(n)]


tests = [
    {"id": "t1", "input": [2, 3, 1, 1, 4], "sample": True, "expected": 2},
    {"id": "t2", "input": [2, 3, 0, 1, 4], "sample": True, "expected": 2},
    {"id": "single", "input": [0], "sample": False},
    {"id": "one-jump", "input": [5, 0, 0, 0], "sample": False},
    {"id": "zeros-skipped", "input": [3, 0, 0, 2, 0, 1, 0], "sample": False},
    {"id": "random-small", "input": reachable(50, 4), "sample": False},
    {"id": "random-1000", "input": reachable(1000, 30), "sample": False},
    {"id": "max-size-10000-ones", "input": [1] * 10000, "sample": False},
    {"id": "max-size-10000-big", "input": [1000] * 10000, "sample": False},
    {"id": "max-size-10000-random", "input": reachable(10000, 1000), "sample": False},
]
write(problem, tests, solve)
