#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

// Nearest smaller bar on each side via pointer jumping, then one pass.
int main() {
    int n;
    cin >> n;
    vector<int> h(n), left(n), right(n);
    for (auto& x : h) cin >> x;
    for (int i = 0; i < n; ++i) {
        int j = i - 1;
        while (j >= 0 && h[j] >= h[i]) j = left[j];
        left[i] = j;
    }
    for (int i = n - 1; i >= 0; --i) {
        int j = i + 1;
        while (j < n && h[j] >= h[i]) j = right[j];
        right[i] = j;
    }
    long long best = 0;
    for (int i = 0; i < n; ++i) best = max(best, (long long)h[i] * (right[i] - left[i] - 1));
    cout << best << '\n';
}
