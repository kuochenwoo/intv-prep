#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

// O(n^2) DP, independent of the binary-search Function-mode solution.
int main() {
    int n;
    cin >> n;
    vector<int> a(n);
    for (auto& x : a) cin >> x;
    vector<int> best(n, 1);
    int answer = 0;
    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < i; ++j)
            if (a[j] < a[i]) best[i] = max(best[i], best[j] + 1);
        answer = max(answer, best[i]);
    }
    cout << answer << '\n';
}
