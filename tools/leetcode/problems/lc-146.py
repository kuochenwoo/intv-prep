import sys, pathlib
from collections import OrderedDict
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-146",
    "title": "LRU 缓存",
    "difficulty": "medium",
    "tags": ["设计", "哈希表", "链表", "双向链表"],
    "statement": "设计一个容量为 capacity 的 LRU（最近最少使用）缓存类 LRUCache：get(key) 返回键对应的值，不存在返回 -1；put(key, value) 写入或更新键值，若插入后超出容量，淘汰最久未被访问的键。get 和 put 都算一次访问。",
    "constraints": [
        "1 ≤ capacity ≤ 3000；0 ≤ key ≤ 10000；0 ≤ value ≤ 100000",
        "get 与 put 总调用次数 ≤ 200000",
    ],
    "complexity": {
        "time": "get 与 put 均为 O(1) 平均时间（官方要求）",
        "space": "O(capacity)",
        "officialRequirement": "官方要求 get 与 put 的平均时间复杂度均为 O(1)。",
        "notes": "哈希表定位节点，双向链表维护访问顺序。本地最大测试为 20000 次调用（官方上限 200000，为控制题库体积而缩小）；有限测试无法证明 O(1)。",
        "sourceUrl": "https://leetcode.com/problems/lru-cache/",
    },
    "cpp": {
        "functionTemplate": "class LRUCache {\npublic:\n    LRUCache(int capacity) {\n    }\n\n    int get(int key) {\n        return -1;\n    }\n\n    void put(int key, int value) {\n    }\n};",
        "acmIoSpec": "stdin：第一行操作数 m；随后 m 行，每行一个操作名（LRUCache / put / get）；接着一行 m，然后 m 行参数，每行先写参数个数 k 再写 k 个整数。stdout：第一行 m，随后 m 行，每个操作一行结果：构造与 put 输出 null，get 输出整数。",
    },
    # Each call prints a line, so the output budget scales with the call count.
    "limits": {"timeLimitMs": 1000, "memoryLimitMb": 128, "outputLimitKb": 256},
    "io": {
        "method": "LRUCache",
        "params": [{"name": "ops", "type": "string[]"}, {"name": "args", "type": "int[][]"}],
        "returns": "string[]",
        "checker": "tokens",
        "harness": "lc-146.cpp",
    },
}


def solve(data):
    cache, out = None, []
    for op, a in zip(data["ops"], data["args"]):
        if op == "LRUCache":
            cache, cap = OrderedDict(), a[0]
            out.append("null")
        elif op == "get":
            if a[0] in cache:
                cache.move_to_end(a[0])
                out.append(str(cache[a[0]]))
            else:
                out.append("-1")
        else:
            cache[a[0]] = a[1]
            cache.move_to_end(a[0])
            if len(cache) > cap:
                cache.popitem(last=False)
            out.append("null")
    return out


def calls(seed, cap, m, keys):
    r = rng(seed)
    ops, args = ["LRUCache"], [[cap]]
    for _ in range(m):
        if r.random() < 0.5:
            ops.append("put"); args.append([r.randint(0, keys), r.randint(0, 100000)])
        else:
            ops.append("get"); args.append([r.randint(0, keys)])
    return {"ops": ops, "args": args}


tests = [
    {"id": "t1", "sample": True,
     "input": {"ops": ["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"],
               "args": [[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]},
     "expected": ["null", "null", "null", "1", "null", "-1", "null", "-1", "3", "4"]},
    {"id": "capacity-one", "sample": False,
     "input": {"ops": ["LRUCache", "put", "get", "put", "get", "get"], "args": [[1], [2, 1], [2], [3, 2], [2], [3]]}},
    {"id": "update-refreshes", "sample": False,
     "input": {"ops": ["LRUCache", "put", "put", "put", "put", "get", "get"], "args": [[2], [1, 1], [2, 2], [1, 10], [3, 3], [2], [1]]}},
    {"id": "random-small", "sample": False, "input": calls(1, 3, 300, 8)},
    # 2e5 calls would make a ~4 MiB file; 20000 calls with heavy eviction is representative.
    {"id": "worst-case-20000-calls", "sample": False, "input": calls(2, 3000, 20000, 10000)},
]
write(problem, tests, solve)
