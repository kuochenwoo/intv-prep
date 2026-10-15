#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

// Kadane over day-to-day price changes: the best contiguous run of gains.
int main() {
    int n;
    cin >> n;
    vector<int> prices(n);
    for (auto& x : prices) cin >> x;
    long long run = 0, best = 0;
    for (int i = 1; i < n; ++i) {
        run = max(0LL, run + prices[i] - prices[i - 1]);
        best = max(best, run);
    }
    cout << best << '\n';
}
