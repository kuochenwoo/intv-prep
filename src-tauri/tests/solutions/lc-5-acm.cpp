#include <algorithm>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

// Manacher's algorithm on the '#'-interleaved string, O(n).
int main() {
    string s;
    getline(cin, s);
    if (!s.empty() && s.back() == '\r') s.pop_back();
    string t = "^#";
    for (char c : s) { t += c; t += '#'; }
    t += '$';
    int m = t.size(), center = 0, right = 0, bestLen = 0, bestCenter = 0;
    vector<int> p(m, 0);
    for (int i = 1; i < m - 1; ++i) {
        if (i < right) p[i] = min(right - i, p[2 * center - i]);
        while (t[i + p[i] + 1] == t[i - p[i] - 1]) ++p[i];
        if (i + p[i] > right) { center = i; right = i + p[i]; }
        if (p[i] > bestLen) { bestLen = p[i]; bestCenter = i; }
    }
    cout << s.substr((bestCenter - bestLen) / 2, bestLen) << '\n';
}
