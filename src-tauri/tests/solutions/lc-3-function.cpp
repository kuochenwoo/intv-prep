class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        vector<int> last(128, -1);
        int left = 0, best = 0;
        for (int i = 0; i < (int)s.size(); ++i) {
            left = max(left, last[(unsigned char)s[i]] + 1);
            last[(unsigned char)s[i]] = i;
            best = max(best, i - left + 1);
        }
        return best;
    }
};
