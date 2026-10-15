import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-994",
    "title": "腐烂的橘子",
    "difficulty": "medium",
    "tags": ["广度优先搜索", "数组", "矩阵"],
    "statement": "在 m × n 网格 grid 中，0 表示空格，1 表示新鲜橘子，2 表示腐烂橘子。每过一分钟，与腐烂橘子上下左右相邻的新鲜橘子都会腐烂。求所有新鲜橘子都腐烂所需的最少分钟数；若不可能全部腐烂，返回 -1（没有新鲜橘子时返回 0）。",
    "constraints": [
        "m == grid.length，n == grid[i].length",
        "1 ≤ m, n ≤ 10",
        "grid[i][j] 为 0、1 或 2",
    ],
    "complexity": {
        "time": "O(m·n)（本地训练目标）",
        "space": "O(m·n)（BFS 队列；输入网格不计入）",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "m、n 为网格行列数。把所有腐烂橘子同时作为起点做多源 BFS，层数即分钟数；BFS 结束后仍有新鲜橘子则返回 -1。",
        "sourceUrl": "https://leetcode.com/problems/rotting-oranges/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int orangesRotting(vector<vector<int>>& grid) {\n        return -1; // Minutes until no fresh orange remains, or -1.\n    }\n};",
        "acmIoSpec": "stdin：第一行行数 m；随后 m 行，每行先写该行长度 n，再写 n 个整数（0/1/2）。stdout：一个整数，所需最少分钟数或 -1。",
    },
    "io": {
        "method": "orangesRotting",
        "params": [{"name": "grid", "type": "int[][]"}],
        "returns": "int",
    },
}


def solve(grid):
    # Minute-by-minute simulation: independent of the BFS C++ solution.
    g = [row[:] for row in grid]
    m, n = len(g), len(g[0])
    minutes = 0
    while True:
        spread = [(i, j) for i in range(m) for j in range(n) if g[i][j] == 1 and any(
            0 <= a < m and 0 <= b < n and g[a][b] == 2
            for a, b in ((i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)))]
        if not spread:
            break
        for i, j in spread:
            g[i][j] = 2
        minutes += 1
    return -1 if any(1 in row for row in g) else minutes


def rand_grid(r, m, n, w):
    return [r.choices([0, 1, 2], weights=w, k=n) for _ in range(m)]


r = rng(994)
snake = [[1] * 10 for _ in range(10)]
for i in range(1, 10, 2):
    for j in range(10):
        snake[i][j] = 0
    snake[i][9 if i % 4 == 1 else 0] = 1
snake[0][0] = 2
tests = [
    {"id": "t1", "sample": True, "expected": 4, "input": [[2, 1, 1], [1, 1, 0], [0, 1, 1]]},
    {"id": "t2", "sample": True, "expected": -1, "input": [[2, 1, 1], [0, 1, 1], [1, 0, 1]]},
    {"id": "t3", "sample": True, "expected": 0, "input": [[0, 2]]},
    {"id": "single-empty", "sample": False, "input": [[0]]},
    {"id": "single-fresh", "sample": False, "input": [[1]]},
    {"id": "single-rotten", "sample": False, "input": [[2]]},
    {"id": "fresh-no-rotten", "sample": False, "input": [[1, 1], [1, 1]]},
    {"id": "two-sources", "sample": False, "input": [[2, 1, 1, 1, 1, 1, 2]]},
    {"id": "random-6x7", "sample": False, "input": rand_grid(r, 6, 7, [2, 6, 1])},
    {"id": "random-unreachable", "sample": False, "input": rand_grid(r, 10, 10, [4, 5, 1])},
    {"id": "max-size-10x10", "sample": False, "input": snake},
    {"id": "max-size-10x10-dense", "sample": False, "input": [[2] + [1] * 9] + [[1] * 10 for _ in range(9)]},
]
write(problem, tests, solve)
