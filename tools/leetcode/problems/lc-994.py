import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-994",
    "title": "腐烂的橘子",
    "difficulty": "medium",
    "tags": ["广度优先搜索", "数组", "矩阵"],
    "statement": "在 m × n 网格 grid 中，0 表示空格，1 表示新鲜橘子，2 表示腐烂橘子。每过一分钟，与腐烂橘子上下左右相邻的新鲜橘子都会腐烂。求所有新鲜橘子都腐烂所需的最少分钟数；若不可能全部腐烂，返回 -1（没有新鲜橘子时返回 0）。",
    "en": {
        "title": "Rotting Oranges",
        "tags": [
            "Breadth-First Search",
            "Array",
            "Matrix"
        ],
        "statement": "In an m × n grid, 0 is an empty cell, 1 is a fresh orange and 2 is a rotten orange. Every minute, each fresh orange that is 4-directionally adjacent (up, down, left, right) to a rotten orange becomes rotten. Find the minimum number of minutes until all fresh oranges are rotten; if this is impossible, return -1 (return 0 if there are no fresh oranges).",
        "constraints": [
            "m == grid.length, n == grid[i].length",
            "1 ≤ m, n ≤ 10",
            "grid[i][j] is 0, 1 or 2"
        ],
        "complexity": {
            "time": "O(m·n) (local training target)",
            "space": "O(m·n) (BFS queue; the input grid is not counted)",
            "officialRequirement": "No official complexity requirement.",
            "notes": "m and n are the grid's row and column counts. Run a multi-source BFS starting from all rotten oranges at once; the number of levels is the number of minutes. If any fresh orange remains after BFS, return -1."
        },
        "acmIoSpec": "stdin: line 1 is the number of rows m; then m lines, each starting with that row's length n followed by n integers (0/1/2). stdout: one integer, the minimum number of minutes or -1."
    },
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
