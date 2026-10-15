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
    "id": "lc-104",
    "title": "二叉树的最大深度",
    "difficulty": "easy",
    "tags": ["树", "深度优先搜索", "广度优先搜索", "二叉树"],
    "statement": "给定二叉树的根节点 root，返回它的最大深度，即从根节点到最远叶子节点的路径上的节点个数。空树深度为 0。\n\n节点定义：struct TreeNode { int val; TreeNode *left; TreeNode *right; }。输入中的树为 LeetCode 层序表示，null 表示空位。",
    "constraints": ["节点数在 [0, 10^4] 范围内；-100 ≤ Node.val ≤ 100"],
    "complexity": {
        "time": "O(n)",
        "space": "递归 DFS 为 O(h) 递归栈；BFS 为 O(w) 队列（w 为最大层宽）。最坏均为 O(n)。",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为节点数，h 为树高。本地最大测试为 10^4 个节点的单链（递归深度 10^4，默认栈可承受）与 10^4 个节点的随机树。",
        "sourceUrl": "https://leetcode.com/problems/maximum-depth-of-binary-tree/",
    },
    "cpp": {
        "functionTemplate": "/**\n * struct TreeNode {\n *     int val;\n *     TreeNode *left;\n *     TreeNode *right;\n * };\n */\nclass Solution {\npublic:\n    int maxDepth(TreeNode* root) {\n        return 0; // Number of nodes on the longest root-to-leaf path.\n    }\n};",
        "acmIoSpec": "stdin：第一行 m（层序 token 数，空树为 0）；第二行 m 个 token（整数或 null），为树的 LeetCode 层序表示。stdout：一个整数，最大深度。",
    },
    "io": {
        "method": "maxDepth",
        "params": [{"name": "root", "type": "TreeNode"}],
        "returns": "int",
    },
}


def solve(tokens):
    # Level-by-level count over the level-order list itself (independent of node pointers).
    root = build(tokens)
    depth, level = 0, [root] if root else []
    while level:
        depth += 1
        level = [c for node in level for c in (node.left, node.right) if c]
    return depth


r = rng(104)
val = lambda: r.randint(-100, 100)
tests = [
    {"id": "t1", "input": [3, 9, 20, None, None, 15, 7], "sample": True, "expected": 3},
    {"id": "t2", "input": [1, None, 2], "sample": True, "expected": 2},
    {"id": "empty", "input": [], "sample": False},
    {"id": "single", "input": [-100], "sample": False},
    {"id": "deep-left-shallow-right", "input": [1, 2, 3, 4, None, None, None, 5], "sample": False},
    {"id": "random-500", "input": shaped(random_shape(r, 500), [val() for _ in range(500)]), "sample": False},
    {"id": "zigzag-5000", "input": shaped(chain_shape(5000, "Z"), [val() for _ in range(5000)]), "sample": False},
    {"id": "random-10000", "input": shaped(random_shape(r, 10000), [val() for _ in range(10000)]), "sample": False},
    {"id": "max-size-10000", "input": shaped(chain_shape(10000, "R"), [val() for _ in range(10000)]), "sample": False},
]
write(problem, tests, solve)
