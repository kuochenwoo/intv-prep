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
    "id": "lc-236",
    "title": "二叉树的最近公共祖先",
    "difficulty": "medium",
    "tags": ["树", "深度优先搜索", "二叉树"],
    "statement": "给定二叉树的根节点 root 以及树中两个不同的节点 p 和 q，返回它们的最近公共祖先：同时以 p、q 为后代的节点中深度最大的那个（一个节点也视为自己的后代）。返回的必须是原树中的节点。\n\n节点定义：struct TreeNode { int val; TreeNode *left; TreeNode *right; }。输入中的树为 LeetCode 层序表示，null 表示空位；树中节点值互不相同，p、q 在输入中以节点值给出，判题程序会找到对应节点再调用你的函数，并输出返回节点的值。",
    "en": {'title': 'Lowest Common Ancestor of a Binary Tree',
     'tags': ['Tree', 'Depth-First Search', 'Binary Tree'],
     'statement': 'Given the root of a binary tree and two distinct nodes p and q in it, return their '
                  'lowest common ancestor: the deepest node that has both p and q as descendants (a '
                  'node counts as its own descendant). The returned node must be a node of the '
                  'original tree.\n'
                  '\n'
                  'Node definition: struct TreeNode { int val; TreeNode *left; TreeNode *right; }. The '
                  "input tree uses LeetCode's level-order representation, with null marking empty "
                  'positions; node values are all distinct, p and q are given in the input by their '
                  'node values, and the judge finds the corresponding nodes before calling your '
                  'function, then prints the value of the node you return.',
     'constraints': ['The number of nodes is in [2, 10^5]; -10^9 ≤ Node.val ≤ 10^9; all node values '
                     'are distinct',
                     'p ≠ q, and both p and q are in the tree'],
     'complexity': {'time': 'O(n)',
                    'space': 'O(h) recursion stack, worst case O(n); the parent-pointer + ancestor-set '
                             'approach is O(n).',
                    'officialRequirement': 'No official complexity requirement.',
                    'notes': 'n is the number of nodes, h the tree height. The official limit is n = '
                             '10^5, but the level-order representation of 10^5 large-valued nodes '
                             'would make the problem bank file exceed 1 MiB, so the local worst-case '
                             'tests are reduced to 20001 nodes (a zigzag chain of 19999 nodes with two '
                             'leaves p and q branching off its bottom, recursion depth about 2·10^4, '
                             'which the default stack can handle) and a random tree of 10000 '
                             'large-valued nodes.'},
     'acmIoSpec': 'stdin: line 1 is m (the number of level-order tokens); line 2 has m tokens '
                  "(integers or null), the tree's LeetCode level-order representation; line 3 is the "
                  'integer node value of p; line 4 is the integer node value of q. stdout: one '
                  'integer, the node value of the lowest common ancestor.'},
    "constraints": [
        "节点数在 [2, 10^5] 范围内；-10^9 ≤ Node.val ≤ 10^9；所有节点值互不相同",
        "p ≠ q，且 p、q 均在树中",
    ],
    "complexity": {
        "time": "O(n)",
        "space": "O(h) 递归栈，最坏 O(n)；父指针 + 祖先集合做法为 O(n)。",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为节点数，h 为树高。官方上限 n = 10^5，但 10^5 个大值节点的层序表示会使题库文件超过 1 MiB，本地最坏测试缩小为 20001 个节点（19999 个节点的之字形单链，链底再分出 p、q 两个叶子，递归深度约 2·10^4，默认栈可承受）与 10000 个节点的大值随机树。",
        "sourceUrl": "https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/",
    },
    "cpp": {
        "functionTemplate": "/**\n * struct TreeNode {\n *     int val;\n *     TreeNode *left;\n *     TreeNode *right;\n * };\n */\nclass Solution {\npublic:\n    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {\n        return root; // The deepest node having both p and q as descendants.\n    }\n};",
        "acmIoSpec": "stdin：第一行 m（层序 token 数）；第二行 m 个 token（整数或 null），为树的 LeetCode 层序表示；第三行整数 p 的节点值；第四行整数 q 的节点值。stdout：一个整数，最近公共祖先的节点值。",
    },
    "io": {
        "method": "lowestCommonAncestor",
        "params": [{"name": "root", "type": "TreeNode"}, {"name": "p", "type": "int"}, {"name": "q", "type": "int"}],
        "returns": "int",
        "harness": "lc-236.cpp",
    },
}


def solve(data):
    # Parent map, then intersect ancestor chains.
    root = build(data["root"])
    parent, by_val, stack = {root: None}, {}, [root]
    while stack:
        n = stack.pop()
        by_val[n.val] = n
        for c in (n.left, n.right):
            if c:
                parent[c] = n
                stack.append(c)
    anc, n = set(), by_val[data["p"]]
    while n:
        anc.add(n)
        n = parent[n]
    n = by_val[data["q"]]
    while n not in anc:
        n = parent[n]
    return n.val


r = rng(236)
B = 10**9


def random_case(n, lo, hi):
    root, nodes = random_shape(r, n)
    for node, v in zip(nodes, r.sample(range(lo, hi + 1), n)):
        node.val = v
    p, q = r.sample(nodes, 2)
    return {"root": dump(root), "p": p.val, "q": q.val}


def deep_pair_case(n, lo, hi):
    # Deepest two leaves of a random tree: forces the full depth to be explored.
    root, nodes = random_shape(r, n)
    for node, v in zip(nodes, r.sample(range(lo, hi + 1), n)):
        node.val = v
    depth, stack = {root: 0}, [root]
    while stack:
        u = stack.pop()
        for c in (u.left, u.right):
            if c:
                depth[c] = depth[u] + 1
                stack.append(c)
    a, b = sorted(nodes, key=lambda x: -depth[x])[:2]
    return {"root": dump(root), "p": a.val, "q": b.val}


chain_root, chain_nodes = chain_shape(19999, "Z")
for i, node in enumerate(chain_nodes):
    node.val = i
# Fork at the bottom so p and q are siblings below a 20000-deep path.
chain_nodes[-1].left = N(19999)
chain_nodes[-1].right = N(20000)
tests = [
    {"id": "t1", "input": {"root": [3, 5, 1, 6, 2, 0, 8, None, None, 7, 4], "p": 5, "q": 1}, "sample": True, "expected": 3},
    {"id": "t2", "input": {"root": [3, 5, 1, 6, 2, 0, 8, None, None, 7, 4], "p": 5, "q": 4}, "sample": True, "expected": 5},
    {"id": "t3", "input": {"root": [1, 2], "p": 1, "q": 2}, "sample": True, "expected": 1},
    {"id": "child-is-q-root-p", "input": {"root": [1, None, 2], "p": 2, "q": 1}, "sample": False},
    {"id": "negative-values", "input": {"root": [-1, 0, 3, -2, 4, None, None, 8], "p": 8, "q": 4}, "sample": False},
    {"id": "extreme-values", "input": {"root": [0, -B, B, None, 7, -7], "p": 7, "q": -7}, "sample": False},
    {"id": "random-300", "input": random_case(300, -B, B), "sample": False},
    {"id": "random-deep-pair-10000", "input": deep_pair_case(10000, -B, B), "sample": False},
    {"id": "chain-ancestor-8000", "input": {"root": shaped(chain_shape(8000, "R"), list(range(8000))), "p": 7999, "q": 3}, "sample": False},
    {"id": "worst-case-chain-fork-20001", "input": {"root": dump(chain_root), "p": 19999, "q": 20000}, "sample": False},
]
write(problem, tests, solve)
