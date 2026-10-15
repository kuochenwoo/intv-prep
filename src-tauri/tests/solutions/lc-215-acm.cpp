#include <iostream>
#include <vector>
using namespace std;

// Counting over the bounded value range [-10000, 10000], scanning from the top.
int main() {
    int n;
    cin >> n;
    const int OFFSET = 10000;
    vector<int> count(2 * OFFSET + 1, 0);
    for (int i = 0; i < n; ++i) {
        int x;
        cin >> x;
        ++count[x + OFFSET];
    }
    int k;
    cin >> k;
    for (int v = 2 * OFFSET; v >= 0; --v) {
        k -= count[v];
        if (k <= 0) {
            cout << v - OFFSET << '\n';
            return 0;
        }
    }
}
