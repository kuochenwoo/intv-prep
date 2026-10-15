class Solution {
    // Kahn's topological sort: finishable iff every course is popped.
public:
    bool canFinish(int numCourses, vector<vector<int>>& prerequisites) {
        vector<vector<int>> next(numCourses);
        vector<int> indegree(numCourses, 0);
        for (auto& p : prerequisites) {
            next[p[1]].push_back(p[0]);
            ++indegree[p[0]];
        }
        queue<int> q;
        for (int i = 0; i < numCourses; ++i)
            if (indegree[i] == 0) q.push(i);
        int taken = 0;
        while (!q.empty()) {
            int c = q.front();
            q.pop();
            ++taken;
            for (int x : next[c])
                if (--indegree[x] == 0) q.push(x);
        }
        return taken == numCourses;
    }
};
