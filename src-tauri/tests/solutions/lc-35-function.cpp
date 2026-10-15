class Solution {
public:
    int searchInsert(vector<int>& nums, int target) {
        // First index with nums[i] >= target, half-open interval.
        int lo = 0, hi = nums.size();
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            if (nums[mid] < target) lo = mid + 1;
            else hi = mid;
        }
        return lo;
    }
};
