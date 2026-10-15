import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-240",
    "title": "搜索二维矩阵 II",
    "difficulty": "medium",
    "tags": ["数组", "二分查找", "分治", "矩阵"],
    "statement": "给定一个 m × n 的整数矩阵 matrix：每一行从左到右严格递增，每一列从上到下严格递增。再给一个整数 target，判断 target 是否出现在矩阵中，出现返回 true，否则返回 false。",
    "en": {
        "title": "Search a 2D Matrix II",
        "tags": [
            "Array",
            "Binary Search",
            "Divide and Conquer",
            "Matrix"
        ],
        "statement": "Given an m × n integer matrix where each row is strictly increasing from left to right and each column is strictly increasing from top to bottom, and an integer target, determine whether target appears in the matrix: return true if it does, otherwise false.",
        "constraints": [
            "m == matrix.length, n == matrix[i].length",
            "1 ≤ m, n ≤ 300",
            "-10^9 ≤ matrix[i][j] ≤ 10^9",
            "Each row is sorted in ascending order from left to right, and each column from top to bottom",
            "-10^9 ≤ target ≤ 10^9"
        ],
        "complexity": {
            "time": "O(m + n) (staircase search starting from the top-right or bottom-left corner); binary search per row is O(m log n)",
            "space": "O(1) extra space",
            "officialRequirement": "Officially only asks for an \"efficient\" algorithm, with no specific complexity requirement.",
            "notes": "m and n are the numbers of rows and columns; the input matrix is not counted as extra space. Starting at the top-right: if the current value is greater than target, move one column left; if smaller, move one row down; each step eliminates a row or a column. The largest local test is 300 × 300 (official limit), with element values compressed into a small range to keep the problem bank small; reading the input is already O(m·n), so local tests cannot distinguish O(m + n) from scanning every element."
        },
        "acmIoSpec": "stdin: line 1 is the number of rows m; then m lines, each starting with the column count n followed by that row's n integers; the last line is target. stdout: true or false."
    },
    "constraints": [
        "m == matrix.length，n == matrix[i].length",
        "1 ≤ m, n ≤ 300",
        "-10^9 ≤ matrix[i][j] ≤ 10^9",
        "每行元素从左到右升序，每列元素从上到下升序",
        "-10^9 ≤ target ≤ 10^9",
    ],
    "complexity": {
        "time": "O(m + n)（从右上角或左下角出发的阶梯搜索）；逐行二分为 O(m log n)",
        "space": "O(1) 额外空间",
        "officialRequirement": "官方只要求写出“高效”的算法，未给出具体复杂度要求。",
        "notes": "m、n 为行列数，输入矩阵不计入额外空间。从右上角出发：当前值大于 target 则左移一列，小于则下移一行，每步排除一行或一列。本地最大测试为 300 × 300（官方上限），元素取值压缩到较小范围以控制题库体积；读入本身已是 O(m·n)，本地测试无法区分 O(m + n) 与逐元素扫描。",
        "sourceUrl": "https://leetcode.com/problems/search-a-2d-matrix-ii/",
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
    return any(d["target"] in row for row in d["matrix"])


def young(r, m, n, base, step):
    a = [[0] * n for _ in range(m)]
    for i in range(m):
        for j in range(n):
            prev = max(a[i - 1][j] if i else base - 1, a[i][j - 1] if j else base - 1)
            a[i][j] = prev + r.randint(1, step)
    return a


r = rng(240)
ex = [[1, 4, 7, 11, 15], [2, 5, 8, 12, 19], [3, 6, 9, 16, 22], [10, 13, 14, 17, 24], [18, 21, 23, 26, 30]]
big = young(r, 300, 300, -300, 2)
mid = young(r, 40, 60, -10**9, 10**7)
missing_big = next(v for v in range(big[150][150] - 1, -10**6, -1) if not any(v in row for row in big))
tests = [
    {"id": "t1", "input": {"matrix": ex, "target": 5}, "sample": True, "expected": True},
    {"id": "t2", "input": {"matrix": ex, "target": 20}, "sample": True, "expected": False},
    {"id": "single-hit", "input": {"matrix": [[-5]], "target": -5}, "sample": False},
    {"id": "single-miss", "input": {"matrix": [[-5]], "target": 5}, "sample": False},
    {"id": "below-min", "input": {"matrix": ex, "target": 0}, "sample": False},
    {"id": "above-max", "input": {"matrix": ex, "target": 31}, "sample": False},
    {"id": "top-right", "input": {"matrix": ex, "target": 15}, "sample": False},
    {"id": "bottom-left", "input": {"matrix": ex, "target": 18}, "sample": False},
    {"id": "one-row", "input": {"matrix": [[-10**9, -3, 0, 7, 10**9]], "target": 10**9}, "sample": False},
    {"id": "one-col-miss", "input": {"matrix": [[1], [3], [5], [7]], "target": 4}, "sample": False},
    {"id": "random-hit", "input": {"matrix": mid, "target": mid[17][41]}, "sample": False},
    {"id": "random-miss", "input": {"matrix": mid, "target": mid[17][41] + 1 if not any(mid[17][41] + 1 in row for row in mid) else mid[0][0] - 1}, "sample": False},
    # Official max dimensions; small strictly increasing values keep the JSON small.
    {"id": "max-size-300x300", "input": {"matrix": big, "target": missing_big}, "sample": False},
]
write(problem, tests, solve)
