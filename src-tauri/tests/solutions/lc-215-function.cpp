class Solution {
public:
    int findKthLargest(vector<int>& nums, int k) {
        // Randomized quickselect with three-way partitioning, iterative.
        int target = (int)nums.size() - k; // index in ascending order
        int lo = 0, hi = (int)nums.size() - 1;
        unsigned seed = 2463534242u;
        while (true) {
            seed ^= seed << 13; seed ^= seed >> 17; seed ^= seed << 5;
            int pivot = nums[lo + seed % (hi - lo + 1)];
            int lt = lo, i = lo, gt = hi; // [lo,lt) < pivot, [lt,i) == pivot, (gt,hi] > pivot
            while (i <= gt) {
                if (nums[i] < pivot) swap(nums[lt++], nums[i++]);
                else if (nums[i] > pivot) swap(nums[i], nums[gt--]);
                else ++i;
            }
            if (target < lt) hi = lt - 1;
            else if (target > gt) lo = gt + 1;
            else return pivot;
        }
    }
};
