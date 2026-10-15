class Solution {
public:
    int subarraySum(vector<int>& nums, int k) {
        unordered_map<int, int> seen{{0, 1}};
        int prefix = 0, count = 0;
        for (int x : nums) {
            prefix += x;
            auto it = seen.find(prefix - k);
            if (it != seen.end()) count += it->second;
            ++seen[prefix];
        }
        return count;
    }
};
