import sys, pathlib, string
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-763",
    "title": "划分字母区间",
    "difficulty": "medium",
    "tags": ["贪心", "哈希表", "双指针", "字符串"],
    "statement": "给定由小写字母组成的字符串 s，把它切成尽可能多的连续片段，使得每个字母只出现在其中一个片段里。片段按顺序拼接后必须仍是 s。按从左到右的顺序返回每个片段的长度。",
    "constraints": ["1 ≤ s.length ≤ 500；s 仅由小写英文字母组成"],
    "complexity": {
        "time": "O(n)（本地训练目标）",
        "space": "O(Σ) 额外空间，Σ = 26（不计输出）",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为字符串长度。先记录每个字母最后出现的位置，扫描时维护当前片段需要延伸到的最远下标，到达时切分。",
        "sourceUrl": "https://leetcode.com/problems/partition-labels/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    vector<int> partitionLabels(string s) {\n        return {}; // Sizes of the parts, left to right.\n    }\n};",
        "acmIoSpec": "stdin：一行字符串 s。stdout：一行，按顺序输出各片段长度，以空格分隔。",
    },
    "io": {
        "method": "partitionLabels",
        "params": [{"name": "s", "type": "string"}],
        "returns": "int[]",
    },
}


def solve(s):
    # Cut after i exactly when the prefix and suffix share no letter (quadratic, independent).
    sizes, start = [], 0
    for i in range(len(s)):
        if not set(s[: i + 1]) & set(s[i + 1:]):
            sizes.append(i + 1 - start)
            start = i + 1
    return sizes


r = rng(763)
tests = [
    {"id": "t1", "input": "ababcbacadefegdehijhklij", "sample": True, "expected": [9, 7, 8]},
    {"id": "t2", "input": "eccbbbbdec", "sample": True, "expected": [10]},
    {"id": "single", "input": "z", "sample": False},
    {"id": "all-distinct", "input": string.ascii_lowercase, "sample": False},
    {"id": "all-same", "input": "q" * 30, "sample": False},
    {"id": "random-small", "input": "".join(r.choice("abcdef") for _ in range(30)), "sample": False},
    {"id": "blocks", "input": "".join(c * r.randint(1, 15) for c in "abcdefghijklmnopqrstuvwxy"), "sample": False},
    {"id": "max-size-500", "input": "".join(r.choice(string.ascii_lowercase) for _ in range(500)), "sample": False},
    {"id": "max-size-500-blocks", "input": "".join(c * 20 for c in "abcdefghijklmnopqrstuvwxy"), "sample": False},
]
write(problem, tests, solve)
