#include <iostream>
#include <vector>
using namespace std;

// Direction simulation with a visited grid.
int main() {
    int m;
    cin >> m;
    vector<vector<int>> a(m);
    for (auto& row : a) {
        int n;
        cin >> n;
        row.resize(n);
        for (auto& x : row) cin >> x;
    }
    int n = a[0].size(), total = m * n;
    vector<vector<bool>> seen(m, vector<bool>(n, false));
    const int dr[4] = {0, 1, 0, -1}, dc[4] = {1, 0, -1, 0};
    int r = 0, c = 0, d = 0;
    for (int k = 0; k < total; ++k) {
        cout << (k ? " " : "") << a[r][c];
        seen[r][c] = true;
        int nr = r + dr[d], nc = c + dc[d];
        if (nr < 0 || nr >= m || nc < 0 || nc >= n || seen[nr][nc]) {
            d = (d + 1) % 4;
            nr = r + dr[d];
            nc = c + dc[d];
        }
        r = nr;
        c = nc;
    }
    cout << '\n';
}
