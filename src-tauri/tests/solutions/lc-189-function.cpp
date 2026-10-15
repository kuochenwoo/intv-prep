class Solution {
public:
    void rotate(vector<int>& nums, int k) {
        // Three reversals, O(1) extra space.
        int n = (int)nums.size();
        k %= n;
        reverse(nums.begin(), nums.end());
        reverse(nums.begin(), nums.begin() + k);
        reverse(nums.begin() + k, nums.end());
    }
};
