import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-24",
    "title": "两两交换链表中的节点",
    "difficulty": "medium",
    "tags": ["递归", "链表"],
    "statement": "给定链表，把每相邻的两个节点交换位置（第 1、2 个交换，第 3、4 个交换，依此类推；节点数为奇数时最后一个保持不动），返回交换后的头节点。只能调整节点之间的指针，不能修改节点的值。\n\n节点定义：struct ListNode { int val; ListNode* next; };",
    "constraints": ["0 ≤ 节点数 ≤ 100；0 ≤ 节点值 ≤ 100", "只能交换节点，不得修改节点值（本地评测仅比较值序列，无法检查这一点，请自觉遵守）"],
    "complexity": {
        "time": "O(n)（本地训练目标）",
        "space": "迭代 O(1) 额外空间；递归需 O(n) 调用栈",
        "officialRequirement": "官方未强制复杂度要求（题面要求只交换节点、不修改节点值）。",
        "notes": "n 为节点数，输入链表不计入额外空间。哑节点 + 每次重连三根指针。最大测试为官方上限 n = 100。",
        "sourceUrl": "https://leetcode.com/problems/swap-nodes-in-pairs/",
    },
    "cpp": {
        "functionTemplate": "// ListNode is provided: int val; ListNode* next;\nclass Solution {\npublic:\n    ListNode* swapPairs(ListNode* head) {\n        return head; // Swap every two adjacent nodes.\n    }\n};",
        "acmIoSpec": "stdin：第一行节点数 n；第二行 n 个整数表示链表值。stdout：一行，交换后链表的值，以空格分隔；空链表输出空行。",
    },
    "io": {
        "method": "swapPairs",
        "params": [{"name": "head", "type": "ListNode"}],
        "returns": "ListNode",
    },
}


def solve(v):
    out = list(v)
    for i in range(0, len(out) - 1, 2):
        out[i], out[i + 1] = out[i + 1], out[i]
    return out


r = rng(24)
tests = [
    {"id": "t1", "sample": True, "input": [1, 2, 3, 4], "expected": [2, 1, 4, 3]},
    {"id": "t2", "sample": True, "input": [], "expected": []},
    {"id": "t3", "sample": True, "input": [1], "expected": [1]},
    {"id": "t4", "sample": True, "input": [1, 2, 3], "expected": [2, 1, 3]},
    {"id": "two", "sample": False, "input": [9, 0]},
    {"id": "odd-five", "sample": False, "input": [1, 2, 3, 4, 5]},
    {"id": "equal-values", "sample": False, "input": [3, 3, 3, 3, 3, 3]},
    {"id": "random", "sample": False, "input": [r.randint(0, 100) for _ in range(37)]},
    {"id": "max-size-100", "sample": False, "input": [r.randint(0, 100) for _ in range(100)]},
]
write(problem, tests, solve)
