#include <iostream>
#include <string>
#include <vector>
using namespace std;

// Binary search on the window length; each length is checked by a fixed-size sliding window
// over a count of how many letter kinds are satisfied. O(m log m), independent of the
// variable-window Function solution.
int main() {
    string s, t;
    getline(cin, s);
    getline(cin, t);
    if (!s.empty() && s.back() == '\r') s.pop_back();
    if (!t.empty() && t.back() == '\r') t.pop_back();
    int need[128] = {0}, kinds = 0;
    for (char c : t) if (need[(unsigned char)c]++ == 0) ++kinds;
    int m = (int)s.size();
    auto find = [&](int len) {
        // Returns the first start of a covering window of length len, or -1.
        int have[128] = {0}, satisfied = 0;
        for (int i = 0; i < m; ++i) {
            unsigned char in = s[i];
            if (need[in] && ++have[in] == need[in]) ++satisfied;
            if (i >= len) {
                unsigned char out = s[i - len];
                if (need[out] && have[out]-- == need[out]) --satisfied;
            }
            if (i >= len - 1 && satisfied == kinds) return i - len + 1;
        }
        return -1;
    };
    int lo = (int)t.size(), hi = m, best = -1;
    while (lo <= hi) {
        int mid = lo + (hi - lo) / 2;
        int start = find(mid);
        if (start >= 0) { best = mid; hi = mid - 1; }
        else lo = mid + 1;
    }
    cout << (best < 0 ? string() : s.substr(find(best), best)) << '\n';
}
