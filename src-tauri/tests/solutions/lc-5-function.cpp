class Solution {
public:
    string longestPalindrome(string s) {
        // Expand around each of the 2n - 1 centers.
        int n = s.size(), start = 0, len = 1;
        for (int c = 0; c < 2 * n - 1; ++c) {
            int l = c / 2, r = l + c % 2;
            while (l >= 0 && r < n && s[l] == s[r]) { --l; ++r; }
            if (r - l - 1 > len) { len = r - l - 1; start = l + 1; }
        }
        return s.substr(start, len);
    }
};
