class Solution {
public:
    int largestRectangleArea(vector<int>& heights) {
        // Increasing stack with a sentinel bar of height 0 at the end.
        int n = heights.size(), best = 0;
        vector<int> st;
        for (int i = 0; i <= n; ++i) {
            int h = i < n ? heights[i] : 0;
            while (!st.empty() && heights[st.back()] >= h) {
                int height = heights[st.back()];
                st.pop_back();
                int left = st.empty() ? -1 : st.back();
                best = max(best, height * (i - left - 1));
            }
            st.push_back(i);
        }
        return best;
    }
};
