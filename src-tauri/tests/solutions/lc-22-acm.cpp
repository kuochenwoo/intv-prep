#include <iostream>
#include <string>
#include <vector>
using namespace std;

// Catalan DP: every string of k pairs is "(" + A + ")" + B with |A| = i pairs, |B| = k-1-i pairs.
int main() {
    int n;
    cin >> n;
    vector<vector<string>> dp(n + 1);
    dp[0] = {""};
    for (int k = 1; k <= n; ++k)
        for (int i = 0; i < k; ++i)
            for (auto& a : dp[i])
                for (auto& b : dp[k - 1 - i]) dp[k].push_back("(" + a + ")" + b);
    cout << dp[n].size() << '\n';
    for (auto& s : dp[n]) cout << s << '\n';
}
