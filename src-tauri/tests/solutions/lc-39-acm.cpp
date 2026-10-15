#include <iostream>
#include <vector>
using namespace std;

// Unbounded-knapsack style DP over candidates: dp[s] holds every combination summing to s
// using only the candidates processed so far, so each multiset is built exactly once.
int main() {
    int n, target;
    cin >> n;
    vector<int> c(n);
    for (auto& x : c) cin >> x;
    cin >> target;
    vector<vector<vector<int>>> dp(target + 1);
    dp[0].push_back({});
    for (int x : c)
        for (int s = x; s <= target; ++s)
            for (auto comb : dp[s - x]) {
                comb.push_back(x);
                dp[s].push_back(comb);
            }
    cout << dp[target].size() << '\n';
    for (auto& comb : dp[target]) {
        for (size_t i = 0; i < comb.size(); ++i) cout << (i ? " " : "") << comb[i];
        cout << '\n';
    }
}
