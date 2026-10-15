class Solution {
    // Depth-first backtracking over the keypad letters.
    const vector<string> keys{"", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"};
    void go(const string& digits, size_t i, string& cur, vector<string>& out) {
        if (i == digits.size()) {
            out.push_back(cur);
            return;
        }
        for (char c : keys[digits[i] - '0']) {
            cur.push_back(c);
            go(digits, i + 1, cur, out);
            cur.pop_back();
        }
    }

public:
    vector<string> letterCombinations(string digits) {
        vector<string> out;
        if (digits.empty()) return out;
        string cur;
        go(digits, 0, cur, out);
        return out;
    }
};
