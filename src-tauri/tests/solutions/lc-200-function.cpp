class Solution {
    // Iterative DFS with an explicit stack: no recursion-depth concerns.
public:
    int numIslands(vector<vector<char>>& grid) {
        int m = grid.size(), n = grid[0].size(), count = 0;
        vector<pair<int, int>> st;
        const int dr[4] = {1, -1, 0, 0}, dc[4] = {0, 0, 1, -1};
        for (int i = 0; i < m; ++i)
            for (int j = 0; j < n; ++j) {
                if (grid[i][j] != '1') continue;
                ++count;
                grid[i][j] = '0';
                st.push_back({i, j});
                while (!st.empty()) {
                    auto [r, c] = st.back();
                    st.pop_back();
                    for (int d = 0; d < 4; ++d) {
                        int a = r + dr[d], b = c + dc[d];
                        if (a >= 0 && b >= 0 && a < m && b < n && grid[a][b] == '1') {
                            grid[a][b] = '0';
                            st.push_back({a, b});
                        }
                    }
                }
            }
        return count;
    }
};
