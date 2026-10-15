import sys, pathlib, itertools
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write

problem = {
    "id": "lc-51",
    "title": "N 皇后",
    "difficulty": "hard",
    "tags": ["数组", "回溯"],
    "statement": "在 n × n 棋盘上放置 n 个皇后，使任意两个皇后都不在同一行、同一列或同一条斜线上。返回所有不同的放置方案；每个方案用 n 个长度为 n 的字符串表示棋盘，'Q' 表示皇后，'.' 表示空位。方案之间的顺序任意。",
    "en": {
        "title": "N-Queens",
        "tags": [
            "Array",
            "Backtracking"
        ],
        "statement": "Place n queens on an n × n chessboard so that no two queens share a row, column, or diagonal. Return all distinct placements; each placement is represented as n strings of length n describing the board, with 'Q' for a queen and '.' for an empty square. Placements may be returned in any order.",
        "constraints": [
            "1 ≤ n ≤ 9"
        ],
        "complexity": {
            "time": "Backtracking on the order of O(n!) (local training target; O(1) conflict checks using flags for columns and both diagonals)",
            "space": "O(n) extra space (flag arrays and recursion stack, recursion stack counted; output boards not counted)",
            "officialRequirement": "No official complexity requirement.",
            "notes": "n is the board side length. Place queens row by row, pruning with three sets of flags for col, row+col and row-col; for n = 9 there are 352 solutions. The judge only ignores the order between placements; the row order within a placement is fixed."
        },
        "acmIoSpec": "stdin: one line with integer n. stdout: first line the number of placements P; then each placement in turn: a line with n, followed by n lines, each a string of length n made of 'Q' and '.' (placements in any order)."
    },
    "constraints": ["1 ≤ n ≤ 9"],
    "complexity": {
        "time": "O(n!) 量级的回溯（本地训练目标；用列与两条对角线的标记做 O(1) 冲突判断）",
        "space": "O(n) 额外空间（标记数组与递归栈，计入递归栈；输出的棋盘不计入）",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为棋盘边长。逐行放置皇后，用 col、row+col、row-col 三组标记剪枝；n = 9 时共 352 个方案。判题只忽略方案之间的顺序，方案内部的行顺序固定。",
        "sourceUrl": "https://leetcode.com/problems/n-queens/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    vector<vector<string>> solveNQueens(int n) {\n        return {}; // Every board, in any order.\n    }\n};",
        "acmIoSpec": "stdin：一行整数 n。stdout：第一行方案数 P；随后依次输出每个方案：先一行 n，再 n 行，每行一个长度为 n 的由 'Q' 和 '.' 组成的字符串（方案之间顺序任意）。",
    },
    "io": {
        "method": "solveNQueens",
        "params": [{"name": "n", "type": "int"}],
        "returns": "string[][]",
        "checker": "unordered",
    },
}


def solve(n):
    # Brute force over column permutations, filtering diagonal clashes.
    out = []
    for cols in itertools.permutations(range(n)):
        if len({r + c for r, c in enumerate(cols)}) == n and len({r - c for r, c in enumerate(cols)}) == n:
            out.append(["." * c + "Q" + "." * (n - c - 1) for c in cols])
    return out


def check(inp, expected, solved):
    return sorted(expected) == sorted(solved)


tests = [
    {"id": "t1", "sample": True, "input": 4,
     "expected": [[".Q..", "...Q", "Q...", "..Q."], ["..Q.", "Q...", "...Q", ".Q.."]]},
    {"id": "t2", "sample": True, "input": 1, "expected": [["Q"]]},
    {"id": "n2-none", "sample": False, "input": 2},
    {"id": "n3-none", "sample": False, "input": 3},
    {"id": "n5", "sample": False, "input": 5},
    {"id": "n6", "sample": False, "input": 6},
    {"id": "n8", "sample": False, "input": 8},
    {"id": "max-size-9", "sample": False, "input": 9},
]
write(problem, tests, solve, check=check)
