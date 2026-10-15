#include <iostream>
#include <string>
#include <vector>
using namespace std;

// Fenwick tree over the bounded value range; the median is found by rank (k-th smallest).
const int OFFSET = 100000, SIZE = 2 * OFFSET + 1;
vector<int> tree(SIZE + 1, 0);
int LOG = 1;

void add(int i) {
    for (++i; i <= SIZE; i += i & -i) ++tree[i];
}

int kth(int k) { // value index of the k-th smallest (1-based k)
    int pos = 0;
    for (int step = LOG; step; step >>= 1)
        if (pos + step <= SIZE && tree[pos + step] < k) {
            pos += step;
            k -= tree[pos];
        }
    return pos; // 0-based index
}

int main() {
    while (LOG * 2 <= SIZE) LOG *= 2;
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
    int count = 0;
    cout << m << '\n';
    for (size_t i = 0; i < m; ++i) {
        if (ops[i] == "MedianFinder") {
            tree.assign(SIZE + 1, 0);
            count = 0;
            cout << "null\n";
        } else if (ops[i] == "addNum") {
            add(args[i][0] + OFFSET);
            ++count;
            cout << "null\n";
        } else {
            long long a = kth((count + 1) / 2) - OFFSET, b = kth(count / 2 + 1) - OFFSET;
            long long sum = a + b; // exact: print sum / 2 without floating point
            if (sum % 2 == 0) cout << sum / 2 << '\n';
            else cout << (sum < 0 ? "-" : "") << (sum < 0 ? -sum : sum) / 2 << ".5\n";
        }
    }
}
