class Solution {
public:
    vector<vector<int>> merge(vector<vector<int>>& intervals) {
        sort(intervals.begin(), intervals.end());
        vector<vector<int>> merged;
        for (const auto& iv : intervals) {
            if (!merged.empty() && iv[0] <= merged.back()[1])
                merged.back()[1] = max(merged.back()[1], iv[1]);
            else
                merged.push_back(iv);
        }
        return merged;
    }
};
