import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-73",
    "title": "矩阵置零",
    "difficulty": "medium",
    "tags": ["数组", "哈希表", "矩阵"],
    "statement": "给定一个 m × n 的整数矩阵 matrix，如果某个元素为 0，就把它所在的整行和整列全部改为 0。要求直接在原矩阵上修改（原地算法），函数无返回值。",
    "en": {
        "title": "Set Matrix Zeroes",
        "tags": [
            "Array",
            "Hash Table",
            "Matrix"
        ],
        "statement": "Given an m × n integer matrix, if an element is 0, set its entire row and entire column to 0. You must modify the original matrix directly (in place); the function returns nothing.",
        "constraints": [
            "m == matrix.length, n == matrix[0].length",
            "1 ≤ m, n ≤ 200",
            "-2^31 ≤ matrix[i][j] ≤ 2^31 - 1"
        ],
        "complexity": {
            "time": "O(m·n)",
            "space": "O(1) extra space (using the first row and column as markers); O(m + n) row/column marker arrays also pass the local tests",
            "officialRequirement": "Official problem requires in-place modification; the follow-up notes that O(m·n) extra space is bad and O(m + n) can still be improved, and expects O(1) constant extra space.",
            "notes": "m and n are the row and column counts; the input matrix does not count toward extra space. First record whether the first row/first column themselves contain 0, then use them to mark the remaining rows and columns. The largest local test is 200 × 200 (the official limit) with element values reduced to a small range to keep the problem bank small; there are also small tests containing INT_MIN/INT_MAX."
        },
        "acmIoSpec": "stdin: first line the row count m; then m lines, each starting with the column count n followed by that row's n integers. stdout: first line m, then m lines, each containing that row's n integers after modification (space-separated, no length prefix)."
    },
    "constraints": [
        "m == matrix.length，n == matrix[0].length",
        "1 ≤ m, n ≤ 200",
        "-2^31 ≤ matrix[i][j] ≤ 2^31 - 1",
    ],
    "complexity": {
        "time": "O(m·n)",
        "space": "O(1) 额外空间（用首行首列做标记）；O(m + n) 的行列标记数组亦可通过本地测试",
        "officialRequirement": "官方要求原地修改；follow-up 指出 O(m·n) 额外空间不好、O(m + n) 仍可改进，期望 O(1) 常数额外空间。",
        "notes": "m、n 为行列数，输入矩阵本身不计入额外空间。需先记录首行/首列自身是否含 0，再用它们标记其余行列。本地最大测试为 200 × 200（官方上限），元素取值缩小到较小范围以控制题库体积，另有含 INT_MIN/INT_MAX 的小测试。",
        "sourceUrl": "https://leetcode.com/problems/set-matrix-zeroes/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    void setZeroes(vector<vector<int>>& matrix) {\n        // Modify matrix in place.\n    }\n};",
        "acmIoSpec": "stdin：第一行行数 m；随后 m 行，每行先写列数 n，再写该行 n 个整数。stdout：第一行 m，随后 m 行，每行为修改后该行的 n 个整数（空格分隔，不写长度）。",
    },
    # The 200 x 200 output matrix is ~110 KiB of text.
    "limits": {"timeLimitMs": 1000, "memoryLimitMb": 128, "outputLimitKb": 256},
    "io": {
        "method": "setZeroes",
        "params": [{"name": "matrix", "type": "int[][]"}],
        "returns": "void",
        "output": "matrix",
    },
}


def solve(mat):
    # Copy-based brute force: collect zero rows/cols first, then rebuild.
    rows = {i for i, row in enumerate(mat) for v in row if v == 0}
    cols = {j for row in mat for j, v in enumerate(row) if v == 0}
    return [[0 if i in rows or j in cols else v for j, v in enumerate(row)] for i, row in enumerate(mat)]


def rand_mat(r, m, n, lo, hi, zeros):
    mat = [[r.randint(lo, hi) or 1 for _ in range(n)] for _ in range(m)]
    for _ in range(zeros):
        mat[r.randrange(m)][r.randrange(n)] = 0
    return mat


r = rng(73)
tests = [
    {"id": "t1", "input": [[1, 1, 1], [1, 0, 1], [1, 1, 1]], "sample": True, "expected": [[1, 0, 1], [0, 0, 0], [1, 0, 1]]},
    {"id": "t2", "input": [[0, 1, 2, 0], [3, 4, 5, 2], [1, 3, 1, 5]], "sample": True, "expected": [[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]]},
    {"id": "single-zero", "input": [[0]], "sample": False},
    {"id": "single-nonzero", "input": [[-7]], "sample": False},
    {"id": "no-zeros", "input": [[1, 2], [3, 4]], "sample": False},
    {"id": "corner-zero", "input": [[1, 2, 3], [4, 5, 6], [7, 8, 0]], "sample": False},
    {"id": "first-row-zero-only", "input": [[1, 0, 3], [4, 5, 6], [7, 8, 9]], "sample": False},
    {"id": "first-col-zero-only", "input": [[1, 2, 3], [0, 5, 6], [7, 8, 9]], "sample": False},
    {"id": "row-vector", "input": [[5, 0, 3, 2, 0, 1]], "sample": False},
    {"id": "col-vector", "input": [[5], [0], [3]], "sample": False},
    {"id": "extremes", "input": [[-2147483648, 2147483647, 1], [2147483647, 0, -2147483648], [1, -2147483648, 2147483647]], "sample": False},
    {"id": "all-zero", "input": [[0] * 5 for _ in range(4)], "sample": False},
    {"id": "random-30x40", "input": rand_mat(r, 30, 40, -2147483648, 2147483647, 6), "sample": False},
    # Official max dimensions; small values keep the JSON small.
    {"id": "max-size-200x200", "input": rand_mat(r, 200, 200, -99, 99, 12), "sample": False},
]
write(problem, tests, solve)
