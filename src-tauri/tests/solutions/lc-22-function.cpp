class Solution {
    // Backtracking: add '(' while fewer than n, ')' while it stays balanced.
    void go(int open, int close, int n, string& cur, vector<string>& out) {
        if ((int)cur.size() == 2 * n) {
            out.push_back(cur);
            return;
        }
        if (open < n) {
            cur.push_back('(');
            go(open + 1, close, n, cur, out);
            cur.pop_back();
        }
        if (close < open) {
            cur.push_back(')');
            go(open, close + 1, n, cur, out);
            cur.pop_back();
        }
    }

public:
    vector<string> generateParenthesis(int n) {
        vector<string> out;
        string cur;
        go(0, 0, n, cur, out);
        return out;
    }
};
