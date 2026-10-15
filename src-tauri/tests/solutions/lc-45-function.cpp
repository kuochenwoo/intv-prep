class Solution {
public:
    int jump(vector<int>& nums) {
        int n = nums.size(), jumps = 0, layerEnd = 0, furthest = 0;
        for (int i = 0; i + 1 < n; ++i) {
            furthest = max(furthest, i + nums[i]);
            if (i == layerEnd) { // must jump again to go past this layer
                ++jumps;
                layerEnd = furthest;
            }
        }
        return jumps;
    }
};
