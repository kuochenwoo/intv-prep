class Solution {
public:
    string minWindow(string s, string t) {
        // Sliding window with a single "missing characters" counter.
        int need[128] = {0};
        for (char c : t) ++need[(unsigned char)c];
        int missing = (int)t.size(), left = 0, bestStart = 0, bestLen = INT_MAX;
        for (int right = 0; right < (int)s.size(); ++right) {
            if (need[(unsigned char)s[right]]-- > 0) --missing;
            while (missing == 0) {
                if (right - left + 1 < bestLen) { bestLen = right - left + 1; bestStart = left; }
                if (++need[(unsigned char)s[left++]] > 0) ++missing;
            }
        }
        return bestLen == INT_MAX ? "" : s.substr(bestStart, bestLen);
    }
};
