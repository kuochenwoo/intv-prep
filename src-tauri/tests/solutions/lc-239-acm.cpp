#include <algorithm>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

// Block decomposition: split into blocks of size k, take suffix max of the left block
// and prefix max of the right block for each window.
int main() {
    int n, k;
    cin >> n >> k;
    vector<int> a(n), prefix(n), suffix(n);
    for (auto& x : a) cin >> x;
    for (int i = 0; i < n; ++i) prefix[i] = (i % k == 0) ? a[i] : max(prefix[i - 1], a[i]);
    for (int i = n - 1; i >= 0; --i)
        suffix[i] = (i == n - 1 || (i + 1) % k == 0) ? a[i] : max(suffix[i + 1], a[i]);
    string out;
    for (int i = 0; i + k <= n; ++i) {
        if (i) out += ' ';
        out += to_string(max(suffix[i], prefix[i + k - 1]));
    }
    cout << out << '\n';
}
