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
    "id": "lc-101",
    "title": "对称二叉树",
    "difficulty": "easy",
    "tags": ["树", "深度优先搜索", "广度优先搜索", "二叉树"],
    "statement": "给定二叉树的根节点 root，判断它是否关于根节点轴对称：即左子树与右子树互为镜像（结构镜像且对应位置节点值相等）。\n\n节点定义：struct TreeNode { int val; TreeNode *left; TreeNode *right; }。输入中的树为 LeetCode 层序表示，null 表示空位。",
    "en": {'title': 'Symmetric Tree',
     'tags': ['Tree', 'Depth-First Search', 'Breadth-First Search', 'Binary Tree'],
     'statement': 'Given the root of a binary tree, determine whether it is symmetric around its root: that is, '
                  'the left and right subtrees are mirror images of each other (mirrored structure with equal '
                  'values at corresponding positions).\n'
                  '\n'
                  'Node definition: struct TreeNode { int val; TreeNode *left; TreeNode *right; }. The input '
                  'tree uses LeetCode level-order representation, with null marking empty positions.',
     'constraints': ['The number of nodes is in the range [1, 1000]; -100 ≤ Node.val ≤ 100'],
     'complexity': {'time': 'O(n)',
                    'space': 'Recursive: O(h) recursion stack; iterative queue: O(w). Worst case O(n).',
                    'officialRequirement': 'No official complexity requirement; the follow-up asks for both a '
                                           'recursive and an iterative implementation.',
                    'notes': 'n is the number of nodes, h the tree height, and w the maximum level width. A '
                             'symmetric tree must have an odd number of nodes, so the largest 1000-node test is '
                             'necessarily asymmetric; there are also a symmetric 999-node V-shaped double chain '
                             'and random symmetric trees.'},
     'acmIoSpec': 'stdin: line 1 is m (the number of level-order tokens); line 2 has m tokens (integers or '
                  "null), the tree's LeetCode level-order representation. stdout: true or false."},
    "constraints": ["节点数在 [1, 1000] 范围内；-100 ≤ Node.val ≤ 100"],
    "complexity": {
        "time": "O(n)",
        "space": "递归为 O(h) 递归栈；迭代队列为 O(w)。最坏 O(n)。",
        "officialRequirement": "官方未强制复杂度要求；follow-up 要求分别用递归和迭代两种方式实现。",
        "notes": "n 为节点数，h 为树高，w 为最大层宽。对称树节点数必为奇数，因此 1000 个节点的最大测试必然不对称；另含 999 个节点的对称 V 形双链与随机对称树。",
        "sourceUrl": "https://leetcode.com/problems/symmetric-tree/",
    },
    "cpp": {
        "functionTemplate": "/**\n * struct TreeNode {\n *     int val;\n *     TreeNode *left;\n *     TreeNode *right;\n * };\n */\nclass Solution {\npublic:\n    bool isSymmetric(TreeNode* root) {\n        return false; // Whether the tree mirrors itself around the root.\n    }\n};",
        "acmIoSpec": "stdin：第一行 m（层序 token 数）；第二行 m 个 token（整数或 null），为树的 LeetCode 层序表示。stdout：true 或 false。",
    },
    "io": {
        "method": "isSymmetric",
        "params": [{"name": "root", "type": "TreeNode"}],
        "returns": "bool",
    },
}


def solve(tokens):
    # Compare each level's value list (with null placeholders) to its reverse.
    level = [build(tokens)]
    while any(level):
        vals = [n.val if n else None for n in level]
        if vals != vals[::-1]:
            return False
        level = [c for n in level if n for c in (n.left, n.right)]
    return True


def copy_mirror(node):
    if not node:
        return None
    m = N(node.val)
    m.left, m.right = copy_mirror(node.right), copy_mirror(node.left)
    return m


def symmetric(half, rootval=0):
    root = N(rootval)
    root.left, root.right = half, copy_mirror(half)
    return root


r = rng(101)
val = lambda: r.randint(-100, 100)


def random_half(k):
    root, nodes = random_shape(r, k)
    for node in nodes:
        node.val = val()
    return root


sym_random = symmetric(random_half(499), 7)
broken = symmetric(random_half(499), 7)
deep = broken.right
while deep.left or deep.right:
    deep = deep.left or deep.right
deep.val = deep.val + 1 if deep.val < 100 else deep.val - 1
lopsided = symmetric(random_half(499), 3)
leaf = lopsided.left
while leaf.left or leaf.right:
    leaf = leaf.left or leaf.right
leaf.left = N(1)  # 1000 nodes, no mirror counterpart
v_root, v_nodes = chain_shape(499, "L")
for node in v_nodes:
    node.val = 1
tests = [
    {"id": "t1", "input": [1, 2, 2, 3, 4, 4, 3], "sample": True, "expected": True},
    {"id": "t2", "input": [1, 2, 2, None, 3, None, 3], "sample": True, "expected": False},
    {"id": "single", "input": [0], "sample": False},
    {"id": "two-nodes", "input": [1, 2], "sample": False},
    {"id": "same-not-mirror", "input": [1, 2, 2, 3, None, 3], "sample": False},
    {"id": "values-differ", "input": [1, 2, 3], "sample": False},
    {"id": "v-shape-999", "input": dump(symmetric(v_root, 1)), "sample": False},
    {"id": "random-symmetric-999", "input": dump(sym_random), "sample": False},
    {"id": "deep-value-mismatch-999", "input": dump(broken), "sample": False},
    {"id": "max-size-1000", "input": dump(lopsided), "sample": False},
]
write(problem, tests, solve)
