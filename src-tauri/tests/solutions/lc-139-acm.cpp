#include <array>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

// Trie + forward propagation: from every reachable start, walk the trie along s.
int main() {
    string s;
    getline(cin, s);
    int k;
    cin >> k;
    vector<array<int, 26>> next(1);
    vector<bool> terminal(1, false);
    next[0].fill(-1);
    for (int i = 0; i < k; ++i) {
        string w;
        cin >> w;
        int node = 0;
        for (char ch : w) {
            int c = ch - 'a';
            if (next[node][c] < 0) {
                next[node][c] = next.size();
                next.emplace_back();
                next.back().fill(-1);
                terminal.push_back(false);
            }
            node = next[node][c];
        }
        terminal[node] = true;
    }
    int n = s.size();
    vector<bool> reach(n + 1, false);
    reach[0] = true;
    for (int start = 0; start < n; ++start) {
        if (!reach[start]) continue;
        int node = 0;
        for (int i = start; i < n; ++i) {
            node = next[node][s[i] - 'a'];
            if (node < 0) break;
            if (terminal[node]) reach[i + 1] = true;
        }
    }
    cout << (reach[n] ? "true" : "false") << '\n';
}
