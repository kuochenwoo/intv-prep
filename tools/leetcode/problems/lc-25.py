import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-25",
    "title": "K 个一组翻转链表",
    "difficulty": "hard",
    "tags": ["递归", "链表"],
    "statement": "给定链表头节点 head 和正整数 k，从表头开始每 k 个节点分为一组，把每组内部的节点顺序翻转；最后不足 k 个的节点保持原顺序。返回处理后的头节点。只能调整指针，不能修改节点的值。\n\n节点定义：struct ListNode { int val; ListNode* next; };",
    "constraints": ["节点数为 n；1 ≤ k ≤ n ≤ 5000；0 ≤ 节点值 ≤ 1000", "只能改变节点指针，不得修改节点值（本地评测仅比较值序列，无法检查这一点，请自觉遵守）"],
    "complexity": {
        "time": "O(n)（本地训练目标）",
        "space": "迭代 O(1) 额外空间；递归需 O(n/k) 调用栈",
        "officialRequirement": "官方 follow-up 要求仅用 O(1) 额外内存。",
        "notes": "n 为节点数，输入链表不计入额外空间。先确认剩余至少 k 个节点，再原地反转该组并接回前后两段。最大测试为官方上限 n = 5000（含 k = 1、k = 2、k = n 及不整除的情况）。",
        "sourceUrl": "https://leetcode.com/problems/reverse-nodes-in-k-group/",
    },
    "cpp": {
        "functionTemplate": "// ListNode is provided: int val; ListNode* next;\nclass Solution {\npublic:\n    ListNode* reverseKGroup(ListNode* head, int k) {\n        return head; // Reverse each full group of k nodes.\n    }\n};",
        "acmIoSpec": "stdin：第一行节点数 n；第二行 n 个整数表示链表值；第三行 k。stdout：一行，处理后链表的值，以空格分隔。",
    },
    "io": {
        "method": "reverseKGroup",
        "params": [{"name": "head", "type": "ListNode"}, {"name": "k", "type": "int"}],
        "returns": "ListNode",
    },
}


def solve(d):
    v, k = d["head"], d["k"]
    out = []
    for i in range(0, len(v), k):
        chunk = v[i:i + k]
        out += chunk[::-1] if len(chunk) == k else chunk
    return out


r = rng(25)
big = lambda: [r.randint(0, 1000) for _ in range(5000)]
tests = [
    {"id": "t1", "sample": True, "input": {"head": [1, 2, 3, 4, 5], "k": 2}, "expected": [2, 1, 4, 3, 5]},
    {"id": "t2", "sample": True, "input": {"head": [1, 2, 3, 4, 5], "k": 3}, "expected": [3, 2, 1, 4, 5]},
    {"id": "single", "sample": False, "input": {"head": [7], "k": 1}},
    {"id": "k-equals-n", "sample": False, "input": {"head": [1, 2, 3, 4], "k": 4}},
    {"id": "k-one", "sample": False, "input": {"head": [5, 4, 3], "k": 1}},
    {"id": "exact-multiple", "sample": False, "input": {"head": list(range(12)), "k": 3}},
    {"id": "equal-values", "sample": False, "input": {"head": [2] * 9, "k": 4}},
    {"id": "random", "sample": False, "input": {"head": [r.randint(0, 1000) for _ in range(103)], "k": 7}},
    {"id": "max-size-5000-k2", "sample": False, "input": {"head": big(), "k": 2}},
    {"id": "max-size-5000-k-n", "sample": False, "input": {"head": big(), "k": 5000}},
    {"id": "max-size-5000-k-uneven", "sample": False, "input": {"head": big(), "k": 777}},
]
write(problem, tests, solve)
