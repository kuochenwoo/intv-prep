import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-39",
    "title": "组合总和",
    "difficulty": "medium",
    "tags": ["数组", "回溯"],
    "statement": "给定一个元素互不相同的正整数数组 candidates 和目标值 target，找出 candidates 中所有和为 target 的不同组合，每个数可以被重复选取任意次。两个组合中各数字出现次数完全相同则视为同一组合。组合之间以及组合内部的顺序任意。",
    "constraints": [
        "1 ≤ candidates.length ≤ 30",
        "2 ≤ candidates[i] ≤ 40，且互不相同",
        "1 ≤ target ≤ 40",
        "保证不同组合的数量少于 150 个",
    ],
    "complexity": {
        "time": "与解的数量相关的指数级（本地训练目标：回溯 + 排序剪枝，搜索树规模约为 O(S)，S 为所有可行路径长度之和）",
        "space": "O(target / min(candidates)) 额外空间（递归栈与当前组合，计入递归栈；输出不计入）",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为 candidates 长度。按非递减顺序选数（下一层从当前下标开始）避免重复组合；排序后当前数超过剩余目标即可停止本层。判题同时忽略组合之间及组合内部的顺序。",
        "sourceUrl": "https://leetcode.com/problems/combination-sum/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    vector<vector<int>> combinationSum(vector<int>& candidates, int target) {\n        return {}; // Unique combinations summing to target, in any order.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个互不相同的整数 candidates；第三行 target。stdout：第一行组合个数；随后每行一个组合，元素用空格分隔（组合之间及内部顺序任意）。",
    },
    "io": {
        "method": "combinationSum",
        "params": [{"name": "candidates", "type": "int[]"}, {"name": "target", "type": "int"}],
        "returns": "int[][]",
        "checker": "unordered_nested",
    },
}


def solve(data):
    # Count-vector recursion over candidates in input order (multiplicity of each value).
    cand, out = data["candidates"], []

    def go(i, left, cur):
        if left == 0:
            out.append(list(cur))
            return
        if i == len(cand):
            return
        k = 0
        while k * cand[i] <= left:
            go(i + 1, left - k * cand[i], cur + [cand[i]] * k)
            k += 1

    go(0, data["target"], [])
    return out


def check(inp, expected, solved):
    norm = lambda v: sorted(tuple(sorted(x)) for x in v)
    return norm(expected) == norm(solved)


def count(cand, target):
    return len(solve({"candidates": cand, "target": target}))


def max_case(seed):
    # 30 distinct candidates; the largest target ≤ 40 that keeps fewer than 150 combinations.
    r = rng(seed)
    while True:
        cand = r.sample(range(2, 41), 30)
        for target in range(40, 0, -1):
            if count(cand, target) < 150:
                if count(cand, target) >= 100:
                    return {"candidates": cand, "target": target}
                break


r = rng(39)
tests = [
    {"id": "t1", "sample": True, "input": {"candidates": [2, 3, 6, 7], "target": 7}, "expected": [[2, 2, 3], [7]]},
    {"id": "t2", "sample": True, "input": {"candidates": [2, 3, 5], "target": 8},
     "expected": [[2, 2, 2, 2], [2, 3, 3], [3, 5]]},
    {"id": "t3", "sample": True, "input": {"candidates": [2], "target": 1}, "expected": []},
    {"id": "target-equals-one-candidate", "sample": False, "input": {"candidates": [40], "target": 40}},
    {"id": "only-repeats", "sample": False, "input": {"candidates": [2], "target": 40}},
    {"id": "unsorted", "sample": False, "input": {"candidates": [8, 7, 4, 3], "target": 11}},
    {"id": "no-solution", "sample": False, "input": {"candidates": [4, 6, 8, 10], "target": 39}},
    {"id": "random-10", "sample": False, "input": {"candidates": r.sample(range(2, 41), 10), "target": 30}},
    {"id": "max-size-30", "sample": False, "input": max_case(1)},
    {"id": "max-size-30-large-parts", "sample": False, "input": {"candidates": list(range(40, 10, -1)), "target": 40}},
]
write(problem, tests, solve, check=check)
