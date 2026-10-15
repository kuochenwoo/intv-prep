#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

// Lexicographic enumeration with next_permutation, independent of swap backtracking.
int main() {
    int n;
    cin >> n;
    vector<int> a(n);
    for (auto& x : a) cin >> x;
    sort(a.begin(), a.end());
    vector<vector<int>> out;
    do out.push_back(a); while (next_permutation(a.begin(), a.end()));
    cout << out.size() << '\n';
    for (auto& p : out) {
        for (int i = 0; i < n; ++i) cout << (i ? " " : "") << p[i];
        cout << '\n';
    }
}
