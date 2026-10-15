#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

// Coordinate-compress all prefix sums, then count matches with a plain array (O(n log n)).
int main() {
    int n;
    long long k;
    cin >> n >> k;
    vector<long long> pre(n + 1, 0);
    for (int i = 1; i <= n; ++i) {
        long long x;
        cin >> x;
        pre[i] = pre[i - 1] + x;
    }
    vector<long long> keys(pre);
    sort(keys.begin(), keys.end());
    keys.erase(unique(keys.begin(), keys.end()), keys.end());
    auto index = [&](long long v) {
        auto it = lower_bound(keys.begin(), keys.end(), v);
        return (it != keys.end() && *it == v) ? (int)(it - keys.begin()) : -1;
    };
    vector<long long> seen(keys.size(), 0);
    long long total = 0;
    for (int j = 0; j <= n; ++j) {
        int want = index(pre[j] - k);
        if (want >= 0) total += seen[want];
        ++seen[index(pre[j])];
    }
    cout << total << '\n';
}
