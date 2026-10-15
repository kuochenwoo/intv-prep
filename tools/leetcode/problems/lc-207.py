import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from gen import write, rng

problem = {
    "id": "lc-207",
    "title": "课程表",
    "difficulty": "medium",
    "tags": ["深度优先搜索", "广度优先搜索", "图", "拓扑排序"],
    "statement": "共有 numCourses 门课程，编号 0 到 numCourses - 1。prerequisites[i] = [a, b] 表示学习课程 a 之前必须先修完课程 b。判断是否能够修完全部课程，能则返回 true，否则返回 false。",
    "en": {'title': 'Course Schedule',
     'tags': ['Depth-First Search', 'Breadth-First Search', 'Graph', 'Topological Sort'],
     'statement': 'There are numCourses courses, numbered 0 to numCourses - 1. prerequisites[i] = [a, '
                  'b] means course b must be completed before taking course a. Determine whether all '
                  'courses can be completed; return true if so, otherwise false.',
     'constraints': ['1 ≤ numCourses ≤ 2000',
                     '0 ≤ prerequisites.length ≤ 5000',
                     'prerequisites[i].length == 2; 0 ≤ a, b < numCourses',
                     'All prerequisite pairs [a, b] are distinct'],
     'complexity': {'time': 'O(V + E) (local training target)',
                    'space': 'O(V + E) (adjacency list, in-degree array/queue, or DFS states and '
                             'recursion stack)',
                    'officialRequirement': 'No official complexity requirement.',
                    'notes': 'V = numCourses, E = prerequisites.length. Equivalent to checking whether '
                             "a directed graph is acyclic: Kahn's topological sort pops all V "
                             'vertices, or a three-color DFS finds no back edge.'},
     'acmIoSpec': 'stdin: line 1 is numCourses; line 2 is the number of prerequisite pairs p; then p '
                  'lines, each of the form 2 a b (2 is the row length). stdout: true or false.'},
    "constraints": [
        "1 ≤ numCourses ≤ 2000",
        "0 ≤ prerequisites.length ≤ 5000",
        "prerequisites[i].length == 2；0 ≤ a, b < numCourses",
        "所有先修关系 [a, b] 互不相同",
    ],
    "complexity": {
        "time": "O(V + E)（本地训练目标）",
        "space": "O(V + E)（邻接表、入度数组/队列或 DFS 状态与递归栈）",
        "officialRequirement": "官方未强制复杂度要求。",
        "notes": "V = numCourses，E = prerequisites.length。等价于判断有向图是否无环：Kahn 拓扑排序能弹出全部 V 个点，或三色 DFS 未遇到回边。",
        "sourceUrl": "https://leetcode.com/problems/course-schedule/",
    },
    "cpp": {
        "functionTemplate": "class Solution {\npublic:\n    bool canFinish(int numCourses, vector<vector<int>>& prerequisites) {\n        return false; // Whether every course can be finished.\n    }\n};",
        "acmIoSpec": "stdin：第一行 numCourses；第二行先修关系条数 p；随后 p 行，每行为 2 a b（2 是该行长度）。stdout：true 或 false。",
    },
    "io": {
        "method": "canFinish",
        "params": [{"name": "numCourses", "type": "int"}, {"name": "prerequisites", "type": "int[][]"}],
        "returns": "bool",
    },
}


def solve(data):
    # Repeatedly delete courses with no remaining prerequisite (set based, quadratic-ish
    # but fine for these sizes); independent of Kahn/DFS in the C++ solutions.
    n = data["numCourses"]
    need = [set() for _ in range(n)]
    users = [[] for _ in range(n)]
    for a, b in data["prerequisites"]:
        need[a].add(b)
        users[b].append(a)
    ready = [i for i in range(n) if not need[i]]
    done = 0
    while ready:
        c = ready.pop()
        done += 1
        for a in users[c]:
            need[a].discard(c)
            if not need[a]:
                ready.append(a)
    return done == n


def dag(r, n, e):
    order = list(range(n))
    r.shuffle(order)
    edges = set()
    while len(edges) < e:
        i, j = sorted(r.sample(range(n), 2))
        edges.add((order[j], order[i]))
    edges = [list(x) for x in edges]
    r.shuffle(edges)
    return edges


r = rng(207)
chain = [[i + 1, i] for i in range(1999)]
r.shuffle(chain)
big_dag = dag(r, 2000, 5000)
big_cycle = dag(r, 2000, 4999)
# Close a long cycle through many courses: walk a path found in a topological order.
cyc = [[i + 1, i] for i in range(1999)] + [[0, 1999]]
r.shuffle(cyc)
tests = [
    {"id": "t1", "sample": True, "expected": True, "input": {"numCourses": 2, "prerequisites": [[1, 0]]}},
    {"id": "t2", "sample": True, "expected": False, "input": {"numCourses": 2, "prerequisites": [[1, 0], [0, 1]]}},
    {"id": "single-course", "sample": False, "input": {"numCourses": 1, "prerequisites": []}},
    {"id": "no-prerequisites", "sample": False, "input": {"numCourses": 5, "prerequisites": []}},
    {"id": "self-loop", "sample": False, "input": {"numCourses": 3, "prerequisites": [[1, 0], [2, 2]]}},
    {"id": "diamond", "sample": False, "input": {"numCourses": 4, "prerequisites": [[1, 0], [2, 0], [3, 1], [3, 2]]}},
    {"id": "cycle-off-main", "sample": False,
     "input": {"numCourses": 6, "prerequisites": [[1, 0], [2, 1], [4, 3], [5, 4], [3, 5]]}},
    {"id": "random-dag-50", "sample": False, "input": {"numCourses": 50, "prerequisites": dag(r, 50, 120)}},
    {"id": "random-cycle-50", "sample": False,
     "input": {"numCourses": 50, "prerequisites": dag(r, 50, 119) + [[3, 40], [40, 3]]}},
    {"id": "long-chain-2000", "sample": False, "input": {"numCourses": 2000, "prerequisites": chain}},
    {"id": "long-cycle-2000", "sample": False, "input": {"numCourses": 2000, "prerequisites": cyc}},
    {"id": "max-size-2000-5000", "sample": False, "input": {"numCourses": 2000, "prerequisites": big_dag}},
    {"id": "max-size-2000-5000-cycle", "sample": False,
     "input": {"numCourses": 2000, "prerequisites": big_cycle + [[big_cycle[0][1], big_cycle[0][0]]]}},
]
write(problem, tests, solve)
