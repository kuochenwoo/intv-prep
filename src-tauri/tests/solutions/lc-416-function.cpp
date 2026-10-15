class Solution {
public:
    bool canPartition(vector<int>& nums) {
        int total = 0;
        for (int x : nums) total += x;
        if (total % 2) return false;
        // Bit s is set when some subset sums to s (max total 20000).
        bitset<20001> reach;
        reach[0] = 1;
        for (int x : nums) reach |= reach << x;
        return reach[total / 2];
    }
};
