import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-20",
    "title": "有效的括号",
    "difficulty": "easy",
    "tags": ["栈", "字符串"],
    "statement": "给定一个只由 '(' ')' '[' ']' '{' '}' 组成的字符串 s，判断它是否有效：每个左括号都必须由同类型的右括号闭合，闭合顺序必须正确（后打开的先闭合），且每个右括号都有对应的左括号。",
    "constraints": [
        "1 ≤ s.length ≤ 10^4",
        "s 只包含字符 '()[]{}'",
    ],
    "complexity": {
        "time": "O(n)",
        "space": "O(n) 额外空间（栈）",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为字符串长度。遇到左括号入栈，遇到右括号检查栈顶是否为匹配的左括号；最后栈必须为空。长度为奇数可直接判定无效。",
        "sourceUrl": "https://leetcode.com/problems/valid-parentheses/",
    },
    "en": {
        "title": "Valid Parentheses",
        "tags": [
            "Stack",
            "String"
        ],
        "statement": "Given a string s consisting only of '(' ')' '[' ']' '{' '}', determine whether it is valid: every opening bracket must be closed by a closing bracket of the same type, brackets must be closed in the correct order (the most recently opened is closed first), and every closing bracket must have a matching opening bracket.",
        "constraints": [
            "1 ≤ s.length ≤ 10^4",
            "s contains only the characters '()[]{}'"
        ],
        "complexity": {
            "time": "O(n)",
            "space": "O(n) extra space (stack)",
            "officialRequirement": "No official complexity requirement.",
            "notes": "n is the string length. Push opening brackets onto the stack; on a closing bracket, check that the top of the stack is the matching opening bracket; at the end the stack must be empty. An odd length is immediately invalid."
        },
        "acmIoSpec": "stdin: one line, the string s. stdout: true or false."
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    bool isValid(string s) {\n        return false; // Whether the brackets are balanced.\n    }\n};",
        "acmIoSpec": "stdin：一行字符串 s。stdout：true 或 false。",
    },
    "io": {
        "method": "isValid",
        "params": [{"name": "s", "type": "string"}],
        "returns": "bool",
    },
}


def solve(s):
    # Repeatedly delete innermost adjacent pairs (independent of a stack).
    prev = None
    while prev != s:
        prev = s
        s = s.replace("()", "").replace("[]", "").replace("{}", "")
    return s == ""


def valid(r, n):
    # Random valid string of length n (n even) via a random walk with a stack.
    out, st = [], []
    pairs = {"(": ")", "[": "]", "{": "}"}
    while len(out) < n:
        remaining = n - len(out)
        if st and (remaining == len(st) or r.random() < 0.5):
            out.append(pairs[st.pop()])
        else:
            c = r.choice("([{")
            st.append(c)
            out.append(c)
    return "".join(out)


r = rng(20)
v_mid = valid(r, 600)
v_big = valid(r, 10**4)
broken_mid = v_mid[:300] + {"(": "[", "[": "{", "{": "(", ")": "]", "]": "}", "}": ")"}[v_mid[300]] + v_mid[301:]
deep = "".join(r.choice("([{") for _ in range(5000))
deep_valid = deep + "".join({"(": ")", "[": "]", "{": "}"}[c] for c in reversed(deep))
tests = [
    {"id": "t1", "input": "()", "sample": True, "expected": True},
    {"id": "t2", "input": "()[]{}", "sample": True, "expected": True},
    {"id": "t3", "input": "(]", "sample": True, "expected": False},
    {"id": "t4", "input": "([])", "sample": True, "expected": True},
    {"id": "t5", "input": "([)]", "sample": True, "expected": False},
    {"id": "single-open", "input": "(", "sample": False},
    {"id": "single-close", "input": "}", "sample": False},
    {"id": "close-first", "input": "][", "sample": False},
    {"id": "unclosed", "input": "{[()]", "sample": False},
    {"id": "extra-close", "input": "{[()]}}", "sample": False},
    {"id": "nested-mixed", "input": "{[()()]}[{}]", "sample": False},
    {"id": "random-valid-600", "input": v_mid, "sample": False},
    {"id": "random-broken-600", "input": broken_mid, "sample": False},
    {"id": "max-size-10000", "input": v_big, "sample": False},
    {"id": "max-size-10000-deep", "input": deep_valid, "sample": False},
    {"id": "max-size-10000-all-open", "input": "(" * 10**4, "sample": False},
    {"id": "deep-mismatch-at-end", "input": deep_valid[:-1] + ("]" if deep_valid[-1] != "]" else ")"), "sample": False},
]
write(problem, tests, solve)
