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
    "id": "lc-98",
    "title": "验证二叉搜索树",
    "difficulty": "medium",
    "tags": ["树", "深度优先搜索", "二叉搜索树", "二叉树"],
    "statement": "给定二叉树的根节点 root，判断它是否为有效的二叉搜索树：任一节点左子树中的所有值都严格小于该节点值，右子树中的所有值都严格大于该节点值，且左右子树本身也都是二叉搜索树。\n\n节点定义：struct TreeNode { int val; TreeNode *left; TreeNode *right; }。输入中的树为 LeetCode 层序表示，null 表示空位。",
    "constraints": ["节点数在 [1, 10^4] 范围内；-2^31 ≤ Node.val ≤ 2^31 - 1"],
    "complexity": {
        "time": "O(n)",
        "space": "O(h) 递归栈或显式栈，最坏 O(n)。",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为节点数，h 为树高。注意相等值不合法，以及节点值可取到 int 边界（上下界宜用 long long 或指针表示）。只比较父子关系是错误的：本地含“孙节点违反祖先约束”的反例。最大测试为 10^4 个节点的单链，另含 10^4 个节点的随机 BST 及其深处被破坏的版本。",
        "sourceUrl": "https://leetcode.com/problems/validate-binary-search-tree/",
    },
    "cpp": {
        "functionTemplate": "/**\n * struct TreeNode {\n *     int val;\n *     TreeNode *left;\n *     TreeNode *right;\n * };\n */\nclass Solution {\npublic:\n    bool isValidBST(TreeNode* root) {\n        return false; // Whether every subtree respects strict BST ordering.\n    }\n};",
        "acmIoSpec": "stdin：第一行 m（层序 token 数）；第二行 m 个 token（整数或 null），为树的 LeetCode 层序表示。stdout：true 或 false。",
    },
    "io": {
        "method": "isValidBST",
        "params": [{"name": "root", "type": "TreeNode"}],
        "returns": "bool",
    },
}


def solve(tokens):
    # Check every node against the open interval inherited from its ancestors.
    stack = [(build(tokens), float("-inf"), float("inf"))]
    while stack:
        node, lo, hi = stack.pop()
        if not node:
            continue
        if not lo < node.val < hi:
            return False
        stack += [(node.left, lo, node.val), (node.right, node.val, hi)]
    return True


LO, HI = -2**31, 2**31 - 1
r = rng(98)


def bst_from(values):
    """Insert values in the given order (iteratively); returns (root, nodes)."""
    root, nodes = None, []
    for v in values:
        node = N(v)
        nodes.append(node)
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
    return root, nodes


vals = r.sample(range(LO, HI + 1), 10000)
big_root, big_nodes = bst_from(vals)
valid_big = dump(big_root)
# Break a deep node: keep it consistent with its parent but equal to a farther ancestor's bound.
info, stack = {}, [(big_root, LO - 1, HI + 1, 0, None)]
while stack:
    n, lo, hi, d, side = stack.pop()
    info[id(n)] = (lo, hi, d, side)
    if n.left:
        stack.append((n.left, lo, n.val, d + 1, "left"))
    if n.right:
        stack.append((n.right, n.val, hi, d + 1, "right"))
for n in sorted(big_nodes, key=lambda n: -info[id(n)][2]):
    lo, hi, d, side = info[id(n)]
    # A left child is bounded above by its parent; violate the inherited lower bound instead.
    if side == "left" and lo >= LO and not (n.left or n.right):
        n.val = lo
        break
    if side == "right" and hi <= HI and not (n.left or n.right):
        n.val = hi
        break
invalid_big = dump(big_root)

chain_vals = sorted(r.sample(range(LO, HI + 1), 10000))
right_chain = shaped(chain_shape(10000, "R"), chain_vals)
tests = [
    {"id": "t1", "input": [2, 1, 3], "sample": True, "expected": True},
    {"id": "t2", "input": [5, 1, 4, None, None, 3, 6], "sample": True, "expected": False},
    {"id": "single-min", "input": [LO], "sample": False},
    {"id": "int-extremes", "input": [LO, None, HI], "sample": False},
    {"id": "int-extremes-wrong-side", "input": [HI, None, LO], "sample": False},
    {"id": "equal-child", "input": [1, 1], "sample": False},
    {"id": "all-equal", "input": [2, 2, 2], "sample": False},
    {"id": "grandchild-violates-root", "input": [5, 4, 6, None, None, 3, 7], "sample": False},
    {"id": "left-subtree-too-big", "input": [10, 5, 15, 2, 11], "sample": False},
    {"id": "random-valid-10000", "input": valid_big, "sample": False},
    {"id": "random-deep-violation-10000", "input": invalid_big, "sample": False},
    {"id": "max-size-10000", "input": right_chain, "sample": False},
]
write(problem, tests, solve)
