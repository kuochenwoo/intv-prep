import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-21",
    "title": "合并两个有序链表",
    "difficulty": "easy",
    "tags": ["递归", "链表"],
    "statement": "给定两条按非递减顺序排列的链表 list1 和 list2，把它们拼接成一条新的非递减链表并返回其头节点。新链表应由两条输入链表的节点直接拼接而成。\n\n节点定义：struct ListNode { int val; ListNode* next; };",
    "constraints": ["0 ≤ 每条链表节点数 ≤ 50；-100 ≤ 节点值 ≤ 100", "list1 与 list2 均按非递减顺序排列"],
    "complexity": {
        "time": "O(m + n)（本地训练目标）",
        "space": "迭代 O(1) 额外空间；递归需 O(m + n) 调用栈",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "m、n 为两条链表长度，输入链表不计入额外空间。哑节点 + 尾指针，每次接上较小的表头。最大测试为官方上限 m = n = 50。",
        "sourceUrl": "https://leetcode.com/problems/merge-two-sorted-lists/",
    },
    "en": {
        "title": "Merge Two Sorted Lists",
        "tags": [
            "Recursion",
            "Linked List"
        ],
        "statement": "Given two linked lists list1 and list2, each sorted in non-decreasing order, splice them into a single non-decreasing list and return its head. The new list should be formed by splicing together the nodes of the two input lists.\n\nNode definition: struct ListNode { int val; ListNode* next; };",
        "constraints": [
            "0 ≤ number of nodes in each list ≤ 50; -100 ≤ node value ≤ 100",
            "Both list1 and list2 are sorted in non-decreasing order"
        ],
        "complexity": {
            "time": "O(m + n) (local training target)",
            "space": "Iterative: O(1) extra space; recursive: O(m + n) call stack",
            "officialRequirement": "No official complexity requirement.",
            "notes": "m and n are the lengths of the two lists; the input lists do not count toward extra space. Use a dummy node + tail pointer, attaching the smaller head each time. The largest test uses the official limit m = n = 50."
        },
        "acmIoSpec": "stdin: line 1 is m; line 2 has m integers list1; line 3 is n; line 4 has n integers list2 (the line is empty when the length is 0). stdout: one line with the values of the merged list, separated by spaces; output an empty line if the result is empty."
    },
    "cpp": {
        "functionTemplate": "// ListNode is provided: int val; ListNode* next;\nclass Solution {\npublic:\n    ListNode* mergeTwoLists(ListNode* list1, ListNode* list2) {\n        return nullptr; // Head of the merged sorted list.\n    }\n};",
        "acmIoSpec": "stdin：第一行 m；第二行 m 个整数 list1；第三行 n；第四行 n 个整数 list2（长度为 0 时对应行为空）。stdout：一行，合并后链表的值，以空格分隔；结果为空时输出空行。",
    },
    "io": {
        "method": "mergeTwoLists",
        "params": [{"name": "list1", "type": "ListNode"}, {"name": "list2", "type": "ListNode"}],
        "returns": "ListNode",
    },
}


def solve(d):
    a, b, out, i, j = d["list1"], d["list2"], [], 0, 0
    while i < len(a) or j < len(b):
        if j == len(b) or (i < len(a) and a[i] <= b[j]):
            out.append(a[i]); i += 1
        else:
            out.append(b[j]); j += 1
    return out


r = rng(21)
srt = lambda n, lo=-100, hi=100: sorted(r.randint(lo, hi) for _ in range(n))
tests = [
    {"id": "t1", "sample": True, "input": {"list1": [1, 2, 4], "list2": [1, 3, 4]}, "expected": [1, 1, 2, 3, 4, 4]},
    {"id": "t2", "sample": True, "input": {"list1": [], "list2": []}, "expected": []},
    {"id": "t3", "sample": True, "input": {"list1": [], "list2": [0]}, "expected": [0]},
    {"id": "second-empty", "sample": False, "input": {"list1": [-3, 5], "list2": []}},
    {"id": "disjoint-ranges", "sample": False, "input": {"list1": [5, 6, 7], "list2": [1, 2, 3]}},
    {"id": "all-equal", "sample": False, "input": {"list1": [2] * 5, "list2": [2] * 4}},
    {"id": "negatives", "sample": False, "input": {"list1": [-100, -50, 0], "list2": [-99, -1, 100]}},
    {"id": "random", "sample": False, "input": {"list1": srt(17, -5, 5), "list2": srt(23, -5, 5)}},
    {"id": "max-size-50", "sample": False, "input": {"list1": srt(50), "list2": srt(50)}},
]
write(problem, tests, solve)
