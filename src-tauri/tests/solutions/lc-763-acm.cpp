#include <algorithm>
#include <iostream>
#include <string>
#include <utility>
#include <vector>
using namespace std;

// Merge the [first, last] occurrence interval of every letter; merged blocks are the parts.
int main() {
    string s;
    getline(cin, s);
    vector<int> first(26, -1), last(26, -1);
    for (int i = 0; i < (int)s.size(); ++i) {
        int c = s[i] - 'a';
        if (first[c] < 0) first[c] = i;
        last[c] = i;
    }
    vector<pair<int, int>> spans;
    for (int c = 0; c < 26; ++c)
        if (first[c] >= 0) spans.push_back({first[c], last[c]});
    sort(spans.begin(), spans.end());
    vector<int> sizes;
    int lo = spans[0].first, hi = spans[0].second;
    for (size_t i = 1; i < spans.size(); ++i) {
        if (spans[i].first > hi) {
            sizes.push_back(hi - lo + 1);
            lo = spans[i].first;
        }
        hi = max(hi, spans[i].second);
    }
    sizes.push_back(hi - lo + 1);
    for (size_t i = 0; i < sizes.size(); ++i) cout << (i ? " " : "") << sizes[i];
    cout << '\n';
}
