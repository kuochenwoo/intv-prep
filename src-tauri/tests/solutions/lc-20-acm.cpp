#include <iostream>
#include <string>
using namespace std;

// Reuse the string itself as the stack (write pointer).
int main() {
    string s;
    getline(cin, s);
    if (!s.empty() && s.back() == '\r') s.pop_back();
    size_t top = 0;
    bool ok = true;
    for (char c : s) {
        if (c == '(' || c == '[' || c == '{') { s[top++] = c; continue; }
        char open = c == ')' ? '(' : c == ']' ? '[' : '{';
        if (top == 0 || s[top - 1] != open) { ok = false; break; }
        --top;
    }
    cout << ((ok && top == 0) ? "true" : "false") << '\n';
}
