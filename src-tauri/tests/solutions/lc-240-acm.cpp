#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

// Binary search in every row, O(m log n).
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
    bool found = false;
    for (auto& row : a)
        if (binary_search(row.begin(), row.end(), target)) { found = true; break; }
    cout << (found ? "true" : "false") << '\n';
}
