#include <iostream>
#include <string>
#include <vector>
using namespace std;

// Array-backed doubly linked list keyed directly by key (0..10000).
int main() {
    size_t m;
    cin >> m;
    vector<string> ops(m);
    for (auto& op : ops) cin >> op;
    cin >> m;
    vector<vector<int>> args(m);
    for (auto& a : args) {
        size_t k;
        cin >> k;
        a.resize(k);
        for (auto& x : a) cin >> x;
    }
    const int K = 10001, HEAD = K, TAIL = K + 1;
    vector<int> prev(K + 2), next(K + 2), value(K, 0);
    vector<bool> present(K, false);
    int capacity = 0, size = 0;
    auto unlink = [&](int x) { next[prev[x]] = next[x]; prev[next[x]] = prev[x]; };
    auto push_front = [&](int x) { next[x] = next[HEAD]; prev[x] = HEAD; prev[next[HEAD]] = x; next[HEAD] = x; };
    cout << m << '\n';
    for (size_t i = 0; i < m; ++i) {
        if (ops[i] == "LRUCache") {
            capacity = args[i][0];
            size = 0;
            fill(present.begin(), present.end(), false);
            next[HEAD] = TAIL; prev[TAIL] = HEAD;
            cout << "null\n";
        } else if (ops[i] == "get") {
            int key = args[i][0];
            if (!present[key]) { cout << -1 << '\n'; continue; }
            unlink(key); push_front(key);
            cout << value[key] << '\n';
        } else {
            int key = args[i][0];
            value[key] = args[i][1];
            if (present[key]) { unlink(key); push_front(key); }
            else {
                present[key] = true; push_front(key);
                if (++size > capacity) { int old = prev[TAIL]; unlink(old); present[old] = false; --size; }
            }
            cout << "null\n";
        }
    }
}
