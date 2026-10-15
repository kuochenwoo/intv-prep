class Solution {
public:
    vector<int> dailyTemperatures(vector<int>& temperatures) {
        // Monotonic stack of indices with non-increasing temperatures.
        int n = temperatures.size();
        vector<int> ans(n, 0);
        vector<int> st;
        for (int i = 0; i < n; ++i) {
            while (!st.empty() && temperatures[st.back()] < temperatures[i]) {
                ans[st.back()] = i - st.back();
                st.pop_back();
            }
            st.push_back(i);
        }
        return ans;
    }
};
