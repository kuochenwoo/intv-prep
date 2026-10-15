class Solution {
    // In-place swap backtracking.
    void go(vector<int>& a, size_t k, vector<vector<int>>& out) {
        if (k == a.size()) {
            out.push_back(a);
            return;
        }
        for (size_t i = k; i < a.size(); ++i) {
            swap(a[k], a[i]);
            go(a, k + 1, out);
            swap(a[k], a[i]);
        }
    }

public:
    vector<vector<int>> permute(vector<int>& nums) {
        vector<vector<int>> out;
        go(nums, 0, out);
        return out;
    }
};
