#include <algorithm>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

// Count table over the value range: pick distinct a <= b, then c = -a-b must be >= b
// with enough copies. Independent of the sort + two-pointer Function solution.
int main() {
    const int OFFSET = 100000;
    int n;
    cin >> n;
    vector<int> count(2 * OFFSET + 1, 0);
    for (int i = 0; i < n; ++i) {
        int x;
        cin >> x;
        ++count[x + OFFSET];
    }
    vector<int> values;
    for (int v = -OFFSET; v <= OFFSET; ++v)
        if (count[v + OFFSET]) values.push_back(v);
    vector<string> lines;
    for (size_t i = 0; i < values.size(); ++i) {
        int a = values[i];
        if (a > 0) break;
        for (size_t j = i; j < values.size(); ++j) {
            int b = values[j];
            int c = -a - b;
            if (c < b) break;
            if (c > OFFSET) continue;
            if (!count[c + OFFSET]) continue;
            int need_a = 1 + (b == a) + (c == a);
            int need_b = (b == a) ? need_a : 1 + (c == b);
            if (count[a + OFFSET] < need_a || count[b + OFFSET] < need_b) continue;
            if (c != b && c != a && count[c + OFFSET] < 1) continue;
            lines.push_back(to_string(a) + " " + to_string(b) + " " + to_string(c));
        }
    }
    string out = to_string(lines.size()) + "\n";
    for (const string& line : lines) out += line + "\n";
    cout << out;
}
