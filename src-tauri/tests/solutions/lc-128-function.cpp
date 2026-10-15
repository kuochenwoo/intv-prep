class Solution {
public:
    int longestConsecutive(vector<int>& nums) {
        // Count forward only from run starts (x-1 absent).
        unordered_set<int> seen(nums.begin(), nums.end());
        int best = 0;
        for (int x : seen) {
            if (x != INT_MIN && seen.count(x - 1)) continue;
            int length = 1;
            while (x + length - 1 != INT_MAX && seen.count(x + length)) ++length;
            best = max(best, length);
        }
        return best;
    }
};
