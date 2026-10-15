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
    "id": "lc-124",
    "title": "二叉树中的最大路径和",
    "difficulty": "hard",
    "tags": ["树", "深度优先搜索", "动态规划", "二叉树"],
    "statement": "二叉树中的一条路径是由若干节点组成的序列，相邻节点之间有边相连，且每个节点至多出现一次；路径至少包含一个节点，不必经过根。路径和为路径上所有节点值之和。给定二叉树的根节点 root，返回所有路径中的最大路径和。\n\n节点定义：struct TreeNode { int val; TreeNode *left; TreeNode *right; }。输入中的树为 LeetCode 层序表示，null 表示空位。",
    "constraints": ["节点数在 [1, 3·10^4] 范围内；-1000 ≤ Node.val ≤ 1000"],
    "complexity": {
        "time": "O(n)",
        "space": "O(h) 递归栈（或迭代后序的显式栈），最坏 O(n)。",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为节点数，h 为树高。后序遍历求每个节点向下的最大单链贡献 max(0, 左, 右) + val，并用 val + max(0,左) + max(0,右) 更新答案。答案绝对值不超过 3·10^7，int 足够。本地最大测试为 3·10^4 个节点的单链（递归深度 3·10^4，默认栈可承受），另含全负树与 10^4 个节点的随机树。",
        "sourceUrl": "https://leetcode.com/problems/binary-tree-maximum-path-sum/",
    },
    "cpp": {
        "functionTemplate": "/**\n * struct TreeNode {\n *     int val;\n *     TreeNode *left;\n *     TreeNode *right;\n * };\n */\nclass Solution {\npublic:\n    int maxPathSum(TreeNode* root) {\n        return 0; // Largest sum over all non-empty paths.\n    }\n};",
        "acmIoSpec": "stdin：第一行 m（层序 token 数）；第二行 m 个 token（整数或 null），为树的 LeetCode 层序表示。stdout：一个整数，最大路径和。",
    },
    "io": {
        "method": "maxPathSum",
        "params": [{"name": "root", "type": "TreeNode"}],
        "returns": "int",
    },
}


def solve(tokens):
    best = float("-inf")

    def gain(node):
        nonlocal best
        if not node:
            return 0
        a, b = max(gain(node.left), 0), max(gain(node.right), 0)
        best = max(best, node.val + a + b)
        return node.val + max(a, b)

    gain(build(tokens))
    return best


def brute(tokens):
    # Small trees only: max over all simple paths via DFS from every node on the undirected tree.
    root = build(tokens)
    adj, stack = {}, [root]
    while stack:
        n = stack.pop()
        adj.setdefault(n, [])
        for c in (n.left, n.right):
            if c:
                adj[n].append(c)
                adj.setdefault(c, []).append(n)
                stack.append(c)
    best = float("-inf")
    for s in adj:
        st = [(s, None, s.val)]
        while st:
            u, p, tot = st.pop()
            best = max(best, tot)
            for v in adj[u]:
                if v is not p:
                    st.append((v, u, tot + v.val))
    return best


r = rng(124)
val = lambda: r.randint(-1000, 1000)
small_cases = [shaped(random_shape(r, k), [val() for _ in range(k)]) for k in (5, 12, 40, 80)]
for c in small_cases:
    assert brute(c) == solve(c)
chain_vals = [r.randint(-200, 1000) for _ in range(30000)]
tests = [
    {"id": "t1", "input": [1, 2, 3], "sample": True, "expected": 6},
    {"id": "t2", "input": [-10, 9, 20, None, None, 15, 7], "sample": True, "expected": 42},
    {"id": "single-negative", "input": [-3], "sample": False},
    {"id": "all-negative", "input": [-5, -2, -7, -9, -1], "sample": False},
    {"id": "skip-negative-root", "input": [-1000, 600, 700], "sample": False},
    {"id": "bend-below-root", "input": [1, -2, 3, 40, 50, None, None, -1000], "sample": False},
] + [{"id": f"small-{i}", "input": c, "sample": False} for i, c in enumerate(small_cases)] + [
    {"id": "all-negative-5000", "input": shaped(random_shape(r, 5000), [r.randint(-1000, -1) for _ in range(5000)]), "sample": False},
    {"id": "random-10000", "input": shaped(random_shape(r, 10000), [val() for _ in range(10000)]), "sample": False},
    {"id": "max-size-30000", "input": shaped(chain_shape(30000, "Z"), chain_vals), "sample": False},
]
write(problem, tests, solve)
