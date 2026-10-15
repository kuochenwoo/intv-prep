#include <iostream>
#include <vector>
using namespace std;

// Layer-by-layer four-way cyclic swaps.
int main() {
    int n;
    cin >> n;
    vector<vector<int>> a(n);
    for (auto& row : a) {
        int k;
        cin >> k;
        row.resize(k);
        for (auto& x : row) cin >> x;
    }
    for (int i = 0; i < n / 2; ++i) {
        for (int j = i; j < n - 1 - i; ++j) {
            int t = a[i][j];
            a[i][j] = a[n - 1 - j][i];
            a[n - 1 - j][i] = a[n - 1 - i][n - 1 - j];
            a[n - 1 - i][n - 1 - j] = a[j][n - 1 - i];
            a[j][n - 1 - i] = t;
        }
    }
    cout << n << '\n';
    for (auto& row : a) {
        for (int j = 0; j < n; ++j) cout << (j ? " " : "") << row[j];
        cout << '\n';
    }
}
