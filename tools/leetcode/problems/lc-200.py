import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-200",
    "title": "岛屿数量",
    "difficulty": "medium",
    "tags": ["深度优先搜索", "广度优先搜索", "并查集", "数组", "矩阵"],
    "statement": "给定一个由字符 '1'（陆地）和 '0'（水）组成的 m × n 二维网格 grid，统计其中岛屿的个数。岛屿由上下左右相邻的陆地连接而成，网格四周都被水包围。",
    "en": {'title': 'Number of Islands',
     'tags': ['Depth-First Search', 'Breadth-First Search', 'Union Find', 'Array', 'Matrix'],
     'statement': "Given an m × n 2D grid of characters '1' (land) and '0' (water), count the number "
                  'of islands. An island is formed by land cells connected horizontally or vertically, '
                  'and the grid is surrounded by water on all four sides.',
     'constraints': ['m == grid.length, n == grid[i].length',
                     '1 ≤ m, n ≤ 300',
                     "grid[i][j] is '0' or '1'"],
     'complexity': {'time': 'O(m·n) (local training target)',
                    'space': 'O(m·n) worst case: DFS recursion stack or BFS queue (recursion stack '
                             'counted); union-find is also O(m·n); the input grid is not counted',
                    'officialRequirement': 'No official complexity requirement.',
                    'notes': "m and n are the grid's row and column counts. Scan cell by cell; on each "
                             'unvisited land cell, increment the count and sink the whole island with '
                             'DFS/BFS/union-find, so each cell is visited a constant number of times. '
                             'When the grid is all land, recursive DFS depth can reach m·n.'},
     'acmIoSpec': 'stdin: line 1 is m; then m lines, each a string of 0/1 of length n representing one '
                  'grid row. stdout: one integer, the number of islands.'},
    "constraints": [
        "m == grid.length，n == grid[i].length",
        "1 ≤ m, n ≤ 300",
        "grid[i][j] 为 '0' 或 '1'",
    ],
    "complexity": {
        "time": "O(m·n)（本地训练目标）",
        "space": "O(m·n) 最坏情况：DFS 递归栈或 BFS 队列（递归栈计入），并查集同为 O(m·n)；输入网格不计入",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "m、n 为网格行列数。逐格扫描，遇到未访问的陆地就计数并用 DFS/BFS/并查集淹没整座岛屿，每格最多访问常数次。全为陆地时递归 DFS 深度可达 m·n。",
        "sourceUrl": "https://leetcode.com/problems/number-of-islands/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int numIslands(vector<vector<char>>& grid) {\n        return 0; // Number of islands.\n    }\n};",
        "acmIoSpec": "stdin：第一行 m；随后 m 行，每行一个长度为 n 的由 0/1 组成的字符串，表示网格的一行。stdout：一个整数，岛屿数量。",
    },
    "io": {
        "method": "numIslands",
        "params": [{"name": "grid", "type": "char[][]"}],
        "returns": "int",
    },
}


def solve(grid):
    # Union-find over land cells: independent of the DFS/BFS C++ solutions.
    m, n = len(grid), len(grid[0])
    parent = list(range(m * n))

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    land = 0
    for i in range(m):
        for j in range(n):
            if grid[i][j] != "1":
                continue
            land += 1
            for a, b in ((i - 1, j), (i, j - 1)):
                if a >= 0 and b >= 0 and grid[a][b] == "1":
                    x, y = find(i * n + j), find(a * n + b)
                    if x != y:
                        parent[x] = y
                        land -= 1
    return land


def g(rows):
    return [list(r) for r in rows]


def rand_grid(r, m, n, p):
    return [["1" if r.random() < p else "0" for _ in range(n)] for _ in range(m)]


def serpentine(m, n):
    # One long winding island: every other row full, joined at alternating ends.
    grid = [["0"] * n for _ in range(m)]
    for i in range(m):
        if i % 2 == 0:
            grid[i] = ["1"] * n
        elif i % 4 == 1:
            grid[i][n - 1] = "1"
        else:
            grid[i][0] = "1"
    return grid


r = rng(200)
tests = [
    {"id": "t1", "sample": True, "expected": 1,
     "input": g(["11110", "11010", "11000", "00000"])},
    {"id": "t2", "sample": True, "expected": 3,
     "input": g(["11000", "11000", "00100", "00011"])},
    {"id": "single-water", "sample": False, "input": g(["0"])},
    {"id": "single-land", "sample": False, "input": g(["1"])},
    {"id": "one-row", "sample": False, "input": g(["1011011101"])},
    {"id": "one-column", "sample": False, "input": [[c] for c in "1101001"]},
    {"id": "checkerboard-20", "sample": False,
     "input": [["1" if (i + j) % 2 == 0 else "0" for j in range(20)] for i in range(20)]},
    {"id": "ring-with-lake", "sample": False, "input": g(["11111", "10001", "10101", "10001", "11111"])},
    {"id": "random-40x60", "sample": False, "input": rand_grid(r, 40, 60, 0.45)},
    {"id": "serpentine-60x80", "sample": False, "input": serpentine(60, 80)},
    # Density just above the percolation threshold: one giant winding island plus many small ones.
    {"id": "max-size-300x300", "sample": False, "input": rand_grid(r, 300, 300, 0.6)},
]
write(problem, tests, solve)
