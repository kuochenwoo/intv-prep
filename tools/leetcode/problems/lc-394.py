import re
import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-394",
    "title": "字符串解码",
    "difficulty": "medium",
    "tags": ["栈", "递归", "字符串"],
    "statement": "给定一个经过编码的字符串 s，编码规则为 k[encoded_string]：方括号内的子串重复恰好 k 次（k 为正整数），编码可以嵌套。输入保证合法：括号成对、没有多余空格、原始数据中不含数字（数字只作为重复次数出现）。返回解码后的字符串。",
    "en": {
        "title": "Decode String",
        "tags": [
            "Stack",
            "Recursion",
            "String"
        ],
        "statement": "Given an encoded string s, where the encoding rule is k[encoded_string]: the substring inside the brackets is repeated exactly k times (k is a positive integer), and encodings may be nested. The input is guaranteed to be valid: brackets are balanced, there are no extra spaces, and the original data contains no digits (digits appear only as repeat counts). Return the decoded string.",
        "constraints": [
            "1 ≤ s.length ≤ 30",
            "s consists of lowercase English letters, digits and square brackets '[]', and is guaranteed to be a valid encoding",
            "All integers in s are in the range [1, 300]",
            "The decoded result has length at most 10^5"
        ],
        "complexity": {
            "time": "O(L) (L is the length of the decoded result; each output character is built a constant number of times; implementations that concatenate at each nesting level may incur extra copying proportional to nesting depth)",
            "space": "O(L) (the result and partial strings on the stack); the recursive solution also uses an O(|s|) recursion stack",
            "officialRequirement": "No official complexity requirement.",
            "notes": "|s| is the encoded string length (≤ 30) and L is the output length (≤ 10^5). Use two stacks, one for repeat counts and one for the prefix built before entering the bracket; on ']', repeat the current string k times and append it to the prefix. The largest local test outputs exactly 10^5 characters."
        },
        "acmIoSpec": "stdin: one line with the encoded string s. stdout: one line, the decoded string."
    },
    "constraints": [
        "1 ≤ s.length ≤ 30",
        "s 由小写英文字母、数字和方括号 '[]' 组成，且保证是合法编码",
        "s 中所有整数取值范围为 [1, 300]",
        "解码结果长度不超过 10^5",
    ],
    "complexity": {
        "time": "O(L)（L 为解码结果长度，每个输出字符被构造常数次；嵌套拼接的实现可能多出与嵌套深度相关的拷贝）",
        "space": "O(L)（结果与栈中的部分串）；递归解法另含 O(|s|) 递归栈",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "|s| 为编码串长度（≤ 30），L 为输出长度（≤ 10^5）。用两个栈分别保存重复次数和进入括号前已构造的前缀；遇到 ']' 时把当前串重复 k 次接到前缀后。本地最大测试输出恰为 10^5 个字符。",
        "sourceUrl": "https://leetcode.com/problems/decode-string/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    string decodeString(string s) {\n        return \"\"; // Decoded string.\n    }\n};",
        "acmIoSpec": "stdin：一行编码字符串 s。stdout：一行，解码后的字符串。",
    },
    # The decoded string can be 10^5 characters (~98 KiB).
    "limits": {"timeLimitMs": 1000, "memoryLimitMb": 128, "outputLimitKb": 128},
    "io": {
        "method": "decodeString",
        "params": [{"name": "s", "type": "string"}],
        "returns": "string",
    },
}

INNER = re.compile(r"(\d+)\[([a-z]*)\]")


def solve(s):
    # Expand innermost brackets until none remain.
    while "[" in s:
        s = INNER.sub(lambda m: m.group(2) * int(m.group(1)), s)
    return s


tests = [
    {"id": "t1", "input": "3[a]2[bc]", "sample": True, "expected": "aaabcbc"},
    {"id": "t2", "input": "3[a2[c]]", "sample": True, "expected": "accaccacc"},
    {"id": "t3", "input": "2[abc]3[cd]ef", "sample": True, "expected": "abcabccdcdcdef"},
    {"id": "plain", "input": "leetcode", "sample": False},
    {"id": "single-char", "input": "x", "sample": False},
    {"id": "one-repeat", "input": "1[z]", "sample": False},
    {"id": "multi-digit", "input": "12[ab]", "sample": False},
    {"id": "three-digit", "input": "300[q]", "sample": False},
    {"id": "nested-mixed", "input": "3[z]2[2[y]pq4[2[jk]e1[f]]]ef", "sample": False},
    {"id": "prefix-suffix", "input": "ab2[c3[d]e]fg", "sample": False},
    {"id": "deep-nesting", "input": "2[2[2[2[2[2[2[a]]]]]]]", "sample": False},
    {"id": "adjacent-groups", "input": "2[a]3[b]4[c]5[d]6[e]", "sample": False},
    {"id": "worst-case-depth-and-size", "input": "10[10[10[10[ab]c]d]e]", "sample": False},
    # Output of exactly 10^5 characters, the official maximum.
    {"id": "max-size-output-100000", "input": "100[100[abcdefghij]]", "sample": False},
    {"id": "max-size-output-300x300", "input": "300[a300[b]]", "sample": False},
]
write(problem, tests, solve)
