import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-234",
    "title": "回文链表",
    "difficulty": "easy",
    "tags": ["栈", "递归", "链表", "双指针"],
    "statement": "给定单链表的头节点 head，判断链表的值序列正着读和反着读是否相同（即是否为回文），是则返回 true，否则返回 false。\n\n节点定义：struct ListNode { int val; ListNode* next; };",
    "constraints": ["1 ≤ 节点数 ≤ 100000；0 ≤ 节点值 ≤ 9"],
    "complexity": {
        "time": "O(n)（本地训练目标）",
        "space": "快慢指针找中点 + 原地反转后半段为 O(1) 额外空间；复制到数组或用栈为 O(n)，同样能通过本地测试",
        "officialRequirement": "官方 follow-up 要求 O(n) 时间且 O(1) 额外空间。",
        "notes": "n 为节点数，输入链表不计入额外空间。O(1) 解法会临时修改链表，比较后应把后半段反转回来；递归解法的调用栈为 O(n)，在 n = 100000 时有栈溢出风险。最大测试为官方上限 n = 100000。",
        "sourceUrl": "https://leetcode.com/problems/palindrome-linked-list/",
    },
    "cpp": {
        "functionTemplate": "// ListNode is provided: int val; ListNode* next;\nclass Solution {\npublic:\n    bool isPalindrome(ListNode* head) {\n        return false; // Same forwards and backwards?\n    }\n};",
        "acmIoSpec": "stdin：第一行节点数 n；第二行 n 个整数表示链表值。stdout：true 或 false。",
    },
    "io": {
        "method": "isPalindrome",
        "params": [{"name": "head", "type": "ListNode"}],
        "returns": "bool",
    },
}


def solve(v):
    return all(v[i] == v[-1 - i] for i in range(len(v) // 2))


r = rng(234)
half = [r.randint(0, 9) for _ in range(50000)]
near = half + half[::-1]
near[49999] = (near[49999] + 1) % 10
tests = [
    {"id": "t1", "sample": True, "input": [1, 2, 2, 1], "expected": True},
    {"id": "t2", "sample": True, "input": [1, 2], "expected": False},
    {"id": "single", "sample": False, "input": [0]},
    {"id": "two-equal", "sample": False, "input": [5, 5]},
    {"id": "odd-palindrome", "sample": False, "input": [1, 2, 3, 2, 1]},
    {"id": "odd-not", "sample": False, "input": [1, 2, 3, 1, 1]},
    {"id": "all-equal", "sample": False, "input": [7] * 101},
    {"id": "random-small", "sample": False, "input": [r.randint(0, 1) for _ in range(12)]},
    {"id": "max-size-100000", "sample": False, "input": half + half[::-1]},
    {"id": "max-size-100000-mid-mismatch", "sample": False, "input": near},
]
write(problem, tests, solve)
