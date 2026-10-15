#include <iostream>
#include <unordered_map>
#include <vector>
using namespace std;

// Boundary-length map: each new value merges the runs ending at x-1 and starting at x+1.
int main() {
    int n;
    cin >> n;
    unordered_map<long long, int> len;
    len.reserve(n * 2 + 1);
    int best = 0;
    for (int i = 0; i < n; ++i) {
        long long x;
        cin >> x;
        if (len.count(x)) continue;
        int left = len.count(x - 1) ? len[x - 1] : 0;
        int right = len.count(x + 1) ? len[x + 1] : 0;
        int total = left + right + 1;
        len[x] = total;
        len[x - left] = total;
        len[x + right] = total;
        if (total > best) best = total;
    }
    cout << best << '\n';
}
