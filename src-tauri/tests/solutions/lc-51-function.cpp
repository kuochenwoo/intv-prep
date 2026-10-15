class Solution {
    // Row-by-row backtracking with column and diagonal flags.
    void go(int r, int n, vector<string>& board, vector<bool>& col, vector<bool>& d1, vector<bool>& d2,
            vector<vector<string>>& out) {
        if (r == n) {
            out.push_back(board);
            return;
        }
        for (int c = 0; c < n; ++c) {
            if (col[c] || d1[r + c] || d2[r - c + n - 1]) continue;
            col[c] = d1[r + c] = d2[r - c + n - 1] = true;
            board[r][c] = 'Q';
            go(r + 1, n, board, col, d1, d2, out);
            board[r][c] = '.';
            col[c] = d1[r + c] = d2[r - c + n - 1] = false;
        }
    }

public:
    vector<vector<string>> solveNQueens(int n) {
        vector<vector<string>> out;
        vector<string> board(n, string(n, '.'));
        vector<bool> col(n), d1(2 * n - 1), d2(2 * n - 1);
        go(0, n, board, col, d1, d2, out);
        return out;
    }
};
