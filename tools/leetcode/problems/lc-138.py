import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

PRELUDE = (
    "class Node {\npublic:\n    int val;\n    Node* next;\n    Node* random;\n\n"
    "    Node(int _val) {\n        val = _val;\n        next = NULL;\n        random = NULL;\n    }\n};"
)

problem = {
    "id": "lc-138",
    "title": "随机链表的复制",
    "difficulty": "medium",
    "tags": ["哈希表", "链表"],
    "statement": (
        "给定一个长度为 n 的链表，每个节点除 next 外还有一个 random 指针，可以指向链表中任意节点或为空。"
        "请构造这条链表的深拷贝：新链表由 n 个全新节点组成，值与原节点一一对应，且新节点的 next 和 random 只指向新链表中的节点，"
        "使新链表与原链表结构完全相同。返回新链表的头节点；原链表在返回时必须保持不变。\n\n"
        "数据用二维数组表示：第 i 行为 [val, randomIndex]，randomIndex 为 random 指向的节点下标（从 0 开始），空指针为 -1。"
        "评测以同样的格式输出你返回的链表；若返回的链表含有原链表的节点、random 指向新链表之外，或原链表被修改，判为错误。\n\n"
        "节点定义（已提供）：class Node { public: int val; Node* next; Node* random; Node(int _val); };"
    ),
    "en": {'title': 'Copy List with Random Pointer',
     'tags': ['Hash Table', 'Linked List'],
     'statement': 'You are given a linked list of length n in which each node has, besides next, a random '
                  'pointer that may point to any node in the list or be null. Construct a deep copy of the list: '
                  'the new list consists of n brand-new nodes whose values correspond one-to-one with the '
                  "original nodes, and the new nodes' next and random pointers point only to nodes of the new "
                  'list, so that the new list has exactly the same structure as the original. Return the head of '
                  'the new list; the original list must be unchanged when you return.\n'
                  '\n'
                  'Data is represented as a 2D array: row i is [val, randomIndex], where randomIndex is the '
                  '0-based index of the node random points to, or -1 for null. The judge prints your returned '
                  'list in the same format; it is judged wrong if the returned list contains original nodes, a '
                  'random pointer points outside the new list, or the original list was modified.\n'
                  '\n'
                  'Node definition (provided): class Node { public: int val; Node* next; Node* random; Node(int '
                  '_val); };',
     'constraints': ['0 ≤ n ≤ 1000; -10000 ≤ Node.val ≤ 10000',
                     'Node.random is null or points to some node in the list'],
     'complexity': {'time': 'O(n) (local training target)',
                    'space': 'Interleaving method: O(1) extra space (excluding the new output list); an O(n) '
                             'hash map approach also passes',
                    'officialRequirement': 'No official complexity requirement.',
                    'notes': 'n is the number of nodes; the returned new list itself is not counted as extra '
                             "space. Interleaving: insert each node's copy right after it, set each copy's "
                             'random via orig->random->next, then split the two lists and restore the original. '
                             'The largest test is the official limit n = 1000.'},
     'acmIoSpec': 'stdin: line 1 is the node count n; then n lines, each "2 val randomIndex" (2 is the number of '
                  'integers that follow on the line; randomIndex -1 means null). stdout: line 1 is n; then n '
                  'lines, each "val randomIndex" describing the copied list.'},
    "constraints": [
        "0 ≤ n ≤ 1000；-10000 ≤ Node.val ≤ 10000",
        "Node.random 为空或指向链表中的某个节点",
    ],
    "complexity": {
        "time": "O(n)（本地训练目标）",
        "space": "节点交织法 O(1) 额外空间（不计输出的新链表）；哈希表映射 O(n) 亦可通过",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为节点数，返回的新链表本身不计入额外空间。交织法：在每个原节点后插入其拷贝，借 orig->random->next 设置拷贝的 random，最后拆分两条链表并恢复原链表。最大测试为官方上限 n = 1000。",
        "sourceUrl": "https://leetcode.com/problems/copy-list-with-random-pointer/",
    },
    "cpp": {
        "functionTemplate": "// Node is provided: int val; Node* next; Node* random; Node(int _val);\nclass Solution {\npublic:\n    Node* copyRandomList(Node* head) {\n        return nullptr; // Deep copy with next and random pointers.\n    }\n};",
        "acmIoSpec": "stdin：第一行节点数 n；随后 n 行，每行为「2 val randomIndex」（2 是该行后续整数个数，randomIndex 为 -1 表示空）。stdout：第一行 n；随后 n 行，每行「val randomIndex」描述复制后的链表。",
    },
    "io": {
        "method": "copyRandomList",
        "params": [{"name": "head", "type": "int[][]"}],
        "returns": "int[][]",
        "harness": "lc-138.cpp",
        "prelude": PRELUDE,
    },
}


class N:
    def __init__(self, val):
        self.val, self.next, self.random = val, None, None


def solve(rows):
    # Build real nodes, deep-copy with an identity map, then serialize the copy.
    orig = [N(v) for v, _ in rows]
    for i, (_, ri) in enumerate(rows):
        orig[i].next = orig[i + 1] if i + 1 < len(orig) else None
        orig[i].random = orig[ri] if ri >= 0 else None
    copy = {None: None}
    for o in orig:
        copy[o] = N(o.val)
    for o in orig:
        copy[o].next, copy[o].random = copy[o.next], copy[o.random]
    nodes, p = [], copy[orig[0]] if orig else None
    while p:
        nodes.append(p)
        p = p.next
    where = {id(x): i for i, x in enumerate(nodes)}
    return [[x.val, where[id(x.random)] if x.random else -1] for x in nodes]


def rand_list(r, n, lo=-10000, hi=10000, p_null=0.2):
    return [[r.randint(lo, hi), -1 if r.random() < p_null else r.randrange(n)] for _ in range(n)]


r = rng(138)
tests = [
    {"id": "t1", "sample": True, "input": [[7, -1], [13, 0], [11, 4], [10, 2], [1, 0]],
     "expected": [[7, -1], [13, 0], [11, 4], [10, 2], [1, 0]]},
    {"id": "t2", "sample": True, "input": [[1, 1], [2, 1]], "expected": [[1, 1], [2, 1]]},
    {"id": "t3", "sample": True, "input": [[3, -1], [3, 0], [3, -1]], "expected": [[3, -1], [3, 0], [3, -1]]},
    {"id": "empty", "sample": False, "input": []},
    {"id": "single-null", "sample": False, "input": [[-5, -1]]},
    {"id": "single-self", "sample": False, "input": [[9, 0]]},
    {"id": "all-to-last", "sample": False, "input": [[i, 9] for i in range(10)]},
    {"id": "all-self", "sample": False, "input": [[0, i] for i in range(20)]},
    {"id": "random-100", "sample": False, "input": rand_list(r, 100, 0, 3)},
    {"id": "max-size-1000", "sample": False, "input": rand_list(r, 1000)},
    {"id": "max-size-1000-backwards", "sample": False, "input": [[r.randint(-10000, 10000), 999 - i] for i in range(1000)]},
]
write(problem, tests, solve)
