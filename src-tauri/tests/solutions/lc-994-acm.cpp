#include <climits>
#include <iostream>
#include <vector>
using namespace std;

// Distance relaxation: repeatedly lower each fresh cell's rot time from its neighbours
// until stable (Bellman-Ford style), independent of the queue-based BFS.
int main() {
    int m;
    cin >> m;
    vector<vector<int>> g(m);
    for (auto& row : g) {
        int k;
        cin >> k;
        row.resize(k);
        for (auto& x : row) cin >> x;
    }
    int n = g[0].size();
    const int INF = INT_MAX / 2;
    vector<vector<int>> t(m, vector<int>(n, INF));
    for (int i = 0; i < m; ++i)
        for (int j = 0; j < n; ++j)
            if (g[i][j] == 2) t[i][j] = 0;
    bool changed = true;
    while (changed) {
        changed = false;
        for (int i = 0; i < m; ++i)
            for (int j = 0; j < n; ++j) {
                if (g[i][j] != 1) continue;
                int best = t[i][j];
                if (i > 0 && g[i - 1][j] != 0) best = min(best, t[i - 1][j] + 1);
                if (i + 1 < m && g[i + 1][j] != 0) best = min(best, t[i + 1][j] + 1);
                if (j > 0 && g[i][j - 1] != 0) best = min(best, t[i][j - 1] + 1);
                if (j + 1 < n && g[i][j + 1] != 0) best = min(best, t[i][j + 1] + 1);
                if (best < t[i][j]) { t[i][j] = best; changed = true; }
            }
    }
    int answer = 0;
    for (int i = 0; i < m; ++i)
        for (int j = 0; j < n; ++j)
            if (g[i][j] == 1) {
                if (t[i][j] >= INF) { cout << -1 << '\n'; return 0; }
                answer = max(answer, t[i][j]);
            }
    cout << answer << '\n';
}
