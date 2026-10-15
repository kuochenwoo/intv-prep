#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

// Two-stage binary search: last row whose first element <= target, then that row.
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
    int target;
    cin >> target;
    int lo = 0, hi = m - 1, row = -1;
    while (lo <= hi) {
        int mid = (lo + hi) / 2;
        if (a[mid][0] <= target) { row = mid; lo = mid + 1; }
        else hi = mid - 1;
    }
    bool found = row >= 0 && binary_search(a[row].begin(), a[row].end(), target);
    cout << (found ? "true" : "false") << '\n';
}
