class Solution {
public:
    int firstMissingPositive(vector<int>& nums) {
        // In-place hashing: put each value v in [1, n] at index v - 1.
        int n = (int)nums.size();
        for (int i = 0; i < n; ++i) {
            while (nums[i] > 0 && nums[i] <= n && nums[nums[i] - 1] != nums[i])
                swap(nums[i], nums[nums[i] - 1]);
        }
        for (int i = 0; i < n; ++i)
            if (nums[i] != i + 1) return i + 1;
        return n + 1;
    }
};
