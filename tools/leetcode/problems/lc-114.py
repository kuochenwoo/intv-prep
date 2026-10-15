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
    "id": "lc-114",
    "title": "二叉树展开为链表",
    "difficulty": "medium",
    "tags": ["栈", "树", "深度优先搜索", "链表", "二叉树"],
    "statement": "给定二叉树的根节点 root，原地将其展开为一条“链表”：仍使用 TreeNode，每个节点的 right 指向链表中的下一个节点，left 一律置为 nullptr，且节点顺序与原树的先序遍历（根 → 左 → 右）一致。函数无返回值，判题时输出展开后的 root。\n\n节点定义：struct TreeNode { int val; TreeNode *left; TreeNode *right; }。输入输出中的树均为 LeetCode 层序表示，null 表示空位（展开后形如 1 null 2 null 3 ...）。",
    "constraints": ["节点数在 [0, 2000] 范围内；-100 ≤ Node.val ≤ 100"],
    "complexity": {
        "time": "O(n)",
        "space": "O(1) 额外空间（寻找左子树最右节点并拼接，Morris 思路）；递归或栈实现为 O(h)/O(n)。",
        "officialRequirement": "官方 follow-up 要求原地展开且只用 O(1) 额外空间。",
        "notes": "n 为节点数，h 为树高。O(1) 解法中每条边至多被额外走一次，总时间仍为 O(n)。本地最大测试为 2000 个节点的随机树，另含 2000 个节点的左单链（展开后为右单链）。",
        "sourceUrl": "https://leetcode.com/problems/flatten-binary-tree-to-linked-list/",
    },
    "cpp": {
        "functionTemplate": "/**\n * struct TreeNode {\n *     int val;\n *     TreeNode *left;\n *     TreeNode *right;\n * };\n */\nclass Solution {\npublic:\n    void flatten(TreeNode* root) {\n        // Rewire in place: right = next node in pre-order, left = nullptr.\n    }\n};",
        "acmIoSpec": "stdin：第一行 m（层序 token 数，空树为 0）；第二行 m 个 token（整数或 null），为树的 LeetCode 层序表示。stdout：一行，展开后树的层序 token（形如 v1 null v2 null v3 …，省略末尾 null；空树输出空行）。",
    },
    "io": {
        "method": "flatten",
        "params": [{"name": "root", "type": "TreeNode"}],
        "returns": "void",
        "output": "root",
    },
}


def solve(tokens):
    pre, stack = [], [build(tokens)]
    while stack:
        node = stack.pop()
        if node:
            pre.append(node.val)
            stack += [node.right, node.left]
    out = []
    for i, v in enumerate(pre):
        out += [v] if i == 0 else [None, v]
    return out


r = rng(114)
val = lambda: r.randint(-100, 100)
tests = [
    {"id": "t1", "input": [1, 2, 5, 3, 4, None, 6], "sample": True, "expected": [1, None, 2, None, 3, None, 4, None, 5, None, 6]},
    {"id": "t2", "input": [], "sample": True, "expected": []},
    {"id": "t3", "input": [0], "sample": True, "expected": [0]},
    {"id": "already-flat", "input": [1, None, 2, None, 3], "sample": False},
    {"id": "only-left", "input": [1, 2, None, 3], "sample": False},
    {"id": "full-127", "input": [val() for _ in range(127)], "sample": False},
    {"id": "left-chain-2000", "input": shaped(chain_shape(2000, "L"), [val() for _ in range(2000)]), "sample": False},
    {"id": "zigzag-2000", "input": shaped(chain_shape(2000, "Z"), [val() for _ in range(2000)]), "sample": False},
    {"id": "max-size-2000", "input": shaped(random_shape(r, 2000), [val() for _ in range(2000)]), "sample": False},
]
write(problem, tests, solve)
