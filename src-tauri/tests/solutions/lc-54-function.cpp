class Solution {
public:
    vector<int> spiralOrder(vector<vector<int>>& matrix) {
        // Shrinking boundaries.
        vector<int> out;
        int top = 0, bottom = (int)matrix.size() - 1, left = 0, right = (int)matrix[0].size() - 1;
        while (top <= bottom && left <= right) {
            for (int j = left; j <= right; ++j) out.push_back(matrix[top][j]);
            for (int i = top + 1; i <= bottom; ++i) out.push_back(matrix[i][right]);
            if (top < bottom && left < right) {
                for (int j = right - 1; j >= left; --j) out.push_back(matrix[bottom][j]);
                for (int i = bottom - 1; i > top; --i) out.push_back(matrix[i][left]);
            }
            ++top; --bottom; ++left; --right;
        }
        return out;
    }
};
