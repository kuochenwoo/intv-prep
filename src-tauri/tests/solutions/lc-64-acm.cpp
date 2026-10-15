#include <algorithm>
#include <climits>
#include <iostream>
#include <vector>
using namespace std;

// Rolling one-row DP processed while reading.
int main() {
    int m;
    cin >> m;
    vector<long long> dp;
    for (int i = 0; i < m; ++i) {
        int n;
        cin >> n;
        if (i == 0) dp.assign(n, LLONG_MAX);
        for (int j = 0; j < n; ++j) {
            long long v;
            cin >> v;
            long long from = (i == 0 && j == 0) ? 0 : min(dp[j], j ? dp[j - 1] : LLONG_MAX);
            dp[j] = from + v;
        }
    }
    cout << dp.back() << '\n';
}
