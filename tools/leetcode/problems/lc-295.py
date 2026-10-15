import sys, pathlib, bisect
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-295",
    "title": "数据流的中位数",
    "difficulty": "hard",
    "tags": ["设计", "双指针", "数据流", "排序", "堆（优先队列）"],
    "statement": "设计类 MedianFinder 维护一个不断加入整数的数据流：MedianFinder() 初始化；addNum(num) 加入一个整数；findMedian() 返回目前所有元素的中位数（元素个数为奇数时取正中间的数，为偶数时取中间两个数的平均值，返回浮点数）。",
    "en": {
        "title": "Find Median from Data Stream",
        "tags": [
            "Design",
            "Two Pointers",
            "Data Stream",
            "Sorting",
            "Heap (Priority Queue)"
        ],
        "statement": "Design a class MedianFinder that maintains a stream of integers being added: MedianFinder() initializes the object; addNum(num) adds an integer; findMedian() returns the median of all elements so far (the middle element when the count is odd, the average of the two middle elements when it is even, returned as a floating-point number).",
        "constraints": [
            "-100000 ≤ num ≤ 100000",
            "There is at least one element in the data structure before findMedian is called",
            "Total calls to addNum and findMedian ≤ 50000"
        ],
        "complexity": {
            "time": "addNum O(log n), findMedian O(1) (local training target)",
            "space": "O(n) extra space",
            "officialRequirement": "No official complexity requirement; the follow-up asks how to optimize if all numbers are in [0, 100], or if 99% of numbers are in [0, 100].",
            "notes": "n is the number of elements added so far. A max-heap holds the smaller half and a min-heap holds the larger half, keeping their sizes within 1 of each other. With a bounded value range, a counting array or Fenwick tree can also find elements by rank. Results are compared with a tolerance of 1e-5. The largest local test has 12000 calls (official limit 50000, reduced to keep the problem bank small)."
        },
        "acmIoSpec": "stdin: line 1 is the number of operations m; then m lines, each with one operation name (MedianFinder / addNum / findMedian); then a line with m, followed by m lines of arguments, each starting with the argument count k followed by k integers (1 for addNum, 0 for the others). stdout: line 1 is m, followed by m lines, one result per operation: the constructor and addNum print null, findMedian prints the median (e.g. 2, 2.5 or -1.5; an error of at most 1e-5 from the answer is accepted)."
    },
    "constraints": [
        "-100000 ≤ num ≤ 100000",
        "调用 findMedian 之前数据结构中至少有一个元素",
        "addNum 与 findMedian 总调用次数 ≤ 50000",
    ],
    "complexity": {
        "time": "addNum O(log n)，findMedian O(1)（本地训练目标）",
        "space": "O(n) 额外空间",
        "officialRequirement": "官方未强制复杂度要求；follow-up 询问：若所有数都在 [0, 100] 内，或 99% 的数在 [0, 100] 内，如何优化。",
        "notes": "n 为已加入的元素个数。大顶堆存较小一半、小顶堆存较大一半，保持两堆大小差不超过 1。值域有界时也可用计数数组或树状数组按名次查找。结果按 1e-5 的误差比较。本地最大测试为 12000 次调用（官方上限 50000，为控制题库体积而缩小）。",
        "sourceUrl": "https://leetcode.com/problems/find-median-from-data-stream/",
    },
    "cpp": {
        "functionTemplate": "class MedianFinder {\npublic:\n    MedianFinder() {\n    }\n\n    void addNum(int num) {\n    }\n\n    double findMedian() {\n        return 0.0;\n    }\n};",
        "acmIoSpec": "stdin：第一行操作数 m；随后 m 行，每行一个操作名（MedianFinder / addNum / findMedian）；接着一行 m，然后 m 行参数，每行先写参数个数 k 再写 k 个整数（addNum 为 1 个，其余为 0 个）。stdout：第一行 m，随后 m 行，每个操作一行结果：构造与 addNum 输出 null，findMedian 输出中位数（如 2、2.5 或 -1.5，与答案误差不超过 1e-5 即可）。",
    },
    # Each call prints a line, so the output budget scales with the call count.
    "limits": {"timeLimitMs": 1000, "memoryLimitMb": 128, "outputLimitKb": 256},
    "io": {
        "method": "MedianFinder",
        "params": [{"name": "ops", "type": "string[]"}, {"name": "args", "type": "int[][]"}],
        "returns": "string[]",
        "checker": "tokens",
        "harness": "lc-295.cpp",
    },
}


def solve(data):
    # Sorted list via bisect.insort: simple and independent of the heap/Fenwick C++ solutions.
    xs, out = None, []
    for op, a in zip(data["ops"], data["args"]):
        if op == "MedianFinder":
            xs = []
            out.append("null")
        elif op == "addNum":
            bisect.insort(xs, a[0]); out.append("null")
        else:
            assert xs
            n = len(xs)
            med = float(xs[n // 2]) if n % 2 else (xs[n // 2 - 1] + xs[n // 2]) / 2
            out.append(repr(med))
    return out


def calls(seed, m, lo, hi, add_bias=0.5, values=None):
    r = rng(seed)
    ops, args, size = ["MedianFinder"], [[]], 0
    for i in range(m):
        if size == 0 or r.random() < add_bias:
            v = values(i, r) if values else r.randint(lo, hi)
            ops.append("addNum"); args.append([v]); size += 1
        else:
            ops.append("findMedian"); args.append([])
    return {"ops": ops, "args": args}


def mostly_small(i, r):
    # Follow-up shape: 99% of values in [0, 100], outliers anywhere in range.
    return r.randint(0, 100) if r.random() < 0.99 else r.choice([-100000, 100000, r.randint(-100000, 100000)])


tests = [
    {"id": "t1", "sample": True,
     "input": {"ops": ["MedianFinder", "addNum", "addNum", "findMedian", "addNum", "findMedian"],
               "args": [[], [1], [2], [], [3], []]},
     "expected": ["null", "null", "null", "1.5", "null", "2.0"]},
    {"id": "single", "sample": False,
     "input": {"ops": ["MedianFinder", "addNum", "findMedian"], "args": [[], [-100000], []]}},
    {"id": "negative-half", "sample": False,
     "input": {"ops": ["MedianFinder", "addNum", "addNum", "findMedian", "addNum", "addNum", "findMedian"],
               "args": [[], [-1], [-2], [], [100000], [-100000], []]}},
    {"id": "all-equal", "sample": False, "input": calls(1, 200, 7, 7)},
    {"id": "random-small", "sample": False, "input": calls(2, 400, -10, 10)},
    {"id": "range-0-100", "sample": False, "input": calls(3, 1500, 0, 100)},
    {"id": "mostly-0-100", "sample": False, "input": calls(4, 1500, 0, 0, values=mostly_small)},
    # Ascending inserts with a query after each: stresses re-sorting on every findMedian.
    {"id": "worst-case-ascending-5000", "sample": False,
     "input": calls(5, 5000, 0, 0, add_bias=0.5, values=lambda i, r: -100000 + 25 * i)},
    {"id": "worst-case-12000-calls", "sample": False, "input": calls(6, 12000, -100000, 100000, 0.5)},
]
write(problem, tests, solve)
