import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-215",
    "title": "数组中的第K个最大元素",
    "difficulty": "medium",
    "tags": ["数组", "分治", "快速选择", "排序", "堆（优先队列）"],
    "statement": "给定整数数组 nums 和整数 k，返回数组排序后从大到小数第 k 个元素（按位置计，重复值分别计数，而不是第 k 个不同的值）。题目希望你不通过排序整个数组来求解。",
    "en": {'title': 'Kth Largest Element in an Array',
     'tags': ['Array', 'Divide and Conquer', 'Quickselect', 'Sorting', 'Heap (Priority Queue)'],
     'statement': 'Given an integer array nums and an integer k, return the k-th element counting from '
                  'the largest in sorted order (counted by position, so duplicates count separately; '
                  'it is not the k-th distinct value). The problem asks you to solve it without '
                  'sorting the whole array.',
     'constraints': ['1 ≤ k ≤ nums.length ≤ 100000; -10000 ≤ nums[i] ≤ 10000'],
     'complexity': {'time': 'Expected O(n) (local training target, randomized quickselect)',
                    'space': 'O(1) extra space (in-place quickselect, ignoring the constant needed for '
                             'recursion/iteration)',
                    'officialRequirement': 'The official statement asks whether you can solve it '
                                           'without sorting; the leetcode.cn Chinese statement '
                                           'explicitly requires an O(n)-time algorithm.',
                    'notes': 'n is the array length. Quickselect is expected O(n) and worst case '
                             'O(n²); it needs a randomized pivot and three-way partitioning to handle '
                             'many duplicates. A min-heap of size k is O(n log k); with only 20001 '
                             'possible values, counting achieves O(n + value range). Sorting in O(n '
                             'log n) also passes the local tests but does not match the intent. Local '
                             'tests include n = 100000 with many duplicates (values -5 to 5), plus '
                             'sorted input (n = 15000) and all-equal input (n = 30000), both reduced '
                             'to keep the problem bank small.'},
     'acmIoSpec': 'stdin: line 1 is n; line 2 has n integers nums; line 3 is the integer k. stdout: '
                  'one integer, the k-th largest element.'},
    "constraints": ["1 ≤ k ≤ nums.length ≤ 100000；-10000 ≤ nums[i] ≤ 10000"],
    "complexity": {
        "time": "期望 O(n)（本地训练目标，随机化快速选择）",
        "space": "O(1) 额外空间（原地快速选择，不计递归/迭代所需常数）",
        "officialRequirement": "官方题面提问能否不排序求解；leetcode.cn 中文题面明确要求设计 O(n) 时间的算法。",
        "notes": "n 为数组长度。快速选择期望 O(n)、最坏 O(n²)，需随机化枢轴并用三路划分应对大量重复值；大小为 k 的最小堆为 O(n log k)；值域只有 20001 个，计数法可做到 O(n + 值域)。排序 O(n log n) 也能通过本地测试，但不符合题意。本地包含 n = 100000 的大量重复值测试（取值 -5~5），以及为控制题库体积缩小的有序输入（n = 15000）与全等输入（n = 30000）测试。",
        "sourceUrl": "https://leetcode.com/problems/kth-largest-element-in-an-array/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    int findKthLargest(vector<int>& nums, int k) {\n        return 0; // The k-th largest element.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数 nums；第三行整数 k。stdout：一个整数，第 k 大的元素。",
    },
    "io": {
        "method": "findKthLargest",
        "params": [{"name": "nums", "type": "int[]"}, {"name": "k", "type": "int"}],
        "returns": "int",
    },
}


def solve(data):
    nums, k = data["nums"], data["k"]
    assert 1 <= k <= len(nums)
    return sorted(nums, reverse=True)[k - 1]


r = rng(215)
sorted_nums = sorted(r.randint(-10000, 10000) for _ in range(15000))
tests = [
    {"id": "t1", "input": {"nums": [3, 2, 1, 5, 6, 4], "k": 2}, "sample": True, "expected": 5},
    {"id": "t2", "input": {"nums": [3, 2, 3, 1, 2, 4, 5, 5, 6], "k": 4}, "sample": True, "expected": 4},
    {"id": "single", "input": {"nums": [-7], "k": 1}, "sample": False},
    {"id": "k-equals-n", "input": {"nums": [4, -1, 9, 0, 9], "k": 5}, "sample": False},
    {"id": "all-equal", "input": {"nums": [2] * 50, "k": 17}, "sample": False},
    {"id": "extremes", "input": {"nums": [-10000, 10000, -10000, 10000], "k": 3}, "sample": False},
    {"id": "random-1000", "input": {"nums": [r.randint(-10000, 10000) for _ in range(1000)], "k": 321}, "sample": False},
    # Heavy duplicates break two-way Lomuto partitioning; small values keep the file compact.
    {"id": "max-size-100000-dups", "input": {"nums": [r.randint(-5, 5) for _ in range(100000)], "k": 50000}, "sample": False},
    {"id": "worst-case-equal-30000", "input": {"nums": [0] * 30000, "k": 30000}, "sample": False},
    # Sorted input defeats a fixed first/last pivot.
    {"id": "worst-case-sorted-15000", "input": {"nums": sorted_nums, "k": 1}, "sample": False},
]
write(problem, tests, solve)
