import sys, pathlib
from collections import Counter
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-79",
    "title": "单词搜索",
    "difficulty": "medium",
    "tags": ["深度优先搜索", "数组", "字符串", "回溯", "矩阵"],
    "statement": "给定 m × n 字符网格 board 和字符串 word，判断 word 能否由网格中一条路径上的字母依次拼出：路径上相邻两格必须上下或左右相邻，同一格在一条路径中最多使用一次。能则返回 true，否则返回 false。",
    "constraints": [
        "m == board.length，n == board[i].length",
        "1 ≤ m, n ≤ 6",
        "1 ≤ word.length ≤ 15",
        "board 与 word 仅由大小写英文字母组成（区分大小写）",
    ],
    "complexity": {
        "time": "O(m·n·3^L) 最坏情况（本地训练目标；L 为 word 长度，除第一步外每步至多 3 个方向）",
        "space": "O(L) 额外空间（递归栈，计入递归栈；原地标记已访问格子，输入不计入）",
        "officialRequirement": "官方未强制复杂度要求；follow-up 询问能否通过搜索剪枝让解法在更大的网格上更快。",
        "notes": "m、n 为网格行列数。常见剪枝：先比较字母计数，网格中某字母数量不足直接返回 false；若 word 末字母在网格中出现得更少，则反转 word 再搜索。本地最坏测试为 6 × 6 全 'A' 网格配合以网格中不存在的字母结尾的长单词，未剪枝的回溯需遍历大量路径。",
        "sourceUrl": "https://leetcode.com/problems/word-search/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    bool exist(vector<vector<char>>& board, string word) {\n        return false; // Whether word can be traced on the board.\n    }\n};",
        "acmIoSpec": "stdin：第一行 m；随后 m 行，每行一个长度为 n 的字母串，表示网格的一行；最后一行为 word。stdout：true 或 false。",
    },
    "io": {
        "method": "exist",
        "params": [{"name": "board", "type": "char[][]"}, {"name": "word", "type": "string"}],
        "returns": "bool",
    },
}


def solve(data):
    board, word = [row[:] for row in data["board"]], data["word"]
    m, n = len(board), len(board[0])
    have = Counter(c for row in board for c in row)
    if any(have[c] < k for c, k in Counter(word).items()):
        return False

    def dfs(i, j, k):
        if board[i][j] != word[k]:
            return False
        if k == len(word) - 1:
            return True
        board[i][j] = None
        found = any(0 <= a < m and 0 <= b < n and board[a][b] is not None and dfs(a, b, k + 1)
                    for a, b in ((i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)))
        board[i][j] = word[k]
        return found

    return any(dfs(i, j, 0) for i in range(m) for j in range(n))


def g(rows):
    return [list(r) for r in rows]


def rand_board(r, m, n, alphabet):
    return [[r.choice(alphabet) for _ in range(n)] for _ in range(m)]


def path_word(r, board, length):
    # Random self-avoiding walk; returns the spelled word (may be shorter if stuck).
    m, n = len(board), len(board[0])
    i, j = r.randrange(m), r.randrange(n)
    seen, out = {(i, j)}, [board[i][j]]
    while len(out) < length:
        nxt = [(a, b) for a, b in ((i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1))
               if 0 <= a < m and 0 <= b < n and (a, b) not in seen]
        if not nxt:
            break
        i, j = r.choice(nxt)
        seen.add((i, j))
        out.append(board[i][j])
    return "".join(out)


r = rng(79)
sample_board = g(["ABCE", "SFCS", "ADEE"])
rb = rand_board(r, 6, 6, "abAB")
all_a = [["A"] * 6 for _ in range(6)]
a_with_b = [["A"] * 6 for _ in range(6)]
a_with_b[5][5] = "B"
tests = [
    {"id": "t1", "sample": True, "expected": True, "input": {"board": sample_board, "word": "ABCCED"}},
    {"id": "t2", "sample": True, "expected": True, "input": {"board": sample_board, "word": "SEE"}},
    {"id": "t3", "sample": True, "expected": False, "input": {"board": sample_board, "word": "ABCB"}},
    {"id": "single-hit", "sample": False, "input": {"board": g(["a"]), "word": "a"}},
    {"id": "single-miss", "sample": False, "input": {"board": g(["a"]), "word": "b"}},
    {"id": "case-sensitive", "sample": False, "input": {"board": g(["aB", "cd"]), "word": "ab"}},
    {"id": "too-long", "sample": False, "input": {"board": g(["ab", "cd"]), "word": "abdca"}},
    {"id": "reuse-forbidden", "sample": False, "input": {"board": g(["aa"]), "word": "aaa"}},
    {"id": "one-row", "sample": False, "input": {"board": g(["abcdef"]), "word": "fedc"}},
    {"id": "random-hit", "sample": False, "input": {"board": rb, "word": path_word(r, rb, 12)}},
    {"id": "random-miss", "sample": False, "input": {"board": rb, "word": "".join(r.choice("abAB") for _ in range(15))}},
    {"id": "max-size-snake-hit", "sample": False, "input": {"board": a_with_b, "word": "A" * 14 + "B"}},
    {"id": "max-size-missing-letter", "sample": False, "input": {"board": all_a, "word": "A" * 14 + "B"}},
    {"id": "max-size-reversed-rare", "sample": False, "input": {"board": a_with_b, "word": "B" + "A" * 14}},
]
write(problem, tests, solve)
