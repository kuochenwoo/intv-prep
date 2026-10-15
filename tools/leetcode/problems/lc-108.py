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
    "id": "lc-108",
    "title": "将有序数组转换为二叉搜索树",
    "difficulty": "easy",
    "tags": ["树", "二叉搜索树", "数组", "分治", "二叉树"],
    "statement": "给定一个严格递增的整数数组 nums，构造一棵高度平衡的二叉搜索树并返回其根节点，使其中序遍历恰好为 nums。高度平衡指每个节点左右子树的高度差不超过 1。答案可能不唯一，返回任意一棵满足要求的树即可。\n\n节点定义：struct TreeNode { int val; TreeNode *left; TreeNode *right; }。输出的树为 LeetCode 层序表示，null 表示空位。",
    "en": {'title': 'Convert Sorted Array to Binary Search Tree',
     'tags': ['Tree', 'Binary Search Tree', 'Array', 'Divide and Conquer', 'Binary Tree'],
     'statement': 'Given a strictly increasing integer array nums, build a height-balanced binary search tree '
                  'whose inorder traversal is exactly nums, and return its root. Height-balanced means the '
                  "heights of every node's left and right subtrees differ by at most 1. The answer may not be "
                  'unique; any tree meeting the requirements is accepted.\n'
                  '\n'
                  'Node definition: struct TreeNode { int val; TreeNode *left; TreeNode *right; }. The output '
                  'tree uses LeetCode level-order representation, with null marking empty positions.',
     'constraints': ['1 ≤ nums.length ≤ 10^4; -10^4 ≤ nums[i] ≤ 10^4; nums is strictly increasing'],
     'complexity': {'time': 'O(n)',
                    'space': 'O(log n) recursion stack (taking the midpoint each time keeps the depth about log '
                             'n); the n new nodes are output and not counted.',
                    'officialRequirement': 'No official complexity requirement.',
                    'notes': 'n is the array length. Using the midpoint of the range as the root and recursing '
                             'on both sides guarantees balance. The checker accepts any height-balanced BST '
                             'whose inorder traversal is nums (it need not match the reference answer). The '
                             'largest local test has n = 10^4. The output is large, so the output limit is '
                             'raised to 256 KiB.'},
     'acmIoSpec': 'stdin: line 1 is n; line 2 has n strictly increasing integers nums. stdout: one line with the '
                  'level-order tokens of the constructed balanced BST (integers or null, space-separated, '
                  'trailing nulls omitted). Any valid answer is accepted.'},
    "constraints": ["1 ≤ nums.length ≤ 10^4；-10^4 ≤ nums[i] ≤ 10^4；nums 严格递增"],
    "complexity": {
        "time": "O(n)",
        "space": "O(log n) 递归栈（每次取中点，深度约 log n）；新建的 n 个节点属于输出，不计入。",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为数组长度。取区间中点为根并递归两侧即可保证平衡。判题器接受任意高度平衡且中序为 nums 的 BST（不要求与参考答案相同）。本地最大测试为 n = 10^4。输出较大，输出上限调至 256 KiB。",
        "sourceUrl": "https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/",
    },
    "cpp": {
        "functionTemplate": "/**\n * struct TreeNode {\n *     int val;\n *     TreeNode *left;\n *     TreeNode *right;\n *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}\n * };\n */\nclass Solution {\npublic:\n    TreeNode* sortedArrayToBST(vector<int>& nums) {\n        return nullptr; // Root of any height-balanced BST with in-order equal to nums.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个严格递增的整数 nums。stdout：一行，所构造的平衡 BST 的层序 token（整数或 null，空格分隔，省略末尾 null）。任意合法答案均可。",
    },
    # A 10^4-node level order with nulls is ~100 KiB of output.
    "limits": {"timeLimitMs": 1000, "memoryLimitMb": 128, "outputLimitKb": 256},
    "io": {
        "method": "sortedArrayToBST",
        "params": [{"name": "nums", "type": "int[]"}],
        "returns": "TreeNode",
        "checker": "balanced_bst",
    },
}


def solve(nums):
    def make(lo, hi):
        if lo > hi:
            return None
        mid = (lo + hi) // 2
        node = N(nums[mid])
        node.left, node.right = make(lo, mid - 1), make(mid + 1, hi)
        return node

    return dump(make(0, len(nums) - 1))


def balanced_bst(nums, tokens):
    order = []

    def walk(node):
        if not node:
            return 0
        a = walk(node.left)
        order.append(node.val)
        b = walk(node.right)
        if a < 0 or b < 0 or abs(a - b) > 1:
            return -1
        return 1 + max(a, b)

    return walk(build(tokens)) >= 0 and order == nums


def check(nums, expected, solved):
    return balanced_bst(nums, expected) and balanced_bst(nums, solved)


r = rng(108)
tests = [
    {"id": "t1", "input": [-10, -3, 0, 5, 9], "sample": True, "expected": [0, -3, 9, -10, None, 5]},
    {"id": "t2", "input": [1, 3], "sample": True, "expected": [3, 1]},
    {"id": "single", "input": [-10000], "sample": False},
    {"id": "seven", "input": [1, 2, 3, 4, 5, 6, 7], "sample": False},
    {"id": "random-100", "input": sorted(r.sample(range(-10000, 10001), 100)), "sample": False},
    {"id": "pow2-minus-1-8191", "input": sorted(r.sample(range(-10000, 10001), 8191)), "sample": False},
    {"id": "max-size-10000", "input": sorted(r.sample(range(-10000, 10001), 10000)), "sample": False},
]
for t in tests:
    if "expected" not in t:
        assert balanced_bst(t["input"], solve(t["input"]))
write(problem, tests, solve, check=check)
