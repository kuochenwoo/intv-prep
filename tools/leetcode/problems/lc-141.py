import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-141",
    "title": "环形链表",
    "difficulty": "easy",
    "tags": ["哈希表", "链表", "双指针"],
    "statement": (
        "给定单链表的头节点 head，判断链表中是否存在环：若沿 next 指针不断前进会再次到达某个已经访问过的节点，则存在环，返回 true，否则返回 false。\n\n"
        "评测数据中的 pos 表示尾节点的 next 指向的节点下标（从 0 开始），pos = -1 表示无环。pos 只用于构造链表，不会作为参数传给函数。\n\n"
        "节点定义：struct ListNode { int val; ListNode* next; };"
    ),
    "constraints": [
        "0 ≤ 节点数 ≤ 10000；-100000 ≤ 节点值 ≤ 100000",
        "pos 为 -1 或链表中的合法下标",
    ],
    "complexity": {
        "time": "O(n)（本地训练目标）",
        "space": "快慢指针 O(1) 额外空间；哈希集合 O(n) 亦可通过",
        "officialRequirement": "官方 follow-up 要求仅用 O(1)（常数）额外内存。",
        "notes": "n 为节点数。Floyd 判圈：快指针每次走两步、慢指针走一步，有环则必在环内相遇，快指针最多走约 2n 步。最大测试为官方上限 n = 10000。",
        "sourceUrl": "https://leetcode.com/problems/linked-list-cycle/",
    },
    "cpp": {
        "functionTemplate": "// ListNode is provided: int val; ListNode* next;\nclass Solution {\npublic:\n    bool hasCycle(ListNode *head) {\n        return false; // Does the list loop back?\n    }\n};",
        "acmIoSpec": "stdin：第一行节点数 n；第二行 n 个整数表示链表值；第三行 pos（尾节点连向的下标，-1 表示无环）。stdout：true 或 false。",
    },
    "io": {
        "method": "hasCycle",
        "params": [{"name": "head", "type": "ListNode"}, {"name": "pos", "type": "int"}],
        "returns": "bool",
        "harness": "lc-141.cpp",
    },
}


def solve(d):
    # Walk successor indices with a visited set (independent of Floyd in C++).
    vals, pos = d["head"], d["pos"]
    n = len(vals)
    nxt = [i + 1 if i + 1 < n else (pos if pos >= 0 else None) for i in range(n)]
    seen, i = set(), 0 if n else None
    while i is not None:
        if i in seen:
            return True
        seen.add(i)
        i = nxt[i]
    return False


r = rng(141)
tests = [
    {"id": "t1", "sample": True, "input": {"head": [3, 2, 0, -4], "pos": 1}, "expected": True},
    {"id": "t2", "sample": True, "input": {"head": [1, 2], "pos": 0}, "expected": True},
    {"id": "t3", "sample": True, "input": {"head": [1], "pos": -1}, "expected": False},
    {"id": "empty", "sample": False, "input": {"head": [], "pos": -1}},
    {"id": "self-loop", "sample": False, "input": {"head": [5], "pos": 0}},
    {"id": "two-no-cycle", "sample": False, "input": {"head": [1, 1], "pos": -1}},
    {"id": "tail-self-loop", "sample": False, "input": {"head": [1, 2, 3, 4, 5], "pos": 4}},
    {"id": "equal-values-no-cycle", "sample": False, "input": {"head": [7] * 50, "pos": -1}},
    {"id": "random-300", "sample": False, "input": {"head": [r.randint(-100000, 100000) for _ in range(300)], "pos": 123}},
    {"id": "max-size-10000", "sample": False, "input": {"head": [r.randint(-100000, 100000) for _ in range(10000)], "pos": 0}},
    {"id": "max-size-10000-acyclic", "sample": False, "input": {"head": [r.randint(-100000, 100000) for _ in range(10000)], "pos": -1}},
    {"id": "max-size-10000-tail-loop", "sample": False, "input": {"head": [0] * 10000, "pos": 9999}},
]
write(problem, tests, solve)
