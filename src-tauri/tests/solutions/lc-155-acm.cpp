#include <iostream>
#include <string>
#include <vector>
using namespace std;

// Difference encoding: store val - min (as long long) and a single running minimum.
// A negative entry marks a push that lowered the minimum, so pop can restore the old one.
int main() {
    size_t m;
    cin >> m;
    vector<string> ops(m);
    for (auto& op : ops) cin >> op;
    cin >> m;
    vector<vector<long long>> args(m);
    for (auto& a : args) {
        size_t k;
        cin >> k;
        a.resize(k);
        for (auto& x : a) cin >> x;
    }
    vector<long long> diffs;
    long long minimum = 0;
    cout << m << '\n';
    for (size_t i = 0; i < m; ++i) {
        const string& op = ops[i];
        if (op == "MinStack") {
            diffs.clear();
            cout << "null\n";
        } else if (op == "push") {
            long long val = args[i][0];
            if (diffs.empty()) { diffs.push_back(0); minimum = val; }
            else {
                diffs.push_back(val - minimum);
                if (val < minimum) minimum = val;
            }
            cout << "null\n";
        } else if (op == "pop") {
            long long d = diffs.back();
            diffs.pop_back();
            if (d < 0) minimum -= d; // popped value was the minimum; previous min = val - d
            cout << "null\n";
        } else if (op == "top") {
            long long d = diffs.back();
            cout << (d < 0 ? minimum : minimum + d) << '\n';
        } else {
            cout << minimum << '\n';
        }
    }
}
