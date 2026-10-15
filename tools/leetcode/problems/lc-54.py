import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-54",
    "title": "螺旋矩阵",
    "difficulty": "medium",
    "tags": ["数组", "矩阵", "模拟"],
    "statement": "给定一个 m 行 n 列的整数矩阵 matrix，从左上角出发，按顺时针螺旋顺序（向右、向下、向左、向上循环，逐层向内）依次访问所有元素，返回访问得到的序列。",
    "constraints": [
        "m == matrix.length，n == matrix[i].length",
        "1 ≤ m, n ≤ 10",
        "-100 ≤ matrix[i][j] ≤ 100",
    ],
    "complexity": {
        "time": "O(m·n)",
        "space": "O(1) 额外空间（不计返回数组）；用 visited 标记的模拟为 O(m·n)",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "m、n 为行列数。维护上下左右四条边界，每走完一条边就收缩对应边界；注意单行或单列时不要重复访问。",
        "sourceUrl": "https://leetcode.com/problems/spiral-matrix/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    vector<int> spiralOrder(vector<vector<int>>& matrix) {\n        return {}; // Elements in clockwise spiral order.\n    }\n};",
        "acmIoSpec": "stdin：第一行行数 m；随后 m 行，每行先写列数 n，再写该行 n 个整数。stdout：一行，按螺旋顺序输出的 m·n 个整数，空格分隔。",
    },
    "io": {
        "method": "spiralOrder",
        "params": [{"name": "matrix", "type": "int[][]"}],
        "returns": "int[]",
    },
}


def solve(mat):
    # Peel the first row, then rotate the rest counter-clockwise; repeat.
    mat = [list(row) for row in mat]
    out = []
    while mat:
        out.extend(mat.pop(0))
        mat = [list(col) for col in zip(*mat)][::-1]
    return out


def rand_mat(r, m, n):
    return [[r.randint(-100, 100) for _ in range(n)] for _ in range(m)]


r = rng(54)
tests = [
    {"id": "t1", "input": [[1, 2, 3], [4, 5, 6], [7, 8, 9]], "sample": True, "expected": [1, 2, 3, 6, 9, 8, 7, 4, 5]},
    {"id": "t2", "input": [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]], "sample": True, "expected": [1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]},
    {"id": "single", "input": [[-100]], "sample": False},
    {"id": "one-row", "input": [[1, 2, 3, 4, 5]], "sample": False},
    {"id": "one-col", "input": [[1], [2], [3], [4]], "sample": False},
    {"id": "two-by-two", "input": [[1, 2], [3, 4]], "sample": False},
    {"id": "tall-3x2", "input": [[1, 2], [3, 4], [5, 6]], "sample": False},
    {"id": "wide-2x5", "input": rand_mat(r, 2, 5), "sample": False},
    {"id": "tall-7x3", "input": rand_mat(r, 7, 3), "sample": False},
    {"id": "wide-4x9", "input": rand_mat(r, 4, 9), "sample": False},
    {"id": "duplicates", "input": [[7] * 6 for _ in range(5)], "sample": False},
    {"id": "random-9x10", "input": rand_mat(r, 9, 10), "sample": False},
    {"id": "max-size-10x10", "input": rand_mat(r, 10, 10), "sample": False},
]
write(problem, tests, solve)
