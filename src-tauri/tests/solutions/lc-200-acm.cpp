#include <iostream>
#include <queue>
#include <string>
#include <utility>
#include <vector>
using namespace std;

// BFS flood fill with a visited array, independent of the stack-based Function-mode solution.
int main() {
    int m;
    cin >> m;
    vector<string> grid(m);
    for (auto& row : grid) cin >> row;
    int n = grid[0].size(), count = 0;
    vector<vector<bool>> seen(m, vector<bool>(n, false));
    for (int i = 0; i < m; ++i)
        for (int j = 0; j < n; ++j) {
            if (grid[i][j] != '1' || seen[i][j]) continue;
            ++count;
            queue<pair<int, int>> q;
            q.push({i, j});
            seen[i][j] = true;
            while (!q.empty()) {
                auto [r, c] = q.front();
                q.pop();
                int nr[4] = {r + 1, r - 1, r, r}, nc[4] = {c, c, c + 1, c - 1};
                for (int d = 0; d < 4; ++d) {
                    int a = nr[d], b = nc[d];
                    if (a < 0 || b < 0 || a >= m || b >= n || seen[a][b] || grid[a][b] != '1') continue;
                    seen[a][b] = true;
                    q.push({a, b});
                }
            }
        }
    cout << count << '\n';
}
