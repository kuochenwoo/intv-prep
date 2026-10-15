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
    "id": "lc-230",
    "title": "二叉搜索树中第 K 小的元素",
    "difficulty": "medium",
    "tags": ["树", "深度优先搜索", "二叉搜索树", "二叉树"],
    "statement": "给定一棵二叉搜索树的根节点 root 和整数 k，返回树中第 k 小的节点值（k 从 1 开始计数）。\n\n节点定义：struct TreeNode { int val; TreeNode *left; TreeNode *right; }。输入中的树为 LeetCode 层序表示，null 表示空位。",
    "constraints": ["节点数为 n；1 ≤ k ≤ n ≤ 10^4；0 ≤ Node.val ≤ 10^4"],
    "complexity": {
        "time": "O(h + k)（中序遍历到第 k 个即停止），最坏 O(n)",
        "space": "O(h) 显式栈或递归栈，最坏 O(n)。",
        "officialRequirement": "官方未强制复杂度要求；follow-up 询问若 BST 被频繁插入/删除且需频繁查询第 k 小，应如何优化（例如在节点上维护子树大小，使单次查询为 O(h)）。",
        "notes": "n 为节点数，h 为树高。本地最大测试为 10^4 个节点的左单链且 k = n（需走完整条链），另含右单链 k = n 与 10^4 个节点的随机 BST。",
        "sourceUrl": "https://leetcode.com/problems/kth-smallest-element-in-a-bst/",
    },
    "cpp": {
        "functionTemplate": "/**\n * struct TreeNode {\n *     int val;\n *     TreeNode *left;\n *     TreeNode *right;\n * };\n */\nclass Solution {\npublic:\n    int kthSmallest(TreeNode* root, int k) {\n        return 0; // The k-th smallest value (1-indexed).\n    }\n};",
        "acmIoSpec": "stdin：第一行 m（层序 token 数）；第二行 m 个 token（整数或 null），为 BST 的 LeetCode 层序表示；第三行整数 k。stdout：一个整数，第 k 小的值。",
    },
    "io": {
        "method": "kthSmallest",
        "params": [{"name": "root", "type": "TreeNode"}, {"name": "k", "type": "int"}],
        "returns": "int",
    },
}


def solve(data):
    # Collect all non-null level-order values and sort: independent of tree walks.
    return sorted(v for v in data["root"] if v is not None)[data["k"] - 1]


r = rng(230)


def bst_from(values):
    root = None
    for v in values:
        node = N(v)
        if not root:
            root = node
            continue
        cur = root
        while True:
            side = "left" if v < cur.val else "right"
            nxt = getattr(cur, side)
            if not nxt:
                setattr(cur, side, node)
                break
            cur = nxt
    return dump(root)


rand_vals = r.sample(range(0, 10001), 10000)
tests = [
    {"id": "t1", "input": {"root": [3, 1, 4, None, 2], "k": 1}, "sample": True, "expected": 1},
    {"id": "t2", "input": {"root": [5, 3, 6, 2, 4, None, None, 1], "k": 3}, "sample": True, "expected": 3},
    {"id": "single", "input": {"root": [0], "k": 1}, "sample": False},
    {"id": "k-equals-n", "input": {"root": [5, 3, 6, 2, 4, None, None, 1], "k": 6}, "sample": False},
    {"id": "root-is-answer", "input": {"root": [10, 5, 15, 2, 7], "k": 4}, "sample": False},
    {"id": "random-200", "input": {"root": bst_from(r.sample(range(0, 10001), 200)), "k": 117}, "sample": False},
    {"id": "random-10000", "input": {"root": bst_from(rand_vals), "k": 6543}, "sample": False},
    {"id": "right-chain-10000-k-n", "input": {"root": bst_from(list(range(1, 10001))), "k": 10000}, "sample": False},
    {"id": "max-size-10000", "input": {"root": bst_from(list(range(10000, 0, -1))), "k": 10000}, "sample": False},
]
write(problem, tests, solve)
