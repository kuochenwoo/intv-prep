#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

// Array-based addition: pad the shorter number with zeros.
int main() {
    int m, n;
    cin >> m;
    vector<int> a(m);
    for (auto& x : a) cin >> x;
    cin >> n;
    vector<int> b(n);
    for (auto& x : b) cin >> x;
    size_t len = max(a.size(), b.size());
    a.resize(len, 0);
    b.resize(len, 0);
    vector<int> sum(len + 1, 0);
    for (size_t i = 0; i < len; ++i) {
        int s = a[i] + b[i] + sum[i];
        sum[i] = s % 10;
        sum[i + 1] = s / 10;
    }
    if (sum.back() == 0) sum.pop_back();
    for (size_t i = 0; i < sum.size(); ++i) cout << (i ? " " : "") << sum[i];
    cout << '\n';
}
