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
    "id": "lc-94",
    "title": "二叉树的中序遍历",
    "difficulty": "easy",
    "tags": ["栈", "树", "深度优先搜索", "二叉树"],
    "statement": "给定二叉树的根节点 root，按“左子树 → 根 → 右子树”的中序顺序返回所有节点值。空树返回空数组。\n\n节点定义：struct TreeNode { int val; TreeNode *left; TreeNode *right; }。输入输出中的树均为 LeetCode 层序表示，null 表示空位。",
    "constraints": ["节点数在 [0, 100] 范围内；-100 ≤ Node.val ≤ 100"],
    "complexity": {
        "time": "O(n)",
        "space": "O(h) 额外空间（显式栈或递归栈，h 为树高，最坏 O(n)）；Morris 遍历可做到 O(1)。输出数组不计入。",
        "officialRequirement": "官方未强制复杂度要求；follow-up 要求用迭代代替递归实现。",
        "notes": "n 为节点数，h 为树高（平衡时约 log n，退化成链时为 n）。本地含 100 个节点的随机树与单链最坏情况。",
        "sourceUrl": "https://leetcode.com/problems/binary-tree-inorder-traversal/",
    },
    "cpp": {
        "functionTemplate": "/**\n * struct TreeNode {\n *     int val;\n *     TreeNode *left;\n *     TreeNode *right;\n * };\n */\nclass Solution {\npublic:\n    vector<int> inorderTraversal(TreeNode* root) {\n        return {}; // Values in in-order.\n    }\n};",
        "acmIoSpec": "stdin：第一行 m（层序 token 数，空树为 0）；第二行 m 个 token（整数或 null），为树的 LeetCode 层序表示。stdout：一行，中序遍历的节点值，空格分隔（空树输出空行）。",
    },
    "io": {
        "method": "inorderTraversal",
        "params": [{"name": "root", "type": "TreeNode"}],
        "returns": "int[]",
    },
}


def solve(tokens):
    out = []

    def go(node):
        if node:
            go(node.left)
            out.append(node.val)
            go(node.right)

    go(build(tokens))
    return out


r = rng(94)
val = lambda: r.randint(-100, 100)
tests = [
    {"id": "t1", "input": [1, None, 2, 3], "sample": True, "expected": [1, 3, 2]},
    {"id": "t2", "input": [1, 2, 3, 4, 5, None, 8, None, None, 6, 7, 9], "sample": True, "expected": [4, 2, 6, 5, 7, 1, 3, 9, 8]},
    {"id": "t3", "input": [], "sample": True, "expected": []},
    {"id": "t4", "input": [1], "sample": True, "expected": [1]},
    {"id": "duplicates", "input": [0, 0, 0, None, 0, 0], "sample": False},
    {"id": "left-chain-100", "input": shaped(chain_shape(100, "L"), [val() for _ in range(100)]), "sample": False},
    {"id": "zigzag-100", "input": shaped(chain_shape(100, "Z"), [val() for _ in range(100)]), "sample": False},
    {"id": "random-30", "input": shaped(random_shape(r, 30), [val() for _ in range(30)]), "sample": False},
    {"id": "max-size-100", "input": shaped(random_shape(r, 100), [val() for _ in range(100)]), "sample": False},
]
write(problem, tests, solve)
