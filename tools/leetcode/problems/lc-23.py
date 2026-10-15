import sys, pathlib, heapq
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-23",
    "title": "合并 K 个升序链表",
    "difficulty": "hard",
    "tags": ["链表", "分治", "堆（优先队列）", "归并排序"],
    "statement": "给定 k 条链表组成的数组 lists，每条链表都已按升序排列。把它们合并成一条升序链表并返回其头节点。\n\n节点定义：struct ListNode { int val; ListNode* next; };",
    "constraints": [
        "0 ≤ k ≤ 10000；0 ≤ lists[i] 的长度 ≤ 500；-10000 ≤ 节点值 ≤ 10000",
        "每条链表按升序排列；所有链表总节点数 N ≤ 10000",
    ],
    "complexity": {
        "time": "O(N log k)（本地训练目标）",
        "space": "最小堆 O(k)；两两分治合并 O(log k) 递归栈（迭代实现 O(1)）",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "N 为总节点数，k 为链表条数，输入链表不计入额外空间。逐条顺序合并为 O(N·k)，在 k = 10000 条单节点链表时约 5×10^7 次指针操作，本地也可能通过，但不是目标解法。最大测试为官方上限：k = 10000 条单节点链表（N = 10000），另有 20 条 × 500 节点。",
        "sourceUrl": "https://leetcode.com/problems/merge-k-sorted-lists/",
    },
    "en": {
        "title": "Merge k Sorted Lists",
        "tags": [
            "Linked List",
            "Divide and Conquer",
            "Heap (Priority Queue)",
            "Merge Sort"
        ],
        "statement": "Given an array lists of k linked lists, each sorted in ascending order, merge them into one ascending list and return its head.\n\nNode definition: struct ListNode { int val; ListNode* next; };",
        "constraints": [
            "0 ≤ k ≤ 10000; 0 ≤ length of lists[i] ≤ 500; -10000 ≤ node value ≤ 10000",
            "Each list is sorted in ascending order; total number of nodes across all lists N ≤ 10000"
        ],
        "complexity": {
            "time": "O(N log k) (local training target)",
            "space": "Min-heap: O(k); pairwise divide-and-conquer merging: O(log k) recursion stack (O(1) if iterative)",
            "officialRequirement": "No official complexity requirement.",
            "notes": "N is the total number of nodes and k the number of lists; the input lists do not count toward extra space. Merging lists one at a time is O(N·k), about 5×10^7 pointer operations with k = 10000 single-node lists; it may pass locally, but it is not the target solution. The largest tests use the official limits: k = 10000 single-node lists (N = 10000), plus 20 lists × 500 nodes."
        },
        "acmIoSpec": "stdin: line 1 is the number of lists k; then k lines, each starting with that list's length len followed by len ascending integers. stdout: one line with the values of the merged list, separated by spaces; output an empty line if the result is empty."
    },
    "cpp": {
        "functionTemplate": "// ListNode is provided: int val; ListNode* next;\nclass Solution {\npublic:\n    ListNode* mergeKLists(vector<ListNode*>& lists) {\n        return nullptr; // Head of the merged sorted list.\n    }\n};",
        "acmIoSpec": "stdin：第一行链表条数 k；随后 k 行，每行先写该链表长度 len，再写 len 个升序整数。stdout：一行，合并后链表的值，以空格分隔；结果为空时输出空行。",
    },
    # Up to 10000 printed values exceed the default 64 KiB output budget.
    "limits": {"timeLimitMs": 1000, "memoryLimitMb": 128, "outputLimitKb": 256},
    "io": {
        "method": "mergeKLists",
        "params": [{"name": "lists", "type": "int[][]"}],
        "returns": "ListNode",
        "harness": "lc-23.cpp",
    },
}


def solve(lists):
    return list(heapq.merge(*lists))


r = rng(23)
srt = lambda n, lo=-10000, hi=10000: sorted(r.randint(lo, hi) for _ in range(n))
tests = [
    {"id": "t1", "sample": True, "input": [[1, 4, 5], [1, 3, 4], [2, 6]], "expected": [1, 1, 2, 3, 4, 4, 5, 6]},
    {"id": "t2", "sample": True, "input": [], "expected": []},
    {"id": "t3", "sample": True, "input": [[]], "expected": []},
    {"id": "some-empty", "sample": False, "input": [[], [-2, 0], [], [-3], []]},
    {"id": "single-list", "sample": False, "input": [[1, 2, 3]]},
    {"id": "all-equal", "sample": False, "input": [[5, 5], [5], [5, 5, 5]]},
    {"id": "random", "sample": False, "input": [srt(r.randint(0, 30), -50, 50) for _ in range(40)]},
    {"id": "max-size-10000-singletons", "sample": False, "input": [[r.randint(-10000, 10000)] for _ in range(10000)]},
    {"id": "max-size-20x500", "sample": False, "input": [srt(500) for _ in range(20)]},
]
write(problem, tests, solve)
