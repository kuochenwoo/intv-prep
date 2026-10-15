#include <iostream>
#include <vector>
using namespace std;

// Marker array over [1, n] (O(n) extra space), independent of the in-place swap solution.
int main() {
    int n;
    cin >> n;
    vector<char> seen(n + 2, 0);
    for (int i = 0; i < n; ++i) {
        long long x;
        cin >> x;
        if (x >= 1 && x <= n) seen[x] = 1;
    }
    int k = 1;
    while (seen[k]) ++k;
    cout << k << '\n';
}
