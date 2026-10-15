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
    "id": "lc-199",
    "title": "二叉树的右视图",
    "difficulty": "medium",
    "tags": ["树", "深度优先搜索", "广度优先搜索", "二叉树"],
    "statement": "给定二叉树的根节点 root，想象自己站在树的右侧，从上到下返回每一层能看到的节点值（即每层最右侧的节点）。空树返回空数组。\n\n节点定义：struct TreeNode { int val; TreeNode *left; TreeNode *right; }。输入中的树为 LeetCode 层序表示，null 表示空位。",
    "en": {'title': 'Binary Tree Right Side View',
     'tags': ['Tree', 'Depth-First Search', 'Breadth-First Search', 'Binary Tree'],
     'statement': 'Given the root of a binary tree, imagine standing on its right side and return, '
                  'from top to bottom, the node values you can see on each level (i.e., the rightmost '
                  'node of each level). An empty tree returns an empty array.\n'
                  '\n'
                  'Node definition: struct TreeNode { int val; TreeNode *left; TreeNode *right; }. The '
                  "input tree uses LeetCode's level-order representation, with null marking empty "
                  'positions.',
     'constraints': ['The number of nodes is in [0, 100]; -100 ≤ Node.val ≤ 100'],
     'complexity': {'time': 'O(n)',
                    'space': 'BFS uses an O(w) queue; right-before-left DFS uses an O(h) recursion '
                             'stack. Worst case O(n). Output not counted.',
                    'officialRequirement': 'No official complexity requirement.',
                    'notes': 'n is the number of nodes, h the tree height, w the maximum level width. '
                             'Note that when the left subtree is deeper, the right side view sees '
                             'nodes on the left. Local tests include a random tree with 100 nodes and '
                             'a left-only chain.'},
     'acmIoSpec': 'stdin: line 1 is m (the number of level-order tokens, 0 for an empty tree); line 2 '
                  "has m tokens (integers or null), the tree's LeetCode level-order representation. "
                  'stdout: one line with the value of the rightmost node of each level, top to bottom, '
                  'space-separated (an empty tree prints an empty line).'},
    "constraints": ["节点数在 [0, 100] 范围内；-100 ≤ Node.val ≤ 100"],
    "complexity": {
        "time": "O(n)",
        "space": "BFS 为 O(w) 队列；先右后左的 DFS 为 O(h) 递归栈。最坏 O(n)。输出不计入。",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为节点数，h 为树高，w 为最大层宽。注意左子树更深时，右视图会看到左侧的节点。本地含 100 个节点的随机树与左单链。",
        "sourceUrl": "https://leetcode.com/problems/binary-tree-right-side-view/",
    },
    "cpp": {
        "functionTemplate": "/**\n * struct TreeNode {\n *     int val;\n *     TreeNode *left;\n *     TreeNode *right;\n * };\n */\nclass Solution {\npublic:\n    vector<int> rightSideView(TreeNode* root) {\n        return {}; // Rightmost value of each level, top to bottom.\n    }\n};",
        "acmIoSpec": "stdin：第一行 m（层序 token 数，空树为 0）；第二行 m 个 token（整数或 null），为树的 LeetCode 层序表示。stdout：一行，从上到下每层最右节点的值，空格分隔（空树输出空行）。",
    },
    "io": {
        "method": "rightSideView",
        "params": [{"name": "root", "type": "TreeNode"}],
        "returns": "int[]",
    },
}


def solve(tokens):
    out, level = [], [build(tokens)] if tokens else []
    while level:
        out.append(level[-1].val)
        level = [c for n in level for c in (n.left, n.right) if c]
    return out


r = rng(199)
val = lambda: r.randint(-100, 100)
tests = [
    {"id": "t1", "input": [1, 2, 3, None, 5, None, 4], "sample": True, "expected": [1, 3, 4]},
    {"id": "t2", "input": [1, 2, 3, 4, None, None, None, 5], "sample": True, "expected": [1, 3, 4, 5]},
    {"id": "t3", "input": [1, None, 3], "sample": True, "expected": [1, 3]},
    {"id": "t4", "input": [], "sample": True, "expected": []},
    {"id": "left-chain-100", "input": shaped(chain_shape(100, "L"), [val() for _ in range(100)]), "sample": False},
    {"id": "deep-left-subtree", "input": [1, 2, 3, 4, 5, None, None, 6, None, None, 7], "sample": False},
    {"id": "random-40", "input": shaped(random_shape(r, 40), [val() for _ in range(40)]), "sample": False},
    {"id": "max-size-100", "input": shaped(random_shape(r, 100), [val() for _ in range(100)]), "sample": False},
]
write(problem, tests, solve)
