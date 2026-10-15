import sys, pathlib, bisect
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-208",
    "title": "实现 Trie (前缀树)",
    "difficulty": "medium",
    "tags": ["设计", "字典树", "哈希表", "字符串"],
    "statement": "实现前缀树类 Trie：Trie() 初始化空树；insert(word) 插入字符串 word；search(word) 判断 word 是否曾被插入（完整匹配），返回 true/false；startsWith(prefix) 判断是否存在某个已插入的字符串以 prefix 为前缀，返回 true/false。",
    "constraints": [
        "1 ≤ word.length, prefix.length ≤ 2000",
        "word 与 prefix 仅由小写英文字母组成",
        "insert、search、startsWith 总调用次数 ≤ 3 × 10^4",
    ],
    "complexity": {
        "time": "每次 insert / search / startsWith 为 O(L)（本地训练目标）",
        "space": "O(S·Σ)：S 为所有插入字符串的总长度，Σ = 26（数组孩子）；用哈希表存孩子则为 O(S)",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "L 为本次操作的字符串长度。本地最大测试为约 10000 次短字符串调用，另有一组长度 2000 的长字符串测试（官方上限 3 × 10^4 次调用、长度 2000，为控制题库体积而缩小）。",
        "sourceUrl": "https://leetcode.com/problems/implement-trie-prefix-tree/",
    },
    "cpp": {
        "functionTemplate": "class Trie {\npublic:\n    Trie() {\n    }\n\n    void insert(string word) {\n    }\n\n    bool search(string word) {\n        return false;\n    }\n\n    bool startsWith(string prefix) {\n        return false;\n    }\n};",
        "acmIoSpec": "stdin：第一行操作数 m；随后 m 行，每行一个操作名（Trie / insert / search / startsWith）；接着一行 m，然后对每个操作先写一行参数个数 k（构造为 0，其余为 1），再写 k 行参数字符串。stdout：第一行 m，随后 m 行，每个操作一行结果：构造与 insert 输出 null，search 与 startsWith 输出 true 或 false。",
    },
    # Each call prints a line, so the output budget scales with the call count.
    "limits": {"timeLimitMs": 1000, "memoryLimitMb": 128, "outputLimitKb": 128},
    "io": {
        "method": "Trie",
        "params": [{"name": "ops", "type": "string[]"}, {"name": "args", "type": "string[][]"}],
        "returns": "string[]",
        "checker": "tokens",
        "harness": "lc-208.cpp",
    },
}


def solve(data):
    # Sorted word list + bisect: independent of the trie C++ solutions.
    words, out = [], []
    for op, a in zip(data["ops"], data["args"]):
        if op == "Trie":
            words = []
            out.append("null")
        elif op == "insert":
            i = bisect.bisect_left(words, a[0])
            if i == len(words) or words[i] != a[0]:
                words.insert(i, a[0])
            out.append("null")
        elif op == "search":
            i = bisect.bisect_left(words, a[0])
            out.append("true" if i < len(words) and words[i] == a[0] else "false")
        else:
            i = bisect.bisect_left(words, a[0])
            out.append("true" if i < len(words) and words[i].startswith(a[0]) else "false")
    return out


def calls(seed, m, alphabet, lo, hi, base_len=0):
    r = rng(seed)
    base = "".join(r.choice(alphabet) for _ in range(base_len))
    inserted = []
    ops, args = ["Trie"], [[]]

    def word():
        return base + "".join(r.choice(alphabet) for _ in range(r.randint(lo, hi)))

    for _ in range(m):
        x = r.random()
        if x < 0.4 or not inserted:
            w = word()
            inserted.append(w)
            ops.append("insert"); args.append([w])
        elif x < 0.7:
            # Half hits on inserted words, half fresh (or a prefix of one).
            w = r.choice(inserted)
            if r.random() < 0.5:
                w = w[: r.randint(1, len(w))] if r.random() < 0.5 else word()
            ops.append("search"); args.append([w])
        else:
            w = r.choice(inserted)
            p = w[: r.randint(1, len(w))]
            if r.random() < 0.4:
                p = p[:-1] + r.choice(alphabet)
            ops.append("startsWith"); args.append([p])
    return {"ops": ops, "args": args}


tests = [
    {"id": "t1", "sample": True,
     "input": {"ops": ["Trie", "insert", "search", "search", "startsWith", "insert", "search"],
               "args": [[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]},
     "expected": ["null", "null", "true", "false", "true", "null", "true"]},
    {"id": "single-letter", "sample": False,
     "input": {"ops": ["Trie", "search", "startsWith", "insert", "search", "startsWith", "search"],
               "args": [[], ["a"], ["a"], ["a"], ["a"], ["a"], ["b"]]}},
    {"id": "prefix-not-word", "sample": False,
     "input": {"ops": ["Trie", "insert", "insert", "search", "search", "startsWith", "startsWith", "search"],
               "args": [[], ["abcd"], ["abcd"], ["abc"], ["abcd"], ["abcde"], ["abcd"], ["abcde"]]}},
    {"id": "random-small", "sample": False, "input": calls(1, 300, "ab", 1, 6)},
    {"id": "long-words-2000", "sample": False, "input": calls(2, 60, "ab", 1, 40, base_len=1960)},
    # 3e4 calls with 2000-letter words would be ~60 MiB; 10000 short-word calls is representative.
    {"id": "worst-case-10000-calls", "sample": False, "input": calls(3, 10000, "abcdefghijklmnopqrstuvwxyz", 1, 10)},
]
write(problem, tests, solve)
