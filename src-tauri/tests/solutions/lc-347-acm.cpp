#include <functional>
#include <iostream>
#include <queue>
#include <utility>
#include <vector>
using namespace std;

// Counting array over the value range, then a size-k min-heap keyed by frequency.
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
    priority_queue<pair<int, int>, vector<pair<int, int>>, greater<pair<int, int>>> heap;
    for (int v = 0; v <= 2 * OFFSET; ++v) {
        if (!count[v]) continue;
        heap.push({count[v], v - OFFSET});
        if ((int)heap.size() > k) heap.pop();
    }
    bool first = true;
    while (!heap.empty()) {
        cout << (first ? "" : " ") << heap.top().second;
        first = false;
        heap.pop();
    }
    cout << '\n';
}
