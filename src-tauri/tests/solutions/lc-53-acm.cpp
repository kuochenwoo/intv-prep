#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

// Divide and conquer that merges (total, best prefix, best suffix, best) per segment: O(n).
struct Info { long long total, prefix, suffix, best; };

vector<long long> a;

Info solve(int lo, int hi) {
    if (lo == hi) return {a[lo], a[lo], a[lo], a[lo]};
    int mid = (lo + hi) / 2;
    Info l = solve(lo, mid), r = solve(mid + 1, hi);
    return {l.total + r.total,
            max(l.prefix, l.total + r.prefix),
            max(r.suffix, r.total + l.suffix),
            max({l.best, r.best, l.suffix + r.prefix})};
}

int main() {
    int n;
    cin >> n;
    a.resize(n);
    for (auto& x : a) cin >> x;
    cout << solve(0, n - 1).best << '\n';
}
