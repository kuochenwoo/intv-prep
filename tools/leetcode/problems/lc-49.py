import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-49",
    "title": "字母异位词分组",
    "difficulty": "medium",
    "tags": ["数组", "哈希表", "字符串", "排序"],
    "statement": "给定一个字符串数组 strs，把其中互为字母异位词的字符串分到同一组（字母异位词指字母种类和每种字母出现次数完全相同、仅排列不同的字符串）。返回所有分组；组与组之间、组内字符串之间的顺序都不限。",
    "en": {
        "title": "Group Anagrams",
        "tags": [
            "Array",
            "Hash Table",
            "String",
            "Sorting"
        ],
        "statement": "Given an array of strings strs, group together the strings that are anagrams of each other (anagrams are strings with exactly the same letters and the same count of each letter, differing only in arrangement). Return all groups; neither the order of the groups nor the order of strings within a group matters.",
        "constraints": [
            "1 ≤ strs.length ≤ 10000; 0 ≤ strs[i].length ≤ 100",
            "strs[i] consists only of lowercase English letters"
        ],
        "complexity": {
            "time": "O(n·k) (count key) or O(n·k·log k) (sorted key) (local training target)",
            "space": "O(n·k), the hash map stores all strings; output counted separately",
            "officialRequirement": "No official complexity requirement.",
            "notes": "n is the number of strings and k is the length of the longest string. Group by a hash key made from the sorted string or a 26-entry letter count; hash map costs are average case. The largest local test has 10000 strings of length ≤ 20 (the official per-string limit is 100; total characters were reduced to keep the problem bank small)."
        },
        "acmIoSpec": "stdin: first line n; then n lines, one string per line (a line may be empty). stdout: first line the number of groups g; then for each group, a line with its size k followed by k lines of strings. Group order and order within groups are arbitrary."
    },
    "constraints": [
        "1 ≤ strs.length ≤ 10000；0 ≤ strs[i].length ≤ 100",
        "strs[i] 只包含小写英文字母",
    ],
    "complexity": {
        "time": "O(n·k)（计数键）或 O(n·k·log k)（排序键）（本地训练目标）",
        "space": "O(n·k)，哈希表保存所有字符串；输出另计",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为字符串个数，k 为最长字符串长度。以排序后的字符串或 26 维字母计数作为哈希键分组；哈希表为平均情况。本地最大测试为 10000 个长度 ≤ 20 的字符串（官方单串长度上限 100，为控制题库体积而缩小总字符量）。",
        "sourceUrl": "https://leetcode.com/problems/group-anagrams/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    vector<vector<string>> groupAnagrams(vector<string>& strs) {\n        return {}; // Groups of anagrams, any order.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；随后 n 行，每行一个字符串（可能为空行）。stdout：第一行组数 g；随后每组先输出一行组内字符串个数 k，再输出 k 行字符串。组的顺序与组内顺序不限。",
    },
    # The largest test prints ~10000 lines plus group sizes, above the 64 KiB default.
    "limits": {"timeLimitMs": 1000, "memoryLimitMb": 128, "outputLimitKb": 512},
    "io": {
        "method": "groupAnagrams",
        "params": [{"name": "strs", "type": "string[]"}],
        "returns": "string[][]",
        "checker": "unordered_nested",
    },
}


def solve(strs):
    # Group by a 26-letter count signature.
    groups = {}
    for s in strs:
        key = tuple(s.count(c) for c in "abcdefghijklmnopqrstuvwxyz")
        groups.setdefault(key, []).append(s)
    return [sorted(g) for g in sorted(groups.values(), key=lambda g: sorted(g))]


def canon(v):
    return sorted(sorted(g) for g in v)


def check(inp, expected, solved):
    return canon(expected) == canon(solved)


r = rng(49)


def word(k, alphabet="abcdefghijklmnopqrstuvwxyz"):
    return "".join(r.choice(alphabet) for _ in range(k))


def anagram_pool(count, bases, maxlen, alphabet):
    base = [word(r.randint(0, maxlen), alphabet) for _ in range(bases)]
    out = []
    for _ in range(count):
        b = list(r.choice(base))
        r.shuffle(b)
        out.append("".join(b))
    return out


tests = [
    {"id": "t1", "input": ["eat", "tea", "tan", "ate", "nat", "bat"], "sample": True,
     "expected": [["bat"], ["nat", "tan"], ["ate", "eat", "tea"]]},
    {"id": "t2", "input": [""], "sample": True, "expected": [[""]]},
    {"id": "t3", "input": ["a"], "sample": True, "expected": [["a"]]},
    {"id": "duplicates-and-empty", "input": ["", "", "ab", "ba", "ab", "abc", "cab", "b"], "sample": False},
    {"id": "same-letters-diff-counts", "input": ["aab", "abb", "bab", "aba", "ab", "ba"], "sample": False},
    {"id": "all-distinct", "input": ["a", "b", "c", "ab", "ac", "bc", "abc", "abd"], "sample": False},
    {"id": "random-300", "input": anagram_pool(300, 40, 8, "abcde"), "sample": False},
    {"id": "long-strings-200", "input": anagram_pool(200, 20, 100, "abcdefghijklmnopqrstuvwxyz"), "sample": False},
    # 10^4 strings at the official count; lengths capped at 20 to keep the JSON small.
    {"id": "worst-case-10000-strings", "input": anagram_pool(10000, 2500, 20, "abcdefghij"), "sample": False},
]
write(problem, tests, solve, check=check)
