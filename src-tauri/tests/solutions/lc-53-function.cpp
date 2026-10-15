class Solution {
public:
    int maxSubArray(vector<int>& nums) {
        // Kadane's algorithm.
        int best = nums[0], current = nums[0];
        for (size_t i = 1; i < nums.size(); ++i) {
            current = max(nums[i], current + nums[i]);
            best = max(best, current);
        }
        return best;
    }
};
