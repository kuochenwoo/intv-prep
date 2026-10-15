class Solution {
    // Multi-source BFS layered by minute.
public:
    int orangesRotting(vector<vector<int>>& grid) {
        int m = grid.size(), n = grid[0].size(), fresh = 0;
        queue<pair<int, int>> q;
        for (int i = 0; i < m; ++i)
            for (int j = 0; j < n; ++j) {
                if (grid[i][j] == 2) q.push({i, j});
                else if (grid[i][j] == 1) ++fresh;
            }
        int minutes = 0;
        const int dr[4] = {1, -1, 0, 0}, dc[4] = {0, 0, 1, -1};
        while (!q.empty() && fresh > 0) {
            for (int k = q.size(); k > 0; --k) {
                auto [r, c] = q.front();
                q.pop();
                for (int d = 0; d < 4; ++d) {
                    int a = r + dr[d], b = c + dc[d];
                    if (a >= 0 && b >= 0 && a < m && b < n && grid[a][b] == 1) {
                        grid[a][b] = 2;
                        --fresh;
                        q.push({a, b});
                    }
                }
            }
            ++minutes;
        }
        return fresh == 0 ? minutes : -1;
    }
};
