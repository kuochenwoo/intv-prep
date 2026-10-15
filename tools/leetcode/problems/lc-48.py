import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-48",
    "title": "旋转图像",
    "difficulty": "medium",
    "tags": ["数组", "数学", "矩阵"],
    "statement": "给定一个 n × n 的整数矩阵 matrix 表示一幅图像，把它顺时针旋转 90 度。必须直接修改传入的矩阵，不能另外分配一个矩阵来完成旋转；函数无返回值。",
    "en": {
        "title": "Rotate Image",
        "tags": [
            "Array",
            "Math",
            "Matrix"
        ],
        "statement": "Given an n × n integer matrix representing an image, rotate it 90 degrees clockwise. You must modify the given matrix directly and may not allocate another matrix to perform the rotation; the function returns nothing.",
        "constraints": [
            "n == matrix.length == matrix[i].length",
            "1 ≤ n ≤ 20",
            "-1000 ≤ matrix[i][j] ≤ 1000"
        ],
        "complexity": {
            "time": "O(n²)",
            "space": "O(1) extra space",
            "officialRequirement": "Official problem requires rotating in place without using another matrix (i.e. O(1) extra space).",
            "notes": "n is the side length; the input matrix does not count toward extra space. Approach 1: transpose along the main diagonal, then reverse each row. Approach 2: rotate layer by layer, cycling groups of four elements. The local judge only compares the resulting matrix and cannot detect whether an extra matrix was used."
        },
        "acmIoSpec": "stdin: first line n; then n lines, each starting with n followed by that row's n integers. stdout: first line n, then n lines, each containing the n integers of that row after rotation (space-separated, no length prefix)."
    },
    "constraints": [
        "n == matrix.length == matrix[i].length",
        "1 ≤ n ≤ 20",
        "-1000 ≤ matrix[i][j] ≤ 1000",
    ],
    "complexity": {
        "time": "O(n²)",
        "space": "O(1) 额外空间",
        "officialRequirement": "官方要求原地旋转，不得使用另一个矩阵（即 O(1) 额外空间）。",
        "notes": "n 为边长，输入矩阵本身不计入额外空间。做法一：先沿主对角线转置，再把每行左右翻转；做法二：按层四元素一组循环交换。本地判题只比较结果矩阵，无法检测是否借助了额外矩阵。",
        "sourceUrl": "https://leetcode.com/problems/rotate-image/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    void rotate(vector<vector<int>>& matrix) {\n        // Rotate matrix 90 degrees clockwise in place.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；随后 n 行，每行先写 n，再写该行 n 个整数。stdout：第一行 n，随后 n 行，每行为旋转后该行的 n 个整数（空格分隔，不写长度）。",
    },
    "io": {
        "method": "rotate",
        "params": [{"name": "matrix", "type": "int[][]"}],
        "returns": "void",
        "output": "matrix",
    },
}


def solve(mat):
    # Build a fresh matrix by index mapping: new[i][j] = old[n-1-j][i].
    n = len(mat)
    return [[mat[n - 1 - j][i] for j in range(n)] for i in range(n)]


def rand_mat(r, n):
    return [[r.randint(-1000, 1000) for _ in range(n)] for _ in range(n)]


r = rng(48)
tests = [
    {"id": "t1", "input": [[1, 2, 3], [4, 5, 6], [7, 8, 9]], "sample": True, "expected": [[7, 4, 1], [8, 5, 2], [9, 6, 3]]},
    {"id": "t2", "input": [[5, 1, 9, 11], [2, 4, 8, 10], [13, 3, 6, 7], [15, 14, 12, 16]], "sample": True,
     "expected": [[15, 13, 2, 5], [14, 3, 4, 1], [12, 6, 8, 9], [16, 7, 10, 11]]},
    {"id": "single", "input": [[-1000]], "sample": False},
    {"id": "two", "input": [[1, 2], [3, 4]], "sample": False},
    {"id": "odd-5", "input": [[i * 5 + j for j in range(5)] for i in range(5)], "sample": False},
    {"id": "duplicates", "input": [[r.choice([-1, 0, 1]) for _ in range(6)] for _ in range(6)], "sample": False},
    {"id": "random-11", "input": rand_mat(r, 11), "sample": False},
    {"id": "max-size-20", "input": rand_mat(r, 20), "sample": False},
]
write(problem, tests, solve)
