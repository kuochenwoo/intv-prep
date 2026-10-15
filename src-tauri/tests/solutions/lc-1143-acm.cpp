#include <algorithm>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

static string read_text() {
    string s;
    getline(cin, s);
    if (!s.empty() && s.back() == '\r') s.pop_back();
    return s;
}

// Single rolling row; `diag` carries dp[i-1][j-1].
int main() {
    string a = read_text(), b = read_text();
    vector<int> row(b.size() + 1, 0);
    for (char c : a) {
        int diag = 0;
        for (size_t j = 1; j <= b.size(); ++j) {
            int up = row[j];
            row[j] = c == b[j - 1] ? diag + 1 : max(row[j], row[j - 1]);
            diag = up;
        }
    }
    cout << row.back() << '\n';
}
