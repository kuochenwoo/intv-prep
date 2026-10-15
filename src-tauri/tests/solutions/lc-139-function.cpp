class Solution {
public:
    bool wordBreak(string s, vector<string>& wordDict) {
        unordered_set<string> dict(wordDict.begin(), wordDict.end());
        size_t maxLen = 0;
        for (auto& w : wordDict) maxLen = max(maxLen, w.size());
        int n = s.size();
        vector<bool> dp(n + 1, false); // dp[i]: prefix of length i splits
        dp[0] = true;
        for (int i = 1; i <= n; ++i)
            for (int j = i - 1; j >= 0 && i - j <= (int)maxLen && !dp[i]; --j)
                if (dp[j] && dict.count(s.substr(j, i - j))) dp[i] = true;
        return dp[n];
    }
};
