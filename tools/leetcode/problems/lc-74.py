import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-74",
    "title": "搜索二维矩阵",
    "difficulty": "medium",
    "tags": ["数组", "二分查找", "矩阵"],
    "statement": "给定一个 m × n 的整数矩阵 matrix：每一行按非严格递增排列，且每一行的第一个数都大于上一行的最后一个数。给定整数 target，判断它是否在矩阵中，在则返回 true，否则返回 false。",
    "en": {
        "title": "Search a 2D Matrix",
        "tags": [
            "Array",
            "Binary Search",
            "Matrix"
        ],
        "statement": "Given an m × n integer matrix in which each row is sorted in non-decreasing order and the first number of each row is greater than the last number of the previous row, and an integer target, determine whether target is in the matrix: return true if it is, otherwise false.",
        "constraints": [
            "m == matrix.length, n == matrix[i].length",
            "1 ≤ m, n ≤ 100",
            "-10^4 ≤ matrix[i][j], target ≤ 10^4"
        ],
        "complexity": {
            "time": "O(log(m·n))",
            "space": "O(1) extra space",
            "officialRequirement": "Official problem requires O(log(m·n)) time complexity.",
            "notes": "m and n are the row and column counts; the input matrix does not count toward extra space. Flattened row by row, the matrix is a sorted array where index k maps to matrix[k / n][k % n], so binary search directly; alternatively, binary search the row and then the column. The local judge needs O(m·n) to read input, so it cannot distinguish binary search from a linear scan by running time."
        },
        "acmIoSpec": "stdin: first line the row count m; then m lines, each starting with the column count n followed by that row's n integers; the last line is target. stdout: true or false."
    },
    "constraints": [
        "m == matrix.length，n == matrix[i].length",
        "1 ≤ m, n ≤ 100",
        "-10^4 ≤ matrix[i][j], target ≤ 10^4",
    ],
    "complexity": {
        "time": "O(log(m·n))",
        "space": "O(1) 额外空间",
        "officialRequirement": "官方要求 O(log(m·n)) 时间复杂度。",
        "notes": "m、n 为行列数，输入矩阵不计入额外空间。整个矩阵按行展开后是一个有序数组，下标 k 对应 matrix[k / n][k % n]，直接二分；也可先二分行再二分列。本地判题需 O(m·n) 读入，无法从耗时上区分二分与线性扫描。",
        "sourceUrl": "https://leetcode.com/problems/search-a-2d-matrix/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    bool searchMatrix(vector<vector<int>>& matrix, int target) {\n        return false; // Whether target occurs in matrix.\n    }\n};",
        "acmIoSpec": "stdin：第一行行数 m；随后 m 行，每行先写列数 n，再写该行 n 个整数；最后一行为 target。stdout：true 或 false。",
    },
    "io": {
        "method": "searchMatrix",
        "params": [{"name": "matrix", "type": "int[][]"}, {"name": "target", "type": "int"}],
        "returns": "bool",
    },
}


def solve(d):
    return any(v == d["target"] for row in d["matrix"] for v in row)


def build(r, m, n, lo, hi):
    vals = sorted(r.randint(lo, hi) for _ in range(m * n))
    rows = [vals[i * n:(i + 1) * n] for i in range(m)]
    # Enforce row[0] > previous row's last element by bumping whole later rows.
    for i in range(1, m):
        if rows[i][0] <= rows[i - 1][-1]:
            shift = rows[i - 1][-1] - rows[i][0] + 1
            rows[i] = [v + shift for v in rows[i]]
    assert all(lo <= v <= 10**4 for row in rows for v in row)
    return rows


def miss(mat, start, step):
    v = start
    while any(v in row for row in mat):
        v += step
    return v


r = rng(74)
ex = [[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]]
mid = build(r, 20, 30, -10**4, 0)
big = build(r, 100, 100, -10**4, -2000)
tests = [
    {"id": "t1", "input": {"matrix": ex, "target": 3}, "sample": True, "expected": True},
    {"id": "t2", "input": {"matrix": ex, "target": 13}, "sample": True, "expected": False},
    {"id": "single-hit", "input": {"matrix": [[1]], "target": 1}, "sample": False},
    {"id": "single-miss", "input": {"matrix": [[1]], "target": 2}, "sample": False},
    {"id": "first", "input": {"matrix": ex, "target": 1}, "sample": False},
    {"id": "last", "input": {"matrix": ex, "target": 60}, "sample": False},
    {"id": "below-min", "input": {"matrix": ex, "target": -10000}, "sample": False},
    {"id": "above-max", "input": {"matrix": ex, "target": 10000}, "sample": False},
    {"id": "gap-between-rows", "input": {"matrix": ex, "target": 21}, "sample": False},
    {"id": "one-col", "input": {"matrix": [[-3], [0], [4], [9]], "target": 4}, "sample": False},
    {"id": "one-row-dups", "input": {"matrix": [[2, 2, 2, 5, 5, 8]], "target": 5}, "sample": False},
    {"id": "random-hit", "input": {"matrix": mid, "target": mid[13][7]}, "sample": False},
    {"id": "random-miss", "input": {"matrix": mid, "target": miss(mid, mid[13][7], 1)}, "sample": False},
    {"id": "max-size-100x100", "input": {"matrix": big, "target": miss(big, big[50][50], -1)}, "sample": False},
]
write(problem, tests, solve)
