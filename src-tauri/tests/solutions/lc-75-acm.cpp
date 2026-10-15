#include <iostream>
#include <vector>
using namespace std;

// Counting pass, then rewrite.
int main() {
    int n;
    cin >> n;
    vector<int> a(n);
    int count[3] = {0, 0, 0};
    for (auto& x : a) { cin >> x; ++count[x]; }
    int k = 0;
    for (int c = 0; c < 3; ++c)
        while (count[c]--) a[k++] = c;
    for (int i = 0; i < n; ++i) cout << a[i] << (i + 1 < n ? ' ' : '\n');
}
