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
    "id": "lc-543",
    "title": "二叉树的直径",
    "difficulty": "easy",
    "tags": ["树", "深度优先搜索", "二叉树"],
    "statement": "给定二叉树的根节点 root，返回树的直径：任意两个节点之间最长路径所含的边数。这条路径不一定经过根节点。\n\n节点定义：struct TreeNode { int val; TreeNode *left; TreeNode *right; }。输入中的树为 LeetCode 层序表示，null 表示空位。",
    "en": {
        "title": "Diameter of Binary Tree",
        "tags": [
            "Tree",
            "Depth-First Search",
            "Binary Tree"
        ],
        "statement": "Given the root of a binary tree, return the diameter of the tree: the number of edges on the longest path between any two nodes. This path does not necessarily pass through the root.\n\nNode definition: struct TreeNode { int val; TreeNode *left; TreeNode *right; }. The tree in the input uses LeetCode's level-order representation, where null marks an empty slot.",
        "constraints": [
            "The number of nodes is in the range [1, 10^4]; -100 ≤ Node.val ≤ 100"
        ],
        "complexity": {
            "time": "O(n)",
            "space": "O(h) recursion stack (or an explicit stack for iterative post-order), O(n) in the worst case.",
            "officialRequirement": "No official complexity requirement.",
            "notes": "n is the number of nodes and h is the tree height. A single post-order traversal finds the maximum, over all nodes, of the sum of left and right subtree heights. The largest local test is a single chain of 10^4 nodes (recursion depth 10^4, which the default stack can handle); there are also a V-shaped double chain, a tree whose diameter does not pass through the root, and a random tree of 10^4 nodes."
        },
        "acmIoSpec": "stdin: line 1 is m (number of level-order tokens); line 2 has m tokens (integers or null), the tree's LeetCode level-order representation. stdout: one integer, the diameter (number of edges)."
    },
    "constraints": ["节点数在 [1, 10^4] 范围内；-100 ≤ Node.val ≤ 100"],
    "complexity": {
        "time": "O(n)",
        "space": "O(h) 递归栈（或迭代后序的显式栈），最坏 O(n)。",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为节点数，h 为树高。一次后序遍历求每个节点左右子树高度之和的最大值。本地最大测试为 10^4 个节点的单链（递归深度 10^4，默认栈可承受），另含 V 形双链、直径不经过根的树与 10^4 个节点的随机树。",
        "sourceUrl": "https://leetcode.com/problems/diameter-of-binary-tree/",
    },
    "cpp": {
        "functionTemplate": "/**\n * struct TreeNode {\n *     int val;\n *     TreeNode *left;\n *     TreeNode *right;\n * };\n */\nclass Solution {\npublic:\n    int diameterOfBinaryTree(TreeNode* root) {\n        return 0; // Edges on the longest path between any two nodes.\n    }\n};",
        "acmIoSpec": "stdin：第一行 m（层序 token 数）；第二行 m 个 token（整数或 null），为树的 LeetCode 层序表示。stdout：一个整数，直径（边数）。",
    },
    "io": {
        "method": "diameterOfBinaryTree",
        "params": [{"name": "root", "type": "TreeNode"}],
        "returns": "int",
    },
}


def solve(tokens):
    # Two BFS passes on the undirected tree: farthest node from any node, then farthest from it.
    root = build(tokens)
    adj, stack = {}, [root]
    while stack:
        node = stack.pop()
        adj.setdefault(node, [])
        for c in (node.left, node.right):
            if c:
                adj[node].append(c)
                adj.setdefault(c, []).append(node)
                stack.append(c)

    def far(src):
        dist, frontier, last = {src: 0}, [src], src
        while frontier:
            nxt = []
            for u in frontier:
                for v in adj[u]:
                    if v not in dist:
                        dist[v] = dist[u] + 1
                        nxt.append(v)
                        last = v
            frontier = nxt
        return last, dist[last]

    a, _ = far(root)
    return far(a)[1]


r = rng(543)
val = lambda: r.randint(-100, 100)


def v_shape(k):
    root = N(0)
    for side in ("left", "right"):
        cur = root
        for _ in range(k):
            child = N(val())
            setattr(cur, side, child)
            cur = child
    return dump(root)


# Diameter inside the left subtree: root -> a with a long bushy subtree, short right side.
off_root = N(1)
off_root.right = N(2)
sub, sub_nodes = random_shape(r, 2000)
for node in sub_nodes:
    node.val = val()
off_root.left = sub
tests = [
    {"id": "t1", "input": [1, 2, 3, 4, 5], "sample": True, "expected": 3},
    {"id": "t2", "input": [1, 2], "sample": True, "expected": 1},
    {"id": "single", "input": [7], "sample": False},
    {"id": "not-through-root", "input": [1, 2, None, 3, 4, 5, None, None, 6, 7, None, None, 8], "sample": False},
    {"id": "v-shape-4999", "input": v_shape(2499), "sample": False},
    {"id": "off-root-2002", "input": dump(off_root), "sample": False},
    {"id": "random-10000", "input": shaped(random_shape(r, 10000), [val() for _ in range(10000)]), "sample": False},
    {"id": "max-size-10000", "input": shaped(chain_shape(10000, "Z"), [val() for _ in range(10000)]), "sample": False},
]
write(problem, tests, solve)
