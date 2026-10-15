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
    "id": "lc-437",
    "title": "路径总和 III",
    "difficulty": "medium",
    "tags": ["树", "深度优先搜索", "二叉树", "前缀和"],
    "statement": "给定二叉树的根节点 root 和整数 targetSum，统计节点值之和等于 targetSum 的路径数目。路径必须自上而下（只能从父节点走向子节点），但不要求从根开始、也不要求在叶子结束；单个节点也算一条路径。\n\n节点定义：struct TreeNode { int val; TreeNode *left; TreeNode *right; }。输入中的树为 LeetCode 层序表示，null 表示空位。",
    "en": {
        "title": "Path Sum III",
        "tags": [
            "Tree",
            "Depth-First Search",
            "Binary Tree",
            "Prefix Sum"
        ],
        "statement": "Given the root of a binary tree and an integer targetSum, count the number of paths whose node values sum to targetSum. A path must go downward (only from parent to child), but it need not start at the root or end at a leaf; a single node also counts as a path.\n\nNode definition: struct TreeNode { int val; TreeNode *left; TreeNode *right; }. The tree in the input uses LeetCode's level-order representation, where null marks an empty slot.",
        "constraints": [
            "The number of nodes is in the range [0, 1000]; -10^9 ≤ Node.val ≤ 10^9; -1000 ≤ targetSum ≤ 1000"
        ],
        "complexity": {
            "time": "O(n) (prefix sums + hash table, average)",
            "space": "O(h) for the hash table and recursion stack, O(n) in the worst case.",
            "officialRequirement": "No official complexity requirement.",
            "notes": "n is the number of nodes and h is the tree height. Path sums can exceed 32 bits (up to 1000 × 10^9), so accumulate with 64-bit integers. Brute-force enumeration downward from each node is O(n·h), which also passes at n = 1000. The largest local test is a 1000-node single chain with values alternating ±10^9; there are also an all-zero chain (answer 500500) and a random tree with large values."
        },
        "acmIoSpec": "stdin: line 1 is m (number of level-order tokens, 0 for an empty tree); line 2 has m tokens (integers or null), the tree's LeetCode level-order representation; line 3 is the integer targetSum. stdout: one integer, the number of paths."
    },
    "constraints": ["节点数在 [0, 1000] 范围内；-10^9 ≤ Node.val ≤ 10^9；-1000 ≤ targetSum ≤ 1000"],
    "complexity": {
        "time": "O(n)（前缀和 + 哈希表，平均）",
        "space": "O(h) 哈希表与递归栈，最坏 O(n)。",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为节点数，h 为树高。路径和可能超出 32 位（最多 1000 × 10^9），需用 64 位整数累加。对每个节点向下暴力枚举为 O(n·h)，在 n = 1000 时同样可过。本地最大测试为 1000 个节点、值交替 ±10^9 的单链，另含全 0 链（答案 500500）与大值随机树。",
        "sourceUrl": "https://leetcode.com/problems/path-sum-iii/",
    },
    "cpp": {
        "functionTemplate": "/**\n * struct TreeNode {\n *     int val;\n *     TreeNode *left;\n *     TreeNode *right;\n * };\n */\nclass Solution {\npublic:\n    int pathSum(TreeNode* root, int targetSum) {\n        return 0; // Number of downward paths summing to targetSum.\n    }\n};",
        "acmIoSpec": "stdin：第一行 m（层序 token 数，空树为 0）；第二行 m 个 token（整数或 null），为树的 LeetCode 层序表示；第三行整数 targetSum。stdout：一个整数，路径数目。",
    },
    "io": {
        "method": "pathSum",
        "params": [{"name": "root", "type": "TreeNode"}, {"name": "targetSum", "type": "int"}],
        "returns": "int",
    },
}


def solve(data):
    # Brute force: for every node, walk the path back up to the root.
    root, target = build(data["root"]), data["targetSum"]
    count, stack = 0, [(root, ())] if root else []
    while stack:
        node, path = stack.pop()
        path = path + (node.val,)
        s = 0
        for v in reversed(path):
            s += v
            count += s == target
        for c in (node.left, node.right):
            if c:
                stack.append((c, path))
    return count


r = rng(437)
B = 10**9


def tree(shape, values):
    return shaped(shape, values)


alt = [B if i % 2 == 0 else -B for i in range(1000)]
alt[500] = 7  # breaks the cancellation in the middle
small = lambda: r.randint(-5, 5)
tests = [
    {"id": "t1", "input": {"root": [10, 5, -3, 3, 2, None, 11, 3, -2, None, 1], "targetSum": 8}, "sample": True, "expected": 3},
    {"id": "t2", "input": {"root": [5, 4, 8, 11, None, 13, 4, 7, 2, None, None, 5, 1], "targetSum": 22}, "sample": True, "expected": 3},
    {"id": "empty", "input": {"root": [], "targetSum": 0}, "sample": False},
    {"id": "single-hit", "input": {"root": [-1000], "targetSum": -1000}, "sample": False},
    {"id": "overflow-trap", "input": {"root": [B, B, None, B, None, -B, None, -B, None, -B], "targetSum": 0}, "sample": False},
    {"id": "int-wrap-trap", "input": {"root": [1000000000, 1000000000, None, 294967296, None, 1000000000, None, 1000000000, None, 1000000000, None, 1000000000], "targetSum": 0}, "sample": False},
    {"id": "zero-chain-1000", "input": {"root": tree(chain_shape(1000, "L"), [0] * 1000), "targetSum": 0}, "sample": False},
    {"id": "small-random-1000", "input": {"root": tree(random_shape(r, 1000), [small() for _ in range(1000)]), "targetSum": 3}, "sample": False},
    {"id": "big-random-1000", "input": {"root": tree(random_shape(r, 1000), [r.choice([-B, B, r.randint(-1000, 1000)]) for _ in range(1000)]), "targetSum": r.randint(-1000, 1000)}, "sample": False},
    {"id": "max-size-1000", "input": {"root": tree(chain_shape(1000, "Z"), alt), "targetSum": 0}, "sample": False},
]
write(problem, tests, solve)
