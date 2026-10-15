#include <array>
#include <iostream>
#include <limits>
#include <map>
#include <string>
#include <vector>
using namespace std;

// Letter-count signature as an ordered-map key, independent of the sorted-key Function solution.
int main() {
    size_t n;
    cin >> n;
    cin.ignore(numeric_limits<streamsize>::max(), '\n');
    map<array<int, 26>, vector<string>> groups;
    for (size_t i = 0; i < n; ++i) {
        string s;
        getline(cin, s);
        if (!s.empty() && s.back() == '\r') s.pop_back();
        array<int, 26> count{};
        for (char c : s) ++count[c - 'a'];
        groups[count].push_back(s);
    }
    string out = to_string(groups.size()) + "\n";
    for (const auto& entry : groups) {
        out += to_string(entry.second.size()) + "\n";
        for (const string& s : entry.second) out += s + "\n";
    }
    cout << out;
}
