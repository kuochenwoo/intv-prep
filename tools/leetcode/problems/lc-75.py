import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-75",
    "title": "颜色分类",
    "difficulty": "medium",
    "tags": ["数组", "双指针", "排序", "原地修改"],
    "statement": "数组 nums 中的 0、1、2 分别代表红、白、蓝三种颜色。请原地重排数组，使相同颜色相邻并按红、白、蓝（0、1、2）的顺序排列，不得调用库函数排序。核心函数不返回值，系统检查修改后的数组。",
    "constraints": ["1 ≤ n ≤ 300；nums[i] 为 0、1 或 2"],
    "complexity": {
        "time": "O(n)，一趟扫描（本地训练目标）",
        "space": "O(1) 额外空间，原地修改",
        "officialRequirement": "官方要求原地排序且不使用库排序函数；follow-up 要求只扫描一趟、使用常数额外空间。",
        "notes": "n 为数组长度。荷兰国旗三指针：low 之前全为 0，high 之后全为 2，mid 扫描未处理区域。两趟计数重写也是 O(n) / O(1)，但不满足一趟扫描的 follow-up；判题只检查结果，无法验证是否调用了 sort 或扫描趟数。",
        "sourceUrl": "https://leetcode.com/problems/sort-colors/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    void sortColors(vector<int>& nums) {\n        // Rearrange nums in place: all 0s, then 1s, then 2s.\n    }\n};",
        "acmIoSpec": "stdin：第一行 n；第二行 n 个整数 nums（取值 0、1、2）。stdout：一行，排序后的 n 个整数，以空格分隔。",
    },
    "io": {
        "method": "sortColors",
        "params": [{"name": "nums", "type": "int[]"}],
        "returns": "void",
        "output": "nums",
    },
}


def solve(a):
    return sorted(a)


r = rng(75)
tests = [
    {"id": "t1", "input": [2, 0, 2, 1, 1, 0], "sample": True, "expected": [0, 0, 1, 1, 2, 2]},
    {"id": "t2", "input": [2, 0, 1], "sample": True, "expected": [0, 1, 2]},
    {"id": "single", "input": [1], "sample": False},
    {"id": "all-twos", "input": [2, 2, 2, 2], "sample": False},
    {"id": "no-ones", "input": [2, 0, 2, 0, 0, 2], "sample": False},
    {"id": "already-sorted", "input": [0, 0, 1, 2, 2], "sample": False},
    {"id": "random-50", "input": [r.randint(0, 2) for _ in range(50)], "sample": False},
    {"id": "max-size-300", "input": [r.randint(0, 2) for _ in range(300)], "sample": False},
    {"id": "max-size-300-reversed", "input": [2] * 100 + [1] * 100 + [0] * 100, "sample": False},
]
write(problem, tests, solve)
