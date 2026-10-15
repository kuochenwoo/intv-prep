#include <iostream>
#include <string>
#include <vector>
using namespace std;

// Bitmask backtracking: free squares of the current row = ~(cols | left diag | right diag).
int n;
vector<int> pos;
vector<vector<int>> found;

void go(int row, unsigned cols, unsigned ld, unsigned rd) {
    if (row == n) {
        found.push_back(pos);
        return;
    }
    unsigned full = (1u << n) - 1, avail = full & ~(cols | ld | rd);
    while (avail) {
        unsigned bit = avail & -avail;
        avail -= bit;
        pos[row] = __builtin_ctz(bit);
        go(row + 1, cols | bit, ((ld | bit) << 1) & full, (rd | bit) >> 1);
    }
}

int main() {
    cin >> n;
    pos.assign(n, 0);
    go(0, 0, 0, 0);
    cout << found.size() << '\n';
    for (auto& p : found) {
        cout << n << '\n';
        for (int c : p) cout << string(c, '.') << 'Q' << string(n - c - 1, '.') << '\n';
    }
}
