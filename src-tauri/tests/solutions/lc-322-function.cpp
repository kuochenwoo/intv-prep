class Solution {
public:
    int coinChange(vector<int>& coins, int amount) {
        const int INF = INT_MAX;
        vector<int> dp(amount + 1, INF);
        dp[0] = 0;
        for (int x = 1; x <= amount; ++x)
            for (int c : coins)
                if (c <= x && dp[x - c] != INF) dp[x] = min(dp[x], dp[x - c] + 1);
        return dp[amount] == INF ? -1 : dp[amount];
    }
};
