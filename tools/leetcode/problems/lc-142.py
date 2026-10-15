import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-142",
    "title": "环形链表 II",
    "difficulty": "medium",
    "tags": ["哈希表", "链表", "双指针"],
    "statement": (
        "给定单链表的头节点 head，返回链表开始入环的第一个节点；若链表无环，返回 nullptr。不允许修改链表。\n\n"
        "评测数据中的 pos 表示尾节点的 next 指向的节点下标（从 0 开始），pos = -1 表示无环。pos 只用于构造链表，不会作为参数传给函数。"
        "评测输出你返回的节点在链表中的下标，返回 nullptr 时输出 -1；返回不属于该链表的节点判为错误。\n\n"
        "节点定义：struct ListNode { int val; ListNode* next; };"
    ),
    "en": {'title': 'Linked List Cycle II',
     'tags': ['Hash Table', 'Linked List', 'Two Pointers'],
     'statement': 'Given the head of a singly linked list, return the first node at which the list '
                  'enters its cycle; if the list has no cycle, return nullptr. You must not modify the '
                  'list.\n'
                  '\n'
                  "In the test data, pos is the index (0-based) of the node that the tail's next "
                  'pointer refers to; pos = -1 means there is no cycle. pos is only used to build the '
                  'list and is not passed to your function. The judge prints the index of the node you '
                  'return within the list, or -1 if you return nullptr; returning a node that does not '
                  'belong to the list is judged wrong.\n'
                  '\n'
                  'Node definition: struct ListNode { int val; ListNode* next; };',
     'constraints': ['0 ≤ number of nodes ≤ 10000; -100000 ≤ node value ≤ 100000',
                     'pos is -1 or a valid index in the list'],
     'complexity': {'time': 'O(n) (local training target)',
                    'space': 'Fast/slow pointers use O(1) extra space; a hash set with O(n) also '
                             'passes',
                    'officialRequirement': 'Official follow-up asks for only O(1) (constant) extra '
                                           'memory.',
                    'notes': 'n is the number of nodes. Floyd: after the fast and slow pointers meet, '
                             'move one pointer back to the head and advance both at the same speed; '
                             'where they meet again is the cycle entry (if a is the distance from the '
                             'head to the entry, then a ≡ distance from the meeting point to the entry '
                             '(mod cycle length)). The largest test uses the official limit n = '
                             '10000.'},
     'acmIoSpec': 'stdin: line 1 is the node count n; line 2 has n integers, the list values; line 3 '
                  'is pos (the index the tail links to, -1 for no cycle). stdout: one integer, the '
                  '0-based index of the cycle-entry node, or -1 if there is no cycle.'},
    "constraints": [
        "0 ≤ 节点数 ≤ 10000；-100000 ≤ 节点值 ≤ 100000",
        "pos 为 -1 或链表中的合法下标",
    ],
    "complexity": {
        "time": "O(n)（本地训练目标）",
        "space": "快慢指针 O(1) 额外空间；哈希集合 O(n) 亦可通过",
        "officialRequirement": "官方 follow-up 要求仅用 O(1)（常数）额外内存。",
        "notes": "n 为节点数。Floyd：快慢指针相遇后，一个指针回到表头，两者同速前进，再次相遇处即入环点（设表头到入环点距离 a，则 a ≡ 相遇点到入环点的距离 (mod 环长)）。最大测试为官方上限 n = 10000。",
        "sourceUrl": "https://leetcode.com/problems/linked-list-cycle-ii/",
    },
    "cpp": {
        "functionTemplate": "// ListNode is provided: int val; ListNode* next;\nclass Solution {\npublic:\n    ListNode *detectCycle(ListNode *head) {\n        return nullptr; // Node where the cycle begins, or nullptr.\n    }\n};",
        "acmIoSpec": "stdin：第一行节点数 n；第二行 n 个整数表示链表值；第三行 pos（尾节点连向的下标，-1 表示无环）。stdout：一个整数，入环节点的下标（从 0 开始），无环输出 -1。",
    },
    "io": {
        "method": "detectCycle",
        "params": [{"name": "head", "type": "ListNode"}, {"name": "pos", "type": "int"}],
        "returns": "int",
        "harness": "lc-142.cpp",
    },
}


def solve(d):
    # First index visited twice while following successor indices.
    vals, pos = d["head"], d["pos"]
    n = len(vals)
    nxt = [i + 1 if i + 1 < n else (pos if pos >= 0 else None) for i in range(n)]
    seen, i = set(), 0 if n else None
    while i is not None:
        if i in seen:
            return i
        seen.add(i)
        i = nxt[i]
    return -1


r = rng(142)
tests = [
    {"id": "t1", "sample": True, "input": {"head": [3, 2, 0, -4], "pos": 1}, "expected": 1},
    {"id": "t2", "sample": True, "input": {"head": [1, 2], "pos": 0}, "expected": 0},
    {"id": "t3", "sample": True, "input": {"head": [1], "pos": -1}, "expected": -1},
    {"id": "empty", "sample": False, "input": {"head": [], "pos": -1}},
    {"id": "self-loop", "sample": False, "input": {"head": [5], "pos": 0}},
    {"id": "tail-self-loop", "sample": False, "input": {"head": [1, 2, 3, 4, 5], "pos": 4}},
    {"id": "equal-values", "sample": False, "input": {"head": [7] * 40, "pos": 17}},
    {"id": "long-tail-short-cycle", "sample": False, "input": {"head": list(range(100)), "pos": 97}},
    {"id": "short-tail-long-cycle", "sample": False, "input": {"head": list(range(100)), "pos": 2}},
    {"id": "random-300", "sample": False, "input": {"head": [r.randint(-100000, 100000) for _ in range(300)], "pos": 211}},
    {"id": "max-size-10000", "sample": False, "input": {"head": [r.randint(-100000, 100000) for _ in range(10000)], "pos": 4321}},
    {"id": "max-size-10000-acyclic", "sample": False, "input": {"head": [r.randint(-100000, 100000) for _ in range(10000)], "pos": -1}},
    {"id": "max-size-10000-equal", "sample": False, "input": {"head": [0] * 10000, "pos": 9998}},
]
write(problem, tests, solve)
