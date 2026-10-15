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
    "id": "lc-105",
    "title": "从前序与中序遍历序列构造二叉树",
    "difficulty": "medium",
    "tags": ["树", "数组", "哈希表", "分治", "二叉树"],
    "statement": "给定同一棵二叉树的先序遍历 preorder 和中序遍历 inorder（树中节点值互不相同），重建这棵二叉树并返回根节点。\n\n节点定义：struct TreeNode { int val; TreeNode *left; TreeNode *right; }。输出的树为 LeetCode 层序表示，null 表示空位。",
    "en": {'title': 'Construct Binary Tree from Preorder and Inorder Traversal',
     'tags': ['Tree', 'Array', 'Hash Table', 'Divide and Conquer', 'Binary Tree'],
     'statement': 'Given the preorder traversal preorder and inorder traversal inorder of the same binary tree '
                  '(all node values are distinct), reconstruct the tree and return its root.\n'
                  '\n'
                  'Node definition: struct TreeNode { int val; TreeNode *left; TreeNode *right; }. The output '
                  'tree uses LeetCode level-order representation, with null marking empty positions.',
     'constraints': ['1 ≤ preorder.length ≤ 3000; inorder.length == preorder.length',
                     '-3000 ≤ preorder[i], inorder[i] ≤ 3000; values are distinct',
                     'Every value in inorder also appears in preorder; the input is guaranteed to be the '
                     'preorder and inorder traversals of the same tree'],
     'complexity': {'time': 'O(n) (a hash map locates the root within inorder; a linear search each time gives '
                            'O(n^2))',
                    'space': 'O(n) hash map + O(h) recursion stack; newly created nodes are output and not '
                             'counted.',
                    'officialRequirement': 'No official complexity requirement.',
                    'notes': 'n is the number of nodes and h is the tree height. The largest local test is a '
                             '3000-node left chain (recursion depth 3000, which also degrades per-step linear '
                             'search to O(n^2)), plus a random 3000-node tree and a zigzag chain. The output is '
                             'large, so the output limit is raised to 128 KiB.'},
     'acmIoSpec': 'stdin: line 1 is n; line 2 has n integers preorder; line 3 is n; line 4 has n integers '
                  "inorder. stdout: one line with the reconstructed tree's level-order tokens (integers or null, "
                  'space-separated, trailing nulls omitted).'},
    "constraints": [
        "1 ≤ preorder.length ≤ 3000；inorder.length == preorder.length",
        "-3000 ≤ preorder[i], inorder[i] ≤ 3000；值互不相同",
        "inorder 中的值均出现在 preorder 中；输入保证是同一棵树的先序与中序遍历",
    ],
    "complexity": {
        "time": "O(n)（哈希表定位根在中序中的位置；每次线性查找则为 O(n^2)）",
        "space": "O(n) 哈希表 + O(h) 递归栈；新建节点属于输出，不计入。",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为节点数，h 为树高。本地最大测试为 3000 个节点的左单链（递归深度 3000，且令逐个线性查找的做法退化到 O(n^2)），另含 3000 个节点的随机树与之字形链。输出较大，输出上限调至 128 KiB。",
        "sourceUrl": "https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/",
    },
    "cpp": {
        "functionTemplate": "/**\n * struct TreeNode {\n *     int val;\n *     TreeNode *left;\n *     TreeNode *right;\n *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}\n * };\n */\nclass Solution {\npublic:\n    TreeNode* buildTree(vector<int>& preorder, vector<int>& inorder) {\n        return nullptr; // Root of the reconstructed tree.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数 preorder；第三行 n；第四行 n 个整数 inorder。stdout：一行，重建后树的层序 token（整数或 null，空格分隔，省略末尾 null）。",
    },
    # A 3000-node chain prints ~6000 tokens (~35 KiB); leave headroom over the 64 KiB default.
    "limits": {"timeLimitMs": 1000, "memoryLimitMb": 128, "outputLimitKb": 128},
    "io": {
        "method": "buildTree",
        "params": [{"name": "preorder", "type": "int[]"}, {"name": "inorder", "type": "int[]"}],
        "returns": "TreeNode",
    },
}


def solve(data):
    pre, ino = data["preorder"], data["inorder"]
    pos = {v: i for i, v in enumerate(ino)}
    it = iter(pre)

    def make(lo, hi):
        if lo > hi:
            return None
        node = N(next(it))
        mid = pos[node.val]
        node.left = make(lo, mid - 1)
        node.right = make(mid + 1, hi)
        return node

    return dump(make(0, len(ino) - 1))


def traversals(root):
    pre, ino = [], []

    def go(node):
        if node:
            pre.append(node.val)
            go(node.left)
            ino.append(node.val)
            go(node.right)

    go(root)
    return {"preorder": pre, "inorder": ino}


r = rng(105)


def case(shape):
    root, nodes = shape
    for node, v in zip(nodes, r.sample(range(-3000, 3001), len(nodes))):
        node.val = v
    return traversals(root)


tests = [
    {"id": "t1", "input": {"preorder": [3, 9, 20, 15, 7], "inorder": [9, 3, 15, 20, 7]}, "sample": True, "expected": [3, 9, 20, None, None, 15, 7]},
    {"id": "t2", "input": {"preorder": [-1], "inorder": [-1]}, "sample": True, "expected": [-1]},
    {"id": "right-only", "input": {"preorder": [1, 2, 3], "inorder": [1, 2, 3]}, "sample": False},
    {"id": "left-only", "input": {"preorder": [3, 2, 1], "inorder": [1, 2, 3]}, "sample": False},
    {"id": "random-50", "input": case(random_shape(r, 50)), "sample": False},
    {"id": "random-3000", "input": case(random_shape(r, 3000)), "sample": False},
    {"id": "zigzag-3000", "input": case(chain_shape(3000, "Z")), "sample": False},
    {"id": "max-size-3000", "input": case(chain_shape(3000, "L")), "sample": False},
]
write(problem, tests, solve)
