#include <functional>
#include <iostream>
#include <vector>
using namespace std;

// Three-colour DFS cycle detection, independent of Kahn's algorithm.
int main() {
    int n, p;
    cin >> n >> p;
    vector<vector<int>> next(n);
    for (int i = 0; i < p; ++i) {
        int k, a, b;
        cin >> k >> a >> b;
        next[b].push_back(a);
    }
    vector<int> color(n, 0); // 0 unvisited, 1 on stack, 2 done
    function<bool(int)> acyclic = [&](int u) {
        color[u] = 1;
        for (int v : next[u]) {
            if (color[v] == 1) return false;
            if (color[v] == 0 && !acyclic(v)) return false;
        }
        color[u] = 2;
        return true;
    };
    bool ok = true;
    for (int i = 0; i < n && ok; ++i)
        if (color[i] == 0) ok = acyclic(i);
    cout << (ok ? "true" : "false") << '\n';
}
