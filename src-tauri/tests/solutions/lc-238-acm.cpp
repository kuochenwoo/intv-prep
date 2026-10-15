#include <iostream>
#include <string>
#include <vector>
using namespace std;

// Separate prefix and suffix product arrays (O(n) extra space), no division.
int main() {
    int n;
    cin >> n;
    vector<long long> a(n), prefix(n + 1, 1), suffix(n + 1, 1);
    for (auto& x : a) cin >> x;
    for (int i = 0; i < n; ++i) prefix[i + 1] = prefix[i] * a[i];
    for (int i = n - 1; i >= 0; --i) suffix[i] = suffix[i + 1] * a[i];
    string out;
    for (int i = 0; i < n; ++i) {
        if (i) out += ' ';
        out += to_string(prefix[i] * suffix[i + 1]);
    }
    cout << out << '\n';
}
