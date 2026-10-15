#include <algorithm>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

// Backtracking with pruning: letter-count check, then search from the rarer end of the word.
int m, n;
vector<string> board;
vector<vector<bool>> used;
string word;

bool dfs(int i, int j, size_t k) {
    if (i < 0 || j < 0 || i >= m || j >= n || used[i][j] || board[i][j] != word[k]) return false;
    if (k + 1 == word.size()) return true;
    used[i][j] = true;
    bool found = dfs(i + 1, j, k + 1) || dfs(i - 1, j, k + 1) || dfs(i, j + 1, k + 1) || dfs(i, j - 1, k + 1);
    used[i][j] = false;
    return found;
}

int main() {
    cin >> m;
    board.resize(m);
    for (auto& row : board) cin >> row;
    cin >> word;
    n = board[0].size();
    vector<int> have(128, 0), need(128, 0);
    for (auto& row : board)
        for (char c : row) ++have[(unsigned char)c];
    for (char c : word) ++need[(unsigned char)c];
    for (int c = 0; c < 128; ++c)
        if (need[c] > have[c]) { cout << "false\n"; return 0; }
    if (have[(unsigned char)word.back()] < have[(unsigned char)word.front()]) reverse(word.begin(), word.end());
    used.assign(m, vector<bool>(n, false));
    bool found = false;
    for (int i = 0; i < m && !found; ++i)
        for (int j = 0; j < n && !found; ++j) found = dfs(i, j, 0);
    cout << (found ? "true" : "false") << '\n';
}
