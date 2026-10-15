class Solution {
public:
    vector<int> productExceptSelf(vector<int>& nums) {
        // Prefix products in the output, then a rolling suffix product: O(1) extra space.
        int n = (int)nums.size();
        vector<int> answer(n, 1);
        for (int i = 1; i < n; ++i) answer[i] = answer[i - 1] * nums[i - 1];
        int suffix = 1;
        for (int i = n - 1; i >= 0; --i) {
            answer[i] *= suffix;
            suffix *= nums[i];
        }
        return answer;
    }
};
