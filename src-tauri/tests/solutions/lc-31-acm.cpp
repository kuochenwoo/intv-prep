#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

// std::next_permutation wraps to the ascending order when no next one exists.
int main() {
    int n;
    cin >> n;
    vector<int> a(n);
    for (auto& x : a) cin >> x;
    next_permutation(a.begin(), a.end());
    for (int i = 0; i < n; ++i) cout << a[i] << (i + 1 < n ? ' ' : '\n');
}
