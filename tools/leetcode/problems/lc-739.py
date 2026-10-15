import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-739",
    "title": "每日温度",
    "difficulty": "medium",
    "tags": ["栈", "数组", "单调栈"],
    "statement": "给定整数数组 temperatures 表示每天的气温，返回数组 answer：answer[i] 是从第 i 天起，至少还要再等几天才会出现严格更高的气温；如果之后都不会更高，answer[i] = 0。",
    "constraints": [
        "1 ≤ temperatures.length ≤ 10^5",
        "30 ≤ temperatures[i] ≤ 100",
    ],
    "complexity": {
        "time": "O(n)",
        "space": "O(n) 额外空间（单调栈，不计返回数组）",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "n 为天数。维护一个下标栈，对应气温单调不增；新气温更高时依次弹栈并填写等待天数。也可从右往左利用已求出的 answer 跳跃查找。逐日向后扫描的 O(n²) 做法在大量相等气温时会超时。官方上限 n = 10^5，但输入与输出都需写入题库，本地最大测试缩小为 n = 50000（全部相等，答案全为 0，专门卡逐日向后扫描）。",
        "sourceUrl": "https://leetcode.com/problems/daily-temperatures/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    vector<int> dailyTemperatures(vector<int>& temperatures) {\n        return {}; // Days to wait for a warmer temperature.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数 temperatures。stdout：一行 n 个整数 answer，空格分隔。",
    },
    # 50000 answers print as ~100 KiB of text, above the 64 KiB default.
    "limits": {"timeLimitMs": 1000, "memoryLimitMb": 128, "outputLimitKb": 256},
    "io": {
        "method": "dailyTemperatures",
        "params": [{"name": "temperatures", "type": "int[]"}],
        "returns": "int[]",
    },
}


def solve(t):
    # Scan right to left, remembering the nearest future day of every temperature value.
    nearest = {}
    ans = [0] * len(t)
    for i in range(len(t) - 1, -1, -1):
        days = [nearest[v] for v in range(t[i] + 1, 101) if v in nearest]
        ans[i] = min(days) - i if days else 0
        nearest[t[i]] = i
    return ans


r = rng(739)
tests = [
    {"id": "t1", "input": [73, 74, 75, 71, 69, 72, 76, 73], "sample": True, "expected": [1, 1, 4, 2, 1, 1, 0, 0]},
    {"id": "t2", "input": [30, 40, 50, 60], "sample": True, "expected": [1, 1, 1, 0]},
    {"id": "t3", "input": [30, 60, 90], "sample": True, "expected": [1, 1, 0]},
    {"id": "single", "input": [55], "sample": False},
    {"id": "decreasing", "input": list(range(100, 29, -1)), "sample": False},
    {"id": "equal-not-warmer", "input": [50, 50, 50, 51], "sample": False},
    {"id": "valley", "input": [80, 70, 60, 50, 60, 70, 80, 90], "sample": False},
    {"id": "sawtooth", "input": [30 + (i % 7) * 10 for i in range(200)], "sample": False},
    {"id": "random-2000", "input": [r.randint(30, 100) for _ in range(2000)], "sample": False},
    {"id": "long-wait-5000", "input": [99] * 4999 + [100], "sample": False},
    {"id": "random-20000", "input": [r.randint(30, 100) for _ in range(20000)], "sample": False},
    # Official max is 1e5; input + output of that size would exceed the JSON budget.
    {"id": "worst-case-50000-all-equal", "input": [70] * 50000, "sample": False},
]
write(problem, tests, solve)
