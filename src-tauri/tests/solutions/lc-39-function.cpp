class Solution {
    // Sorted backtracking: reuse index i, stop once a candidate exceeds what is left.
    void go(const vector<int>& c, size_t i, int left, vector<int>& cur, vector<vector<int>>& out) {
        if (left == 0) {
            out.push_back(cur);
            return;
        }
        for (size_t j = i; j < c.size() && c[j] <= left; ++j) {
            cur.push_back(c[j]);
            go(c, j, left - c[j], cur, out);
            cur.pop_back();
        }
    }

public:
    vector<vector<int>> combinationSum(vector<int>& candidates, int target) {
        sort(candidates.begin(), candidates.end());
        vector<vector<int>> out;
        vector<int> cur;
        go(candidates, 0, target, cur, out);
        return out;
    }
};
