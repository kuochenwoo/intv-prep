class Solution {
    // First index whose value is >= target (or > target when strict).
    int bound(const vector<int>& a, int target, bool strict) {
        int lo = 0, hi = a.size();
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            if (a[mid] < target || (strict && a[mid] == target)) lo = mid + 1;
            else hi = mid;
        }
        return lo;
    }
public:
    vector<int> searchRange(vector<int>& nums, int target) {
        int first = bound(nums, target, false);
        if (first == (int)nums.size() || nums[first] != target) return {-1, -1};
        return {first, bound(nums, target, true) - 1};
    }
};
