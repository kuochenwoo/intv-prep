#include <iostream>
#include <string>
#include <vector>
using namespace std;

// Memoised suffix DP: parts[i] lists every palindrome partition of s[i..], built right to left.
int main() {
    string s;
    getline(cin, s);
    while (!s.empty() && (s.back() == '\r' || s.back() == ' ')) s.pop_back();
    int n = s.size();
    auto is_pal = [&](int i, int j) {
        while (i < j)
            if (s[i++] != s[j--]) return false;
        return true;
    };
    vector<vector<vector<string>>> parts(n + 1);
    parts[n].push_back({});
    for (int i = n - 1; i >= 0; --i)
        for (int j = i; j < n; ++j) {
            if (!is_pal(i, j)) continue;
            string head = s.substr(i, j - i + 1);
            for (auto& rest : parts[j + 1]) {
                vector<string> p{head};
                p.insert(p.end(), rest.begin(), rest.end());
                parts[i].push_back(move(p));
            }
        }
    cout << parts[0].size() << '\n';
    for (auto& p : parts[0]) {
        cout << p.size() << '\n';
        for (auto& x : p) cout << x << '\n';
    }
}
