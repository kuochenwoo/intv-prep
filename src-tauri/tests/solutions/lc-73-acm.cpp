#include <iostream>
#include <vector>
using namespace std;

// Row/column flag arrays, O(m + n) extra space.
int main() {
    int m;
    cin >> m;
    vector<vector<long long>> a(m);
    for (auto& row : a) {
        int n;
        cin >> n;
        row.resize(n);
        for (auto& x : row) cin >> x;
    }
    int n = a.empty() ? 0 : a[0].size();
    vector<bool> zr(m, false), zc(n, false);
    for (int i = 0; i < m; ++i)
        for (int j = 0; j < n; ++j)
            if (a[i][j] == 0) zr[i] = zc[j] = true;
    cout << m << '\n';
    for (int i = 0; i < m; ++i) {
        for (int j = 0; j < n; ++j) cout << (j ? " " : "") << ((zr[i] || zc[j]) ? 0 : a[i][j]);
        cout << '\n';
    }
}
