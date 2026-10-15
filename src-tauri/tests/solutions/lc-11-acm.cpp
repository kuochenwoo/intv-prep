#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

// Process lines from tallest to shortest; the current line is the short side, so pair it
// with the farthest index already seen (all taller or equal). O(n log n).
int main() {
    int n;
    cin >> n;
    vector<int> h(n), order(n);
    for (auto& x : h) cin >> x;
    for (int i = 0; i < n; ++i) order[i] = i;
    sort(order.begin(), order.end(), [&](int a, int b) { return h[a] > h[b]; });
    long long best = 0;
    int lo = order[0], hi = order[0];
    for (int idx : order) {
        lo = min(lo, idx);
        hi = max(hi, idx);
        best = max(best, (long long)h[idx] * max(idx - lo, hi - idx));
    }
    cout << best << '\n';
}
