class Solution {
    // Include/exclude backtracking.
    void go(const vector<int>& nums, size_t i, vector<int>& cur, vector<vector<int>>& out) {
        if (i == nums.size()) {
            out.push_back(cur);
            return;
        }
        go(nums, i + 1, cur, out);
        cur.push_back(nums[i]);
        go(nums, i + 1, cur, out);
        cur.pop_back();
    }

public:
    vector<vector<int>> subsets(vector<int>& nums) {
        vector<vector<int>> out;
        vector<int> cur;
        go(nums, 0, cur, out);
        return out;
    }
};
