class Solution {
public:
    double findMedianSortedArrays(vector<int>& nums1, vector<int>& nums2) {
        // Binary search the partition of the shorter array: O(log(min(m, n))).
        if (nums1.size() > nums2.size()) return findMedianSortedArrays(nums2, nums1);
        int m = nums1.size(), n = nums2.size(), half = (m + n + 1) / 2;
        int lo = 0, hi = m;
        while (lo <= hi) {
            int i = (lo + hi) / 2, j = half - i;
            int aLeft = i ? nums1[i - 1] : INT_MIN, aRight = i < m ? nums1[i] : INT_MAX;
            int bLeft = j ? nums2[j - 1] : INT_MIN, bRight = j < n ? nums2[j] : INT_MAX;
            if (aLeft > bRight) hi = i - 1;
            else if (bLeft > aRight) lo = i + 1;
            else {
                int leftMax = max(aLeft, bLeft);
                if ((m + n) % 2) return leftMax;
                return (leftMax + (double)min(aRight, bRight)) / 2.0;
            }
        }
        return 0.0;
    }
};
