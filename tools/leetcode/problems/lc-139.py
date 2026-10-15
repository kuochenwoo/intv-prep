import sys, pathlib, string
from functools import lru_cache
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-139",
    "title": "单词拆分",
    "difficulty": "medium",
    "tags": ["字典树", "记忆化搜索", "数组", "哈希表", "字符串", "动态规划"],
    "statement": "给定字符串 s 和字符串列表 wordDict（字典）。判断 s 能否被切分成一个或多个字典中的单词依次拼接而成；字典中的单词可以重复使用。",
    "en": {'title': 'Word Break',
     'tags': ['Trie', 'Memoization', 'Array', 'Hash Table', 'String', 'Dynamic Programming'],
     'statement': 'Given a string s and a list of strings wordDict (the dictionary), determine whether s can be '
                  'split into a concatenation of one or more dictionary words in order; dictionary words may be '
                  'reused.',
     'constraints': ['1 ≤ s.length ≤ 300; 1 ≤ wordDict.length ≤ 1000; 1 ≤ wordDict[i].length ≤ 20',
                     's and wordDict[i] consist only of lowercase English letters; all words in wordDict are '
                     'distinct'],
     'complexity': {'time': 'O(n · L²), or O(n · L) with a trie (local training target)',
                    'space': 'O(n + total dictionary length) extra space',
                    'officialRequirement': 'No official complexity requirement.',
                    'notes': 'n is the length of s and L ≤ 20 is the longest word length. dp[i] means the prefix '
                             's[0..i) can be split; only the last L split points need to be checked. '
                             'Backtracking without memoization is exponential on inputs like "aaa…ab".'},
     'acmIoSpec': 'stdin: line 1 is the string s; line 2 is the word count k; then k lines, one word each. '
                  'stdout: true or false.'},
    "constraints": [
        "1 ≤ s.length ≤ 300；1 ≤ wordDict.length ≤ 1000；1 ≤ wordDict[i].length ≤ 20",
        "s 与 wordDict[i] 仅由小写英文字母组成；wordDict 中的单词互不相同",
    ],
    "complexity": {
        "time": "O(n · L²) 或 O(n · L) 配合字典树（本地训练目标）",
        "space": "O(n + 字典总长度) 额外空间",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为 s 的长度，L ≤ 20 为最长单词长度。dp[i] 表示前缀 s[0..i) 可拆分；只需回看最近 L 个切分点。不带记忆化的回溯在 \"aaa…ab\" 这类输入上是指数级。",
        "sourceUrl": "https://leetcode.com/problems/word-break/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    bool wordBreak(string s, vector<string>& wordDict) {\n        return false; // Whether s splits into dictionary words.\n    }\n};",
        "acmIoSpec": "stdin：第一行字符串 s；第二行单词数 k；随后 k 行，每行一个单词。stdout：true 或 false。",
    },
    "io": {
        "method": "wordBreak",
        "params": [{"name": "s", "type": "string"}, {"name": "wordDict", "type": "string[]"}],
        "returns": "bool",
    },
}


def solve(data):
    s, words = data["s"], set(data["wordDict"])
    assert len(words) == len(data["wordDict"])

    @lru_cache(maxsize=None)
    def ok(i):  # can s[i:] be split?
        return i == len(s) or any(s.startswith(w, i) and ok(i + len(w)) for w in words)

    return ok(0)


r = rng(139)


def word(lo, hi, alpha=string.ascii_lowercase):
    return "".join(r.choice(alpha) for _ in range(r.randint(lo, hi)))


def unique(words):
    seen, out = set(), []
    for w in words:
        if w not in seen:
            seen.add(w); out.append(w)
    return out


a_runs = ["a" * k for k in range(1, 21)]
filler = unique(word(3, 20, "bcdefg") for _ in range(1200))[:980]
words_ab = unique(word(1, 6, "ab") for _ in range(400))[:60]
true_s = ""
while len(true_s) < 300:
    true_s += r.choice(words_ab)
true_s = true_s[:300]
tests = [
    {"id": "t1", "input": {"s": "leetcode", "wordDict": ["leet", "code"]}, "sample": True, "expected": True},
    {"id": "t2", "input": {"s": "applepenapple", "wordDict": ["apple", "pen"]}, "sample": True, "expected": True},
    {"id": "t3", "input": {"s": "catsandog", "wordDict": ["cats", "dog", "sand", "and", "cat"]}, "sample": True, "expected": False},
    {"id": "single-char", "input": {"s": "a", "wordDict": ["a"]}, "sample": False},
    {"id": "word-longer", "input": {"s": "ab", "wordDict": ["abc"]}, "sample": False},
    {"id": "needs-backtrack", "input": {"s": "aaaaaaab", "wordDict": ["aaaa", "aaa", "aab"]}, "sample": False},
    {"id": "prefix-trap", "input": {"s": "cars", "wordDict": ["car", "ca", "rs"]}, "sample": False},
    {"id": "random-ab", "input": {"s": true_s[:60], "wordDict": words_ab}, "sample": False},
    # Exponential for unmemoized backtracking; every word is distinct.
    {"id": "max-size-a-run-false", "input": {"s": "a" * 299 + "b", "wordDict": a_runs + filler}, "sample": False},
    {"id": "max-size-a-run-true", "input": {"s": "a" * 300, "wordDict": a_runs[1:] + filler}, "sample": False},
    {"id": "max-size-ab", "input": {"s": true_s, "wordDict": words_ab}, "sample": False},
]
write(problem, tests, solve)
