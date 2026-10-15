#include <iostream>
#include <string>
#include <vector>
using namespace std;

// Difference array on a doubled coordinate grid (so [1,2] and [3,4] stay apart while
// [1,4] and [4,5] join), then read off covered runs. O(n + V).
int main() {
    const int V = 2 * 10000 + 2;
    int n;
    cin >> n;
    vector<int> diff(V + 1, 0);
    for (int i = 0; i < n; ++i) {
        int k, s, e;
        cin >> k >> s >> e;
        ++diff[2 * s];
        --diff[2 * e + 1];
    }
    vector<pair<int, int>> runs;
    int depth = 0, start = -1;
    for (int x = 0; x <= V; ++x) {
        depth += diff[x];
        if (depth > 0 && start < 0) start = x;
        if (depth == 0 && start >= 0) {
            runs.push_back({start / 2, (x - 1) / 2});
            start = -1;
        }
    }
    string out = to_string(runs.size()) + "\n";
    for (auto& r : runs) out += to_string(r.first) + " " + to_string(r.second) + "\n";
    cout << out;
}
