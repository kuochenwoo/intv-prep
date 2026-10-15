#include <algorithm>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

static string read_word() {
    string s;
    getline(cin, s);
    if (!s.empty() && s.back() == '\r') s.pop_back();
    return s;
}

// Rolling one-row DP over prefixes of word2.
int main() {
    string a = read_word(), b = read_word();
    size_t n = b.size();
    vector<int> row(n + 1);
    for (size_t j = 0; j <= n; ++j) row[j] = j;
    for (size_t i = 1; i <= a.size(); ++i) {
        int diag = row[0];
        row[0] = i;
        for (size_t j = 1; j <= n; ++j) {
            int up = row[j];
            row[j] = a[i - 1] == b[j - 1] ? diag : 1 + min({diag, up, row[j - 1]});
            diag = up;
        }
    }
    cout << row[n] << '\n';
}
