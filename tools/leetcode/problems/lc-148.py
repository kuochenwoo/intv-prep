import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-148",
    "title": "排序链表",
    "difficulty": "medium",
    "tags": ["链表", "双指针", "分治", "排序", "归并排序"],
    "statement": "给定链表的头节点 head，把链表按节点值升序排列并返回排序后的头节点。\n\n节点定义：struct ListNode { int val; ListNode* next; };",
    "en": {'title': 'Sort List',
     'tags': ['Linked List', 'Two Pointers', 'Divide and Conquer', 'Sorting', 'Merge Sort'],
     'statement': 'Given the head of a linked list, sort the list in ascending order of node values '
                  'and return the head of the sorted list.\n'
                  '\n'
                  'Node definition: struct ListNode { int val; ListNode* next; };',
     'constraints': ['0 ≤ number of nodes ≤ 50000; -100000 ≤ node value ≤ 100000'],
     'complexity': {'time': 'O(n log n) (local training target)',
                    'space': 'Bottom-up merge sort uses O(1) extra space; top-down recursive merge '
                             'sort needs an O(log n) call stack',
                    'officialRequirement': 'Official follow-up asks for O(n log n) time and O(1) extra '
                                           'memory (i.e., constant space).',
                    'notes': 'n is the number of nodes; the input list does not count toward extra '
                             'space. Bottom-up merge: segment lengths go 1, 2, 4, ...; each pass cuts '
                             'off two adjacent segments, merges them, and reattaches the result. The '
                             'largest test uses the official limit n = 50000, but to keep the problem '
                             'bank small its node values are in [-999, 999] (smaller random tests '
                             'cover the full value range); copying into an array and using std::sort '
                             'also passes the local tests but does not meet O(1) space.'},
     'acmIoSpec': 'stdin: line 1 is the node count n; line 2 has n integers, the list values. stdout: '
                  'one line with the values in ascending order, space-separated; an empty list prints '
                  'an empty line.'},
    "constraints": ["0 ≤ 节点数 ≤ 50000；-100000 ≤ 节点值 ≤ 100000"],
    "complexity": {
        "time": "O(n log n)（本地训练目标）",
        "space": "自底向上归并 O(1) 额外空间；自顶向下递归归并需 O(log n) 调用栈",
        "officialRequirement": "官方 follow-up 要求 O(n log n) 时间且 O(1) 额外内存（即常数空间）。",
        "notes": "n 为节点数，输入链表不计入额外空间。自底向上归并：子段长度依次取 1、2、4……，每轮切出相邻两段合并后接回。最大测试为官方上限 n = 50000，但为控制题库体积，节点值取 [-999, 999]（另有全值域的较小随机测试）；拷贝到数组后 std::sort 也能通过本地测试，但不满足 O(1) 空间。",
        "sourceUrl": "https://leetcode.com/problems/sort-list/",
    },
    "cpp": {
        "functionTemplate": "// ListNode is provided: int val; ListNode* next;\nclass Solution {\npublic:\n    ListNode* sortList(ListNode* head) {\n        return head; // Sort ascending.\n    }\n};",
        "acmIoSpec": "stdin：第一行节点数 n；第二行 n 个整数表示链表值。stdout：一行，升序排列后的值，以空格分隔；空链表输出空行。",
    },
    # 50000 printed values exceed the default 64 KiB output budget.
    "limits": {"timeLimitMs": 1000, "memoryLimitMb": 128, "outputLimitKb": 512},
    "io": {
        "method": "sortList",
        "params": [{"name": "head", "type": "ListNode"}],
        "returns": "ListNode",
    },
}


def solve(v):
    return sorted(v)


r = rng(148)
tests = [
    {"id": "t1", "sample": True, "input": [4, 2, 1, 3], "expected": [1, 2, 3, 4]},
    {"id": "t2", "sample": True, "input": [-1, 5, 3, 4, 0], "expected": [-1, 0, 3, 4, 5]},
    {"id": "t3", "sample": True, "input": [], "expected": []},
    {"id": "single", "sample": False, "input": [100000]},
    {"id": "two-desc", "sample": False, "input": [2, 1]},
    {"id": "duplicates", "sample": False, "input": [3, 1, 3, 1, 2, 2, 3]},
    {"id": "extremes", "sample": False, "input": [100000, -100000, 0, 100000, -100000]},
    {"id": "random-full-range", "sample": False, "input": [r.randint(-100000, 100000) for _ in range(2000)]},
    {"id": "max-size-50000", "sample": False, "input": [r.randint(-999, 999) for _ in range(50000)]},
    {"id": "descending-2000", "sample": False, "input": list(range(999, -1001, -1))},
]
write(problem, tests, solve)
