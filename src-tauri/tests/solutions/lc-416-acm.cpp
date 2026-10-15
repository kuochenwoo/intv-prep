#include <iostream>
#include <vector>
using namespace std;

// One-dimensional 0/1 knapsack over sums, iterated downward.
int main() {
    int n;
    cin >> n;
    vector<int> a(n);
    int total = 0;
    for (auto& x : a) { cin >> x; total += x; }
    bool ok = false;
    if (total % 2 == 0) {
        int half = total / 2;
        vector<char> can(half + 1, 0);
        can[0] = 1;
        for (int x : a)
            for (int s = half; s >= x; --s)
                if (can[s - x]) can[s] = 1;
        ok = can[half];
    }
    cout << (ok ? "true" : "false") << '\n';
}
