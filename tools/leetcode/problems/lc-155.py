import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-155",
    "title": "最小栈",
    "difficulty": "medium",
    "tags": ["栈", "设计"],
    "statement": "设计一个栈类 MinStack，除常规操作外还能随时取得栈中的最小值：MinStack() 创建空栈；push(val) 压入 val；pop() 弹出栈顶；top() 返回栈顶元素；getMin() 返回栈中当前最小元素。pop、top、getMin 只会在栈非空时调用。",
    "constraints": [
        "-2³¹ ≤ val ≤ 2³¹ - 1",
        "pop、top、getMin 总在非空栈上调用",
        "push、pop、top、getMin 总调用次数 ≤ 30000",
    ],
    "complexity": {
        "time": "每个操作 O(1)（官方要求）",
        "space": "O(n) 额外空间",
        "officialRequirement": "官方要求每个函数的时间复杂度均为 O(1)。",
        "notes": "n 为栈中元素个数。辅助栈同步记录每层的最小值（相等值也要压入），或每个元素携带压入时的最小值。本地最大测试为 13000 次调用（官方上限 30000，为控制题库体积而缩小）；有限测试无法证明 O(1)。",
        "sourceUrl": "https://leetcode.com/problems/min-stack/",
    },
    "cpp": {
        "functionTemplate": "class MinStack {\npublic:\n    MinStack() {\n    }\n\n    void push(int val) {\n    }\n\n    void pop() {\n    }\n\n    int top() {\n        return 0;\n    }\n\n    int getMin() {\n        return 0;\n    }\n};",
        "acmIoSpec": "stdin：第一行操作数 m；随后 m 行，每行一个操作名（MinStack / push / pop / top / getMin）；接着一行 m，然后 m 行参数，每行先写参数个数 k 再写 k 个整数（push 为 1 个，其余为 0 个）。stdout：第一行 m，随后 m 行，每个操作一行结果：构造、push、pop 输出 null，top 与 getMin 输出整数。",
    },
    # Each call prints a line, so the output budget scales with the call count.
    "limits": {"timeLimitMs": 1000, "memoryLimitMb": 128, "outputLimitKb": 256},
    "io": {
        "method": "MinStack",
        "params": [{"name": "ops", "type": "string[]"}, {"name": "args", "type": "int[][]"}],
        "returns": "string[]",
        "checker": "tokens",
        "harness": "lc-155.cpp",
    },
}


def solve(data):
    # Plain list with a linear min scan: obviously correct, independent of the C++ tricks.
    stack, out = None, []
    for op, a in zip(data["ops"], data["args"]):
        if op == "MinStack":
            stack = []
            out.append("null")
        elif op == "push":
            stack.append(a[0]); out.append("null")
        elif op == "pop":
            assert stack
            stack.pop(); out.append("null")
        elif op == "top":
            out.append(str(stack[-1]))
        else:
            out.append(str(min(stack)))
    return out


LO, HI = -2**31, 2**31 - 1


def calls(seed, m, lo, hi, push_bias=0.5):
    r = rng(seed)
    ops, args, size = ["MinStack"], [[]], 0
    for _ in range(m):
        x = r.random()
        if size == 0 or x < push_bias:
            ops.append("push"); args.append([r.randint(lo, hi)]); size += 1
        elif x < push_bias + (1 - push_bias) / 3:
            ops.append("pop"); args.append([]); size -= 1
        elif x < push_bias + 2 * (1 - push_bias) / 3:
            ops.append("top"); args.append([])
        else:
            ops.append("getMin"); args.append([])
    return {"ops": ops, "args": args}


def descending_then_unwind(m):
    # Push strictly decreasing values, then alternate getMin/pop: the minimum changes on every pop.
    half = m // 3
    ops, args = ["MinStack"], [[]]
    for i in range(half):
        ops.append("push"); args.append([1000000 - 7 * i])
    for _ in range(half):
        ops += ["getMin", "pop"]; args += [[], []]
    return {"ops": ops, "args": args}


tests = [
    {"id": "t1", "sample": True,
     "input": {"ops": ["MinStack", "push", "push", "push", "getMin", "pop", "top", "getMin"],
               "args": [[], [-2], [0], [-3], [], [], [], []]},
     "expected": ["null", "null", "null", "null", "-3", "null", "0", "-2"]},
    {"id": "duplicate-min", "sample": False,
     "input": {"ops": ["MinStack", "push", "push", "push", "getMin", "pop", "getMin", "pop", "getMin"],
               "args": [[], [0], [1], [0], [], [], [], [], []]}},
    {"id": "int-extremes", "sample": False,
     "input": {"ops": ["MinStack", "push", "push", "push", "getMin", "top", "pop", "getMin", "pop", "getMin", "top"],
               "args": [[], [HI], [LO], [HI], [], [], [], [], [], [], []]}},
    {"id": "empty-then-reuse", "sample": False,
     "input": {"ops": ["MinStack", "push", "pop", "push", "getMin", "top"], "args": [[], [5], [], [7], [], []]}},
    {"id": "random-small", "sample": False, "input": calls(1, 300, -5, 5)},
    {"id": "random-full-range", "sample": False, "input": calls(2, 1000, LO, HI)},
    # 30000 full-range calls would make a ~1.2 MiB file; smaller runs keep the same shapes.
    {"id": "worst-case-13000-calls", "sample": False, "input": calls(3, 13000, LO, HI, 0.55)},
    {"id": "worst-case-6000-unwind", "sample": False, "input": descending_then_unwind(6000)},
]
write(problem, tests, solve)
