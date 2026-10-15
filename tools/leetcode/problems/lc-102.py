import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

sys.setrecursionlimit(1_000_000)


# ---- binary tree helpers (LeetCode level order, None = null) ----
class N:
    __slots__ = ("val", "left", "right")

    def __init__(self, val=0):
        self.val, self.left, self.right = val, None, None


def build(tokens):
    if not tokens or tokens[0] is None:
        return None
    root = N(tokens[0])
    queue, head, i = [root], 0, 1
    while head < len(queue) and i < len(tokens):
        node = queue[head]
        head += 1
        for side in ("left", "right"):
            if i < len(tokens):
                if tokens[i] is not None:
                    child = N(tokens[i])
                    setattr(node, side, child)
                    queue.append(child)
                i += 1
    return root


def dump(root):
    out, queue, head = [], [root], 0
    while head < len(queue):
        node = queue[head]
        head += 1
        if node is None:
            out.append(None)
            continue
        out.append(node.val)
        queue += [node.left, node.right]
    while out and out[-1] is None:
        out.pop()
    return out


def random_shape(r, n):
    """n nodes attached at uniformly random free child slots; returns (root, nodes in creation order)."""
    nodes, free = [N()], [(0, "left"), (0, "right")]
    for _ in range(n - 1):
        j = r.randrange(len(free))
        idx, side = free[j]
        free[j] = free[-1]
        free.pop()
        child = N()
        setattr(nodes[idx], side, child)
        nodes.append(child)
        free += [(len(nodes) - 1, "left"), (len(nodes) - 1, "right")]
    return nodes[0], nodes


def chain_shape(n, pattern):
    """Degenerate chain: pattern 'L' (all left), 'R' (all right) or 'Z' (zigzag)."""
    nodes = [N()]
    for i in range(1, n):
        side = {"L": "left", "R": "right"}.get(pattern, "left" if i % 2 else "right")
        child = N()
        setattr(nodes[-1], side, child)
        nodes.append(child)
    return nodes[0], nodes


def shaped(root_nodes, values):
    root, nodes = root_nodes
    for node, v in zip(nodes, values):
        node.val = v
    return dump(root)
# ---- end helpers ----

problem = {
    "id": "lc-102",
    "title": "二叉树的层序遍历",
    "difficulty": "medium",
    "tags": ["树", "广度优先搜索", "二叉树"],
    "statement": "给定二叉树的根节点 root，按层从上到下、每层从左到右返回节点值，每一层的值组成一个数组。空树返回空数组。\n\n节点定义：struct TreeNode { int val; TreeNode *left; TreeNode *right; }。输入中的树为 LeetCode 层序表示，null 表示空位。",
    "en": {'title': 'Binary Tree Level Order Traversal',
     'tags': ['Tree', 'Breadth-First Search', 'Binary Tree'],
     'statement': 'Given the root of a binary tree, return the node values level by level from top to bottom, '
                  "left to right within each level, with each level's values forming one array. Return an empty "
                  'array for an empty tree.\n'
                  '\n'
                  'Node definition: struct TreeNode { int val; TreeNode *left; TreeNode *right; }. The input '
                  'tree uses LeetCode level-order representation, with null marking empty positions.',
     'constraints': ['The number of nodes is in the range [0, 2000]; -1000 ≤ Node.val ≤ 1000'],
     'complexity': {'time': 'O(n)',
                    'space': 'BFS queue O(w) (w is the maximum level width, about n/2 in the worst case); DFS by '
                             'depth uses an O(h) recursion stack. Output not counted.',
                    'officialRequirement': 'No official complexity requirement.',
                    'notes': 'n is the number of nodes and h is the tree height. The largest local test is a '
                             'random 2000-node tree, plus a 2000-node single chain (2000 levels) and a complete '
                             'binary tree.'},
     'acmIoSpec': 'stdin: line 1 is m (the number of level-order tokens, 0 for an empty tree); line 2 has m '
                  "tokens (integers or null), the tree's LeetCode level-order representation. stdout: line 1 is "
                  "the number of levels L; then L lines, each with that level's node values from left to right "
                  '(space-separated, no count). For an empty tree output only 0.'},
    "constraints": ["节点数在 [0, 2000] 范围内；-1000 ≤ Node.val ≤ 1000"],
    "complexity": {
        "time": "O(n)",
        "space": "BFS 队列 O(w)（w 为最大层宽，最坏约 n/2）；按深度 DFS 为 O(h) 递归栈。输出不计入。",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为节点数，h 为树高。本地最大测试为 2000 个节点的随机树，另含 2000 个节点的单链（2000 层）与满二叉树。",
        "sourceUrl": "https://leetcode.com/problems/binary-tree-level-order-traversal/",
    },
    "cpp": {
        "functionTemplate": "/**\n * struct TreeNode {\n *     int val;\n *     TreeNode *left;\n *     TreeNode *right;\n * };\n */\nclass Solution {\npublic:\n    vector<vector<int>> levelOrder(TreeNode* root) {\n        return {}; // One vector of values per level, top to bottom.\n    }\n};",
        "acmIoSpec": "stdin：第一行 m（层序 token 数，空树为 0）；第二行 m 个 token（整数或 null），为树的 LeetCode 层序表示。stdout：第一行层数 L；随后 L 行，每行为该层从左到右的节点值（空格分隔，不写个数）。空树只输出 0。",
    },
    "io": {
        "method": "levelOrder",
        "params": [{"name": "root", "type": "TreeNode"}],
        "returns": "int[][]",
    },
}


def solve(tokens):
    # Recursive DFS with depth (independent of the BFS C++ solution).
    levels = []

    def go(node, d):
        if node:
            if d == len(levels):
                levels.append([])
            levels[d].append(node.val)
            go(node.left, d + 1)
            go(node.right, d + 1)

    go(build(tokens), 0)
    return levels


r = rng(102)
val = lambda: r.randint(-1000, 1000)
tests = [
    {"id": "t1", "input": [3, 9, 20, None, None, 15, 7], "sample": True, "expected": [[3], [9, 20], [15, 7]]},
    {"id": "t2", "input": [1], "sample": True, "expected": [[1]]},
    {"id": "t3", "input": [], "sample": True, "expected": []},
    {"id": "gaps", "input": [1, 2, 3, 4, None, None, 5, None, 6, 7], "sample": False},
    {"id": "full-1023", "input": [val() for _ in range(1023)], "sample": False},
    {"id": "chain-2000", "input": shaped(chain_shape(2000, "Z"), [val() for _ in range(2000)]), "sample": False},
    {"id": "max-size-2000", "input": shaped(random_shape(r, 2000), [val() for _ in range(2000)]), "sample": False},
]
write(problem, tests, solve)
