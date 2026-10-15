class Solution {
public:
    void sortColors(vector<int>& nums) {
        // Dutch national flag: [0, low) = 0, [low, mid) = 1, (high, end) = 2.
        int low = 0, mid = 0, high = (int)nums.size() - 1;
        while (mid <= high) {
            if (nums[mid] == 0) swap(nums[low++], nums[mid++]);
            else if (nums[mid] == 2) swap(nums[mid], nums[high--]);
            else ++mid;
        }
    }
};
