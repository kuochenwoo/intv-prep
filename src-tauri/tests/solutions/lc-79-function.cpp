class Solution {
    // Backtracking with in-place visited marking, after a letter-count feasibility check.
    bool dfs(vector<vector<char>>& b, const string& w, int i, int j, size_t k) {
        if (b[i][j] != w[k]) return false;
        if (k + 1 == w.size()) return true;
        char saved = b[i][j];
        b[i][j] = '#';
        const int dr[4] = {1, -1, 0, 0}, dc[4] = {0, 0, 1, -1};
        bool found = false;
        for (int d = 0; d < 4 && !found; ++d) {
            int a = i + dr[d], c = j + dc[d];
            if (a >= 0 && c >= 0 && a < (int)b.size() && c < (int)b[0].size())
                found = dfs(b, w, a, c, k + 1);
        }
        b[i][j] = saved;
        return found;
    }

public:
    bool exist(vector<vector<char>>& board, string word) {
        unordered_map<char, int> count;
        for (auto& row : board)
            for (char c : row) ++count[c];
        for (char c : word)
            if (--count[c] < 0) return false;
        for (int i = 0; i < (int)board.size(); ++i)
            for (int j = 0; j < (int)board[0].size(); ++j)
                if (dfs(board, word, i, j, 0)) return true;
        return false;
    }
};
