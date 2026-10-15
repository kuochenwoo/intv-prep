import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-160",
    "title": "相交链表",
    "difficulty": "easy",
    "tags": ["哈希表", "链表", "双指针"],
    "statement": (
        "给定两个单链表的头节点 headA 和 headB，找出两条链表开始共享的第一个节点（按节点身份而非节点值判断），"
        "不存在则返回 nullptr。整个结构中不含环，函数返回后两条链表必须保持原有结构。\n\n"
        "评测数据沿用 LeetCode 的描述方式：intersectVal 为相交节点的值（不相交时为 0），listA、listB 为两条链表的值序列，"
        "skipA、skipB 分别是从各自表头走到相交节点需要跳过的节点数；评测程序据此让 listB 的后半段直接复用 listA 的节点。"
        "评测输出你返回的节点在链表 A 中的下标（从 0 开始），返回 nullptr 时输出 -1；返回不属于链表 A 的节点判为错误。"
        "注意：两段值相同但属于不同节点的链表并不相交。\n\n"
        "节点定义：struct ListNode { int val; ListNode* next; };"
    ),
    "constraints": [
        "listA 长度为 m，listB 长度为 n；1 ≤ m, n ≤ 30000；1 ≤ 节点值 ≤ 100000",
        "0 ≤ skipA ≤ m；0 ≤ skipB ≤ n；不相交时 intersectVal = 0，且 skipA = m、skipB = n",
        "相交时 intersectVal == listA[skipA] == listB[skipB]，两条链表从该处起完全相同",
    ],
    "complexity": {
        "time": "O(m + n)（本地训练目标）",
        "space": "双指针 O(1) 额外空间；哈希集合 O(m) 亦可通过",
        "officialRequirement": "官方 follow-up 要求 O(m + n) 时间且仅用 O(1) 额外内存。",
        "notes": "m、n 为两条链表长度，输入链表不计入额外空间。双指针各走完自己的链表后切到对方表头，两者走过的总长度相同，必在交点（或同时到达 nullptr）相遇。最大测试为官方上限 m = n = 30000、仅最后一个节点相交，O(mn) 的暴力比较会超时。",
        "sourceUrl": "https://leetcode.com/problems/intersection-of-two-linked-lists/",
    },
    "cpp": {
        "functionTemplate": "// ListNode is provided: int val; ListNode* next;\nclass Solution {\npublic:\n    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {\n        return nullptr; // First shared node, or nullptr.\n    }\n};",
        "acmIoSpec": "stdin：第一行 intersectVal；第二行 m；第三行 m 个整数 listA；第四行 n；第五行 n 个整数 listB；第六行 skipA；第七行 skipB。stdout：一个整数，相交节点在链表 A 中的下标（从 0 开始），不相交输出 -1。",
    },
    "io": {
        "method": "getIntersectionNode",
        "params": [
            {"name": "intersectVal", "type": "int"},
            {"name": "listA", "type": "ListNode"},
            {"name": "listB", "type": "ListNode"},
            {"name": "skipA", "type": "int"},
            {"name": "skipB", "type": "int"},
        ],
        "returns": "int",
        "harness": "lc-160.cpp",
    },
}


class Node:
    __slots__ = ("val", "next")

    def __init__(self, val):
        self.val, self.next = val, None


def solve(d):
    # Build real shared nodes, then find the first node of B whose identity is in A.
    a, b, sa, sb = d["listA"], d["listB"], d["skipA"], d["skipB"]
    nodes_a = [Node(v) for v in a]
    for x, y in zip(nodes_a, nodes_a[1:]):
        x.next = y
    head_b, tail = None, None
    for v in b[:sb]:
        node = Node(v)
        if tail:
            tail.next = node
        else:
            head_b = node
        tail = node
    if sb < len(b):
        shared = nodes_a[sa]
        if tail:
            tail.next = shared
        else:
            head_b = shared
    where = {id(n): i for i, n in enumerate(nodes_a)}
    p = head_b
    while p:
        if id(p) in where:
            return where[id(p)]
        p = p.next
    return -1


def case(a_prefix, b_prefix, common):
    a, b = a_prefix + common, b_prefix + common
    return {"intersectVal": common[0] if common else 0, "listA": a, "listB": b,
            "skipA": len(a_prefix), "skipB": len(b_prefix)}


r = rng(160)
tests = [
    {"id": "t1", "sample": True, "input": case([4, 1], [5, 6, 1], [8, 4, 5]), "expected": 2},
    {"id": "t2", "sample": True, "input": case([1, 9, 1], [3], [2, 4]), "expected": 3},
    {"id": "t3", "sample": True, "input": case([2, 6, 4], [1, 5], []), "expected": -1},
    {"id": "same-head", "sample": False, "input": case([], [], [3, 1, 4])},
    {"id": "single-shared", "sample": False, "input": case([], [], [7])},
    {"id": "single-separate-equal", "sample": False, "input": case([7], [7], [])},
    {"id": "equal-values-not-shared", "sample": False, "input": case([1, 2, 3], [1, 2, 3], [])},
    {"id": "b-is-suffix-of-a", "sample": False, "input": case([5, 5, 5, 5], [], [5, 5])},
    {"id": "last-node", "sample": False, "input": case([1, 2, 3, 4], [9], [2])},
    {"id": "random-mid", "sample": False,
     "input": case([r.randint(1, 5) for _ in range(120)], [r.randint(1, 5) for _ in range(37)],
                   [r.randint(1, 5) for _ in range(60)])},
    {"id": "random-disjoint", "sample": False,
     "input": case([r.randint(1, 100000) for _ in range(300)], [r.randint(1, 100000) for _ in range(500)], [])},
    # Small values keep the file compact and make value-based matching wrong.
    {"id": "max-size-30000", "sample": False,
     "input": case([r.randint(1, 999) for _ in range(29999)], [r.randint(1, 999) for _ in range(29999)], [r.randint(1, 999)])},
    {"id": "worst-case-disjoint-10000", "sample": False,
     "input": case([r.randint(1, 999) for _ in range(10000)], [r.randint(1, 999) for _ in range(10000)], [])},
]
write(problem, tests, solve)
