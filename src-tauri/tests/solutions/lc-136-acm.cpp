#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

// Sort and scan pairs: O(n log n) cross-check, independent of the XOR solution.
int main() {
    int n;
    cin >> n;
    vector<int> a(n);
    for (auto& x : a) cin >> x;
    sort(a.begin(), a.end());
    int i = 0;
    while (i + 1 < n && a[i] == a[i + 1]) i += 2;
    cout << a[i] << '\n';
}
