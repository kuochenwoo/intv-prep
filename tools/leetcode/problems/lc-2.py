import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-2",
    "title": "两数相加",
    "difficulty": "medium",
    "tags": ["递归", "链表", "数学"],
    "statement": "两条非空链表各表示一个非负整数，数字按逆序存放（表头是个位），每个节点存一位数字。求两数之和，并以同样的逆序链表形式返回。除数字 0 本身外，两个数都不含前导零。\n\n节点定义：struct ListNode { int val; ListNode* next; };",
    "constraints": ["1 ≤ 每条链表节点数 ≤ 100；0 ≤ 节点值 ≤ 9", "表示的数不含前导零（数字 0 本身除外）"],
    "complexity": {
        "time": "O(max(m, n))（本地训练目标）",
        "space": "O(1) 额外空间，不计输出链表（长度至多 max(m, n) + 1）",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "m、n 为两条链表长度。逐位相加并维护进位，最后若仍有进位需追加一个节点。数字可达 100 位，不能转成内置整数类型再相加。最大测试为官方上限 100 位。",
        "sourceUrl": "https://leetcode.com/problems/add-two-numbers/",
    },
    "en": {
        "title": "Add Two Numbers",
        "tags": [
            "Recursion",
            "Linked List",
            "Math"
        ],
        "statement": "Two non-empty linked lists each represent a non-negative integer, with digits stored in reverse order (the head holds the ones digit) and one digit per node. Compute the sum of the two numbers and return it as a linked list in the same reversed form. Apart from the number 0 itself, neither number has leading zeros.\n\nNode definition: struct ListNode { int val; ListNode* next; };",
        "constraints": [
            "1 ≤ number of nodes in each list ≤ 100; 0 ≤ node value ≤ 9",
            "The represented numbers have no leading zeros (except the number 0 itself)"
        ],
        "complexity": {
            "time": "O(max(m, n)) (local training target)",
            "space": "O(1) extra space, excluding the output list (length at most max(m, n) + 1)",
            "officialRequirement": "No official complexity requirement.",
            "notes": "m and n are the lengths of the two lists. Add digit by digit while tracking the carry; if a carry remains at the end, append one more node. Numbers can have up to 100 digits, so they cannot be converted to a built-in integer type and added. The largest test uses the official limit of 100 digits."
        },
        "acmIoSpec": "stdin: line 1 is m; line 2 has m digits of l1 (ones digit first); line 3 is n; line 4 has n digits of l2. stdout: one line with the digits of the sum (ones digit first), separated by spaces."
    },
    "cpp": {
        "functionTemplate": "// ListNode is provided: int val; ListNode* next;\nclass Solution {\npublic:\n    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {\n        return nullptr; // Sum digits, least significant first.\n    }\n};",
        "acmIoSpec": "stdin：第一行 m；第二行 m 个数字 l1（个位在前）；第三行 n；第四行 n 个数字 l2。stdout：一行，和的各位数字（个位在前），以空格分隔。",
    },
    "io": {
        "method": "addTwoNumbers",
        "params": [{"name": "l1", "type": "ListNode"}, {"name": "l2", "type": "ListNode"}],
        "returns": "ListNode",
    },
}


def solve(d):
    # Python big integers: independent of digit-by-digit carry.
    num = lambda ds: int("".join(map(str, reversed(ds))))
    return [int(c) for c in reversed(str(num(d["l1"]) + num(d["l2"])))]


r = rng(2)


def number(n):
    ds = [r.randint(0, 9) for _ in range(n - 1)] + [r.randint(1, 9)]
    return ds if n > 1 else [r.randint(0, 9)]


tests = [
    {"id": "t1", "sample": True, "input": {"l1": [2, 4, 3], "l2": [5, 6, 4]}, "expected": [7, 0, 8]},
    {"id": "t2", "sample": True, "input": {"l1": [0], "l2": [0]}, "expected": [0]},
    {"id": "t3", "sample": True, "input": {"l1": [9, 9, 9, 9, 9, 9, 9], "l2": [9, 9, 9, 9]}, "expected": [8, 9, 9, 9, 0, 0, 0, 1]},
    {"id": "zero-plus", "sample": False, "input": {"l1": [0], "l2": [3, 2, 1]}},
    {"id": "final-carry", "sample": False, "input": {"l1": [5], "l2": [5]}},
    {"id": "carry-chain", "sample": False, "input": {"l1": [1], "l2": [9] * 30}},
    {"id": "random", "sample": False, "input": {"l1": number(20), "l2": number(13)}},
    {"id": "max-size-100", "sample": False, "input": {"l1": number(100), "l2": number(100)}},
    {"id": "max-size-100-all-nines", "sample": False, "input": {"l1": [9] * 100, "l2": [9] * 100}},
]
write(problem, tests, solve)
