#include <iostream>
#include <vector>
using namespace std;

// Copy values to an array and compare from both ends.
int main() {
    int n;
    cin >> n;
    vector<int> v(n);
    for (auto& x : v) cin >> x;
    bool same = true;
    for (int i = 0, j = n - 1; i < j; ++i, --j)
        if (v[i] != v[j]) same = false;
    cout << (same ? "true" : "false") << '\n';
}
