import sys, pathlib, itertools
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-17",
    "title": "电话号码的字母组合",
    "difficulty": "medium",
    "tags": ["哈希表", "字符串", "回溯"],
    "statement": "给定一个只包含数字 2-9 的字符串 digits，按电话按键的映射（2: abc，3: def，4: ghi，5: jkl，6: mno，7: pqrs，8: tuv，9: wxyz）返回它能表示的所有字母组合，顺序任意。数字 1 不对应任何字母。若 digits 为空，返回空列表。",
    "constraints": [
        "1 ≤ digits.length ≤ 4（官方现行约束；旧版题面允许长度为 0）",
        "digits[i] 是 ['2', '9'] 范围内的数字",
        "本地额外包含 digits 为空串的测试，应返回空列表",
    ],
    "complexity": {
        "time": "O(n · 4^n)（本地训练目标）",
        "space": "O(n) 额外空间（回溯递归栈与当前路径，计入递归栈；输出不计入）",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为 digits 长度；每个数字最多对应 4 个字母，组合数至多 4^n，每个长度 n。判题忽略组合之间的顺序。",
        "sourceUrl": "https://leetcode.com/problems/letter-combinations-of-a-phone-number/",
    },
    "en": {
        "title": "Letter Combinations of a Phone Number",
        "tags": [
            "Hash Table",
            "String",
            "Backtracking"
        ],
        "statement": "Given a string digits containing only the digits 2-9, return all letter combinations it can represent using the phone keypad mapping (2: abc, 3: def, 4: ghi, 5: jkl, 6: mno, 7: pqrs, 8: tuv, 9: wxyz), in any order. The digit 1 maps to no letters. If digits is empty, return an empty list.",
        "constraints": [
            "1 ≤ digits.length ≤ 4 (current official constraint; an older version of the problem allowed length 0)",
            "digits[i] is a digit in the range ['2', '9']",
            "Locally there is an extra test with an empty digits string, which should return an empty list"
        ],
        "complexity": {
            "time": "O(n · 4^n) (local training target)",
            "space": "O(n) extra space (backtracking recursion stack and current path, recursion stack included; output not counted)",
            "officialRequirement": "No official complexity requirement.",
            "notes": "n is the length of digits; each digit maps to at most 4 letters, so there are at most 4^n combinations, each of length n. The judge ignores the order of the combinations."
        },
        "acmIoSpec": "stdin: one line, the string digits (may be an empty line). stdout: line 1 is the number of combinations k; then k lines, each with one letter combination (any order)."
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    vector<string> letterCombinations(string digits) {\n        return {}; // All letter combinations, in any order.\n    }\n};",
        "acmIoSpec": "stdin：一行字符串 digits（可能为空行）。stdout：第一行组合个数 k；随后 k 行，每行一个字母组合（顺序任意）。",
    },
    "io": {
        "method": "letterCombinations",
        "params": [{"name": "digits", "type": "string"}],
        "returns": "string[]",
        "checker": "unordered",
    },
}

KEYS = {"2": "abc", "3": "def", "4": "ghi", "5": "jkl", "6": "mno", "7": "pqrs", "8": "tuv", "9": "wxyz"}


def solve(digits):
    if not digits:
        return []
    return ["".join(p) for p in itertools.product(*(KEYS[d] for d in digits))]


def check(inp, expected, solved):
    return sorted(expected) == sorted(solved)


r = rng(17)
tests = [
    {"id": "t1", "sample": True, "input": "23",
     "expected": ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]},
    {"id": "t2", "sample": True, "input": "2", "expected": ["a", "b", "c"]},
    {"id": "empty", "sample": False, "input": ""},
    {"id": "single-seven", "sample": False, "input": "7"},
    {"id": "repeated", "sample": False, "input": "222"},
    {"id": "random-3", "sample": False, "input": "".join(r.choice("23456789") for _ in range(3))},
    {"id": "max-size-4", "sample": False, "input": "7979"},
]
write(problem, tests, solve, check=check)
