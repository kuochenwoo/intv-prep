#include <algorithm>
#include <iomanip>
#include <iostream>
#include <vector>
using namespace std;

// k-th smallest by discarding k/2 elements per step: O(log(m + n)).
int kth(const vector<int>& a, const vector<int>& b, int k) {
    size_t i = 0, j = 0;
    while (true) {
        if (i == a.size()) return b[j + k - 1];
        if (j == b.size()) return a[i + k - 1];
        if (k == 1) return min(a[i], b[j]);
        size_t ni = min(a.size(), i + k / 2) - 1, nj = min(b.size(), j + k / 2) - 1;
        if (a[ni] <= b[nj]) { k -= ni - i + 1; i = ni + 1; }
        else { k -= nj - j + 1; j = nj + 1; }
    }
}

int main() {
    int m, n;
    cin >> m;
    vector<int> a(m);
    for (auto& x : a) cin >> x;
    cin >> n;
    vector<int> b(n);
    for (auto& x : b) cin >> x;
    int total = m + n;
    double median = total % 2 ? kth(a, b, total / 2 + 1)
                              : (kth(a, b, total / 2) + (double)kth(a, b, total / 2 + 1)) / 2.0;
    cout << setprecision(12) << median << '\n';
}
