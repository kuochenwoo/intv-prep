class Solution {
public:
    int rob(vector<int>& nums) {
        int take = 0, skip = 0; // best ending with / without robbing the current house
        for (int x : nums) {
            int nextTake = skip + x;
            skip = max(skip, take);
            take = nextTake;
        }
        return max(take, skip);
    }
};
