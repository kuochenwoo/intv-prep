class Solution {
public:
    bool searchMatrix(vector<vector<int>>& matrix, int target) {
        // Staircase search from the top-right corner, O(m + n).
        int i = 0, j = (int)matrix[0].size() - 1, m = matrix.size();
        while (i < m && j >= 0) {
            if (matrix[i][j] == target) return true;
            if (matrix[i][j] > target) --j;
            else ++i;
        }
        return false;
    }
};
