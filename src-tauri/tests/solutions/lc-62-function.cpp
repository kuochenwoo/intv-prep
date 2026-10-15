class Solution {
public:
    int uniquePaths(int m, int n) {
        // Rolling DP row: row[j] = paths to (i, j).
        vector<int> row(n, 1);
        for (int i = 1; i < m; ++i)
            for (int j = 1; j < n; ++j) row[j] += row[j - 1];
        return row[n - 1];
    }
};
