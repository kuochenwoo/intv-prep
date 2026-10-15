class Solution {
    // Backtracking over cut positions with a precomputed palindrome table.
    vector<vector<bool>> pal;
    void go(const string& s, size_t start, vector<string>& cur, vector<vector<string>>& out) {
        if (start == s.size()) {
            out.push_back(cur);
            return;
        }
        for (size_t end = start; end < s.size(); ++end) {
            if (!pal[start][end]) continue;
            cur.push_back(s.substr(start, end - start + 1));
            go(s, end + 1, cur, out);
            cur.pop_back();
        }
    }

public:
    vector<vector<string>> partition(string s) {
        int n = s.size();
        pal.assign(n, vector<bool>(n, false));
        for (int i = n - 1; i >= 0; --i)
            for (int j = i; j < n; ++j)
                pal[i][j] = s[i] == s[j] && (j - i < 2 || pal[i + 1][j - 1]);
        vector<vector<string>> out;
        vector<string> cur;
        go(s, 0, cur, out);
        return out;
    }
};
