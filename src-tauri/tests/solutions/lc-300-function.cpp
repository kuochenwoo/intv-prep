class Solution {
public:
    int lengthOfLIS(vector<int>& nums) {
        // tails[k] = smallest tail of an increasing subsequence of length k + 1.
        vector<int> tails;
        for (int x : nums) {
            auto it = lower_bound(tails.begin(), tails.end(), x);
            if (it == tails.end()) tails.push_back(x);
            else *it = x;
        }
        return (int)tails.size();
    }
};
