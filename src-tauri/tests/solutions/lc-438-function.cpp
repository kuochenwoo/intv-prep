class Solution {
public:
    vector<int> findAnagrams(string s, string p) {
        // Sliding window that tracks how many letters differ in count from p.
        int n = (int)s.size(), m = (int)p.size();
        vector<int> result;
        if (m > n) return result;
        int diff[26] = {0};
        for (int i = 0; i < m; ++i) { --diff[p[i] - 'a']; ++diff[s[i] - 'a']; }
        int mismatched = 0;
        for (int c = 0; c < 26; ++c) mismatched += diff[c] != 0;
        auto bump = [&](int c, int delta) {
            if (diff[c] == 0) ++mismatched;
            diff[c] += delta;
            if (diff[c] == 0) --mismatched;
        };
        if (mismatched == 0) result.push_back(0);
        for (int i = m; i < n; ++i) {
            bump(s[i] - 'a', 1);
            bump(s[i - m] - 'a', -1);
            if (mismatched == 0) result.push_back(i - m + 1);
        }
        return result;
    }
};
