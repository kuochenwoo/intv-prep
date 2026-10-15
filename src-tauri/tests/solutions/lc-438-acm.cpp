#include <iostream>
#include <string>
#include <vector>
using namespace std;

// Variable-size window: grow right while the letter is still available, shrink from the left otherwise.
int main() {
    string s, p;
    getline(cin, s);
    getline(cin, p);
    if (!s.empty() && s.back() == '\r') s.pop_back();
    if (!p.empty() && p.back() == '\r') p.pop_back();
    int need[26] = {0};
    for (char c : p) ++need[c - 'a'];
    int m = (int)p.size(), left = 0;
    string out;
    for (int right = 0; right < (int)s.size(); ++right) {
        int c = s[right] - 'a';
        --need[c];
        while (need[c] < 0) ++need[s[left++] - 'a'];
        if (right - left + 1 == m) {
            if (!out.empty()) out += ' ';
            out += to_string(left);
        }
    }
    cout << out << '\n';
}
