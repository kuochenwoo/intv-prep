#include <iostream>
#include <string>
#include <vector>
using namespace std;

// Iterative breadth-wise expansion, independent of the recursive Function-mode solution.
int main() {
    string digits;
    getline(cin, digits);
    while (!digits.empty() && (digits.back() == '\r' || digits.back() == ' ')) digits.pop_back();
    const string keys[10] = {"", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"};
    vector<string> out;
    if (!digits.empty()) {
        out.push_back("");
        for (char d : digits) {
            vector<string> next;
            for (auto& s : out)
                for (char c : keys[d - '0']) next.push_back(s + c);
            out.swap(next);
        }
    }
    cout << out.size() << '\n';
    for (auto& s : out) cout << s << '\n';
}
