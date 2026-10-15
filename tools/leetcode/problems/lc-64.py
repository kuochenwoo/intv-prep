import sys, pathlib, heapq
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-64",
    "title": "最小路径和",
    "difficulty": "medium",
    "tags": ["数组", "动态规划", "矩阵"],
    "statement": "给定 m × n 的非负整数网格 grid，从左上角走到右下角，每步只能向右或向下，求路径上所有数字之和的最小值。",
    "constraints": ["1 ≤ m, n ≤ 200；0 ≤ grid[i][j] ≤ 200"],
    "complexity": {
        "time": "O(m · n)（本地训练目标）",
        "space": "O(n) 额外空间（滚动数组）；直接在 grid 上原地累加可做到 O(1)",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "m、n 为网格行数与列数，输入网格不计入额外空间。dp[i][j] = grid[i][j] + min(上方, 左方)。",
        "sourceUrl": "https://leetcode.com/problems/minimum-path-sum/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int minPathSum(vector<vector<int>>& grid) {\n        return 0; // Minimum sum along a right/down path.\n    }\n};",
        "acmIoSpec": "stdin：第一行行数 m；随后 m 行，每行先写列数 n 再写 n 个整数。stdout：一个整数，最小路径和。",
    },
    "io": {
        "method": "minPathSum",
        "params": [{"name": "grid", "type": "int[][]"}],
        "returns": "int",
    },
}


def solve(g):
    # Dijkstra on the DAG of right/down moves: independent of the C++ DP.
    m, n = len(g), len(g[0])
    dist = {(0, 0): g[0][0]}
    pq = [(g[0][0], 0, 0)]
    while pq:
        d, i, j = heapq.heappop(pq)
        if (i, j) == (m - 1, n - 1):
            return d
        if d > dist[(i, j)]:
            continue
        for a, b in ((i + 1, j), (i, j + 1)):
            if a < m and b < n and d + g[a][b] < dist.get((a, b), float("inf")):
                dist[(a, b)] = d + g[a][b]
                heapq.heappush(pq, (d + g[a][b], a, b))


r = rng(64)
grid = lambda m, n, hi: [[r.randint(0, hi) for _ in range(n)] for _ in range(m)]
tests = [
    {"id": "t1", "input": [[1, 3, 1], [1, 5, 1], [4, 2, 1]], "sample": True, "expected": 7},
    {"id": "t2", "input": [[1, 2, 3], [4, 5, 6]], "sample": True, "expected": 12},
    {"id": "single-cell", "input": [[0]], "sample": False},
    {"id": "single-row", "input": [[3, 1, 4, 1, 5, 9]], "sample": False},
    {"id": "single-column", "input": [[2], [7], [1], [8]], "sample": False},
    {"id": "random-15x20", "input": grid(15, 20, 9), "sample": False},
    {"id": "max-size-200x200", "input": grid(200, 200, 200), "sample": False},
    {"id": "max-size-200x200-all-max", "input": [[200] * 200 for _ in range(200)], "sample": False},
]
write(problem, tests, solve)
