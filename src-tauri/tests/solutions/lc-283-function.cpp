class Solution {
public:
    void moveZeroes(vector<int>& nums) {
        int next = 0;
        for (int value : nums) if (value) nums[next++] = value;
        while (next < (int)nums.size()) nums[next++] = 0;
    }
};
