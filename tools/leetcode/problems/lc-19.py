import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-19",
    "title": "删除链表的倒数第 N 个结点",
    "difficulty": "medium",
    "tags": ["链表", "双指针"],
    "statement": "给定链表的头节点 head 和整数 n，删除链表的倒数第 n 个节点（n = 1 表示最后一个节点），返回删除后链表的头节点。\n\n节点定义：struct ListNode { int val; ListNode* next; };",
    "constraints": ["1 ≤ 节点数 sz ≤ 30；0 ≤ 节点值 ≤ 100；1 ≤ n ≤ sz"],
    "complexity": {
        "time": "O(sz)（本地训练目标）",
        "space": "O(1) 额外空间",
        "officialRequirement": "官方 follow-up 要求只遍历一趟完成（未额外要求空间）。",
        "notes": "sz 为节点数，输入链表不计入额外空间。哑节点 + 快慢指针：快指针先走 n 步，再同步前进至表尾，慢指针的下一个节点即待删节点。先求长度再删除的两趟做法同样能通过本地测试。最大测试为官方上限 sz = 30。",
        "sourceUrl": "https://leetcode.com/problems/remove-nth-node-from-end-of-list/",
    },
    "cpp": {
        "functionTemplate": "// ListNode is provided: int val; ListNode* next;\nclass Solution {\npublic:\n    ListNode* removeNthFromEnd(ListNode* head, int n) {\n        return head; // Head after removing the n-th node from the end.\n    }\n};",
        "acmIoSpec": "stdin：第一行节点数 sz；第二行 sz 个整数表示链表值；第三行 n。stdout：一行，删除后链表的值，以空格分隔；结果为空时输出空行。",
    },
    "io": {
        "method": "removeNthFromEnd",
        "params": [{"name": "head", "type": "ListNode"}, {"name": "n", "type": "int"}],
        "returns": "ListNode",
    },
}


def solve(d):
    v, n = d["head"], d["n"]
    k = len(v) - n
    return v[:k] + v[k + 1:]


r = rng(19)
tests = [
    {"id": "t1", "sample": True, "input": {"head": [1, 2, 3, 4, 5], "n": 2}, "expected": [1, 2, 3, 5]},
    {"id": "t2", "sample": True, "input": {"head": [1], "n": 1}, "expected": []},
    {"id": "t3", "sample": True, "input": {"head": [1, 2], "n": 1}, "expected": [1]},
    {"id": "remove-head", "sample": False, "input": {"head": [1, 2], "n": 2}},
    {"id": "remove-head-long", "sample": False, "input": {"head": [4, 3, 2, 1, 0], "n": 5}},
    {"id": "equal-values", "sample": False, "input": {"head": [7, 7, 7, 7], "n": 3}},
    {"id": "random", "sample": False, "input": {"head": [r.randint(0, 100) for _ in range(12)], "n": 7}},
    {"id": "max-size-30", "sample": False, "input": {"head": [r.randint(0, 100) for _ in range(30)], "n": 30}},
    {"id": "max-size-30-last", "sample": False, "input": {"head": [r.randint(0, 100) for _ in range(30)], "n": 1}},
]
write(problem, tests, solve)
