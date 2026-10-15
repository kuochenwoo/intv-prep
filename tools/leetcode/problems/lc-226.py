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
    "id": "lc-226",
    "title": "翻转二叉树",
    "difficulty": "easy",
    "tags": ["树", "深度优先搜索", "广度优先搜索", "二叉树"],
    "statement": "给定二叉树的根节点 root，将每个节点的左右子树互换（即得到原树的镜像），返回翻转后的根节点。\n\n节点定义：struct TreeNode { int val; TreeNode *left; TreeNode *right; }。输入输出中的树均为 LeetCode 层序表示，null 表示空位。",
    "en": {'title': 'Invert Binary Tree',
     'tags': ['Tree', 'Depth-First Search', 'Breadth-First Search', 'Binary Tree'],
     'statement': 'Given the root of a binary tree, swap the left and right subtrees of every node '
                  '(producing the mirror image of the original tree) and return the root of the '
                  'inverted tree.\n'
                  '\n'
                  'Node definition: struct TreeNode { int val; TreeNode *left; TreeNode *right; }. '
                  "Trees in both input and output use LeetCode's level-order representation, with null "
                  'marking empty positions.',
     'constraints': ['The number of nodes is in [0, 100]; -100 ≤ Node.val ≤ 100'],
     'complexity': {'time': 'O(n)',
                    'space': 'Recursion uses an O(h) stack; BFS uses an O(w) queue. Worst case O(n). '
                             'Modified in place; no new nodes are created.',
                    'officialRequirement': 'No official complexity requirement.',
                    'notes': 'n is the number of nodes, h the tree height, w the maximum level width. '
                             'Local tests include a random tree with 100 nodes and a single chain.'},
     'acmIoSpec': 'stdin: line 1 is m (the number of level-order tokens, 0 for an empty tree); line 2 '
                  "has m tokens (integers or null), the tree's LeetCode level-order representation. "
                  "stdout: one line with the inverted tree's level-order tokens (space-separated, "
                  'trailing nulls omitted; an empty tree prints an empty line).'},
    "constraints": ["节点数在 [0, 100] 范围内；-100 ≤ Node.val ≤ 100"],
    "complexity": {
        "time": "O(n)",
        "space": "递归为 O(h) 递归栈；BFS 为 O(w) 队列。最坏 O(n)。原地修改，不新建节点。",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为节点数，h 为树高，w 为最大层宽。本地含 100 个节点的随机树与单链。",
        "sourceUrl": "https://leetcode.com/problems/invert-binary-tree/",
    },
    "cpp": {
        "functionTemplate": "/**\n * struct TreeNode {\n *     int val;\n *     TreeNode *left;\n *     TreeNode *right;\n * };\n */\nclass Solution {\npublic:\n    TreeNode* invertTree(TreeNode* root) {\n        return root; // Root of the mirrored tree.\n    }\n};",
        "acmIoSpec": "stdin：第一行 m（层序 token 数，空树为 0）；第二行 m 个 token（整数或 null），为树的 LeetCode 层序表示。stdout：一行，翻转后树的层序 token（空格分隔，省略末尾 null；空树输出空行）。",
    },
    "io": {
        "method": "invertTree",
        "params": [{"name": "root", "type": "TreeNode"}],
        "returns": "TreeNode",
    },
}


def solve(tokens):
    def mirror(node):
        if node:
            node.left, node.right = mirror(node.right), mirror(node.left)
        return node

    return dump(mirror(build(tokens)))


r = rng(226)
val = lambda: r.randint(-100, 100)
tests = [
    {"id": "t1", "input": [4, 2, 7, 1, 3, 6, 9], "sample": True, "expected": [4, 7, 2, 9, 6, 3, 1]},
    {"id": "t2", "input": [2, 1, 3], "sample": True, "expected": [2, 3, 1]},
    {"id": "t3", "input": [], "sample": True, "expected": []},
    {"id": "single", "input": [5], "sample": False},
    {"id": "only-left", "input": [1, 2, None, 3], "sample": False},
    {"id": "left-chain-100", "input": shaped(chain_shape(100, "L"), [val() for _ in range(100)]), "sample": False},
    {"id": "zigzag-100", "input": shaped(chain_shape(100, "Z"), [val() for _ in range(100)]), "sample": False},
    {"id": "max-size-100", "input": shaped(random_shape(r, 100), [val() for _ in range(100)]), "sample": False},
]
write(problem, tests, solve)
