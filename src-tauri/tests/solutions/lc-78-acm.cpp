#include <iostream>
#include <vector>
using namespace std;

// Bitmask enumeration, independent of the backtracking Function-mode solution.
int main() {
    int n;
    cin >> n;
    vector<int> a(n);
    for (auto& x : a) cin >> x;
    cout << (1 << n) << '\n';
    for (int mask = 0; mask < (1 << n); ++mask) {
        bool first = true;
        for (int i = 0; i < n; ++i)
            if (mask >> i & 1) {
                cout << (first ? "" : " ") << a[i];
                first = false;
            }
        cout << '\n';
    }
}
