#include <iostream>
#include <string>
using namespace std;

// Recursive descent: parse a sequence until ']' or end of input.
string parse(const string& s, size_t& i) {
    string out;
    while (i < s.size() && s[i] != ']') {
        if (s[i] >= '0' && s[i] <= '9') {
            int k = 0;
            while (s[i] >= '0' && s[i] <= '9') k = k * 10 + (s[i++] - '0');
            ++i; // '['
            string inner = parse(s, i);
            ++i; // ']'
            while (k--) out += inner;
        } else {
            out += s[i++];
        }
    }
    return out;
}

int main() {
    string s;
    getline(cin, s);
    if (!s.empty() && s.back() == '\r') s.pop_back();
    size_t i = 0;
    cout << parse(s, i) << '\n';
}
