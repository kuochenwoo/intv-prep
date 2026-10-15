#include <iostream>
#include <string>
#include <unordered_set>
#include <algorithm>
using namespace std;
int main() {
    string text;
    getline(cin, text);
    unordered_set<char> window;
    int left = 0, best = 0;
    for (int right = 0; right < (int)text.size(); ++right) {
        while (window.count(text[right])) window.erase(text[left++]);
        window.insert(text[right]);
        best = max(best, right - left + 1);
    }
    cout << best << '\n';
}
