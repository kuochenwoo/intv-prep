class Solution {
public:
    int singleNumber(vector<int>& nums) {
        // Pairs cancel under XOR.
        int x = 0;
        for (int v : nums) x ^= v;
        return x;
    }
};
