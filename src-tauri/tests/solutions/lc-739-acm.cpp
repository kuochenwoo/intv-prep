#include <iostream>
#include <vector>
using namespace std;

// Right-to-left: jump along already computed answers to find a warmer day.
int main() {
    int n;
    cin >> n;
    vector<int> t(n), ans(n, 0);
    for (auto& x : t) cin >> x;
    for (int i = n - 2; i >= 0; --i) {
        int j = i + 1;
        while (true) {
            if (t[j] > t[i]) { ans[i] = j - i; break; }
            if (ans[j] == 0) break; // nothing warmer than t[j] (>= t[i]) ahead
            j += ans[j];
        }
    }
    for (int i = 0; i < n; ++i) cout << (i ? " " : "") << ans[i];
    cout << '\n';
}
