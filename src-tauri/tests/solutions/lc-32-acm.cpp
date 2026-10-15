#include <algorithm>
#include <iostream>
#include <string>
using namespace std;

// Two counting passes with O(1) extra space.
int main() {
    string s;
    getline(cin, s);
    if (!s.empty() && s.back() == '\r') s.pop_back();
    int n = (int)s.size(), best = 0, open = 0, close = 0;
    for (int i = 0; i < n; ++i) {
        (s[i] == '(' ? open : close)++;
        if (open == close) best = max(best, 2 * close);
        else if (close > open) open = close = 0;
    }
    open = close = 0;
    for (int i = n - 1; i >= 0; --i) {
        (s[i] == '(' ? open : close)++;
        if (open == close) best = max(best, 2 * open);
        else if (open > close) open = close = 0;
    }
    cout << best << '\n';
}
