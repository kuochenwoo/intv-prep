class Solution {
public:
    vector<int> topKFrequent(vector<int>& nums, int k) {
        // Hash count, then bucket values by frequency and read buckets from the top.
        unordered_map<int, int> freq;
        for (int x : nums) ++freq[x];
        vector<vector<int>> buckets(nums.size() + 1);
        for (auto& [value, count] : freq) buckets[count].push_back(value);
        vector<int> result;
        for (int c = (int)nums.size(); c >= 1 && (int)result.size() < k; --c)
            for (int value : buckets[c]) result.push_back(value);
        result.resize(k);
        return result;
    }
};
