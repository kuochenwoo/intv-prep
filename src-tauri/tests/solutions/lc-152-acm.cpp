#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

// Prefix and suffix products restarting after zeros.
int main() {
    int n;
    cin >> n;
    vector<long long> a(n);
    for (auto& x : a) cin >> x;
    long long best = a[0], prefix = 1, suffix = 1;
    for (int i = 0; i < n; ++i) {
        prefix = (prefix == 0 ? 1 : prefix) * a[i];
        suffix = (suffix == 0 ? 1 : suffix) * a[n - 1 - i];
        best = max({best, prefix, suffix});
    }
    cout << best << '\n';
}
