#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

// Backward DP: best[i] = 1 + min(best[i+1 .. i+nums[i]]), O(n * max(nums)).
int main() {
    int n;
    cin >> n;
    vector<int> nums(n);
    for (auto& x : nums) cin >> x;
    const int INF = 1 << 29;
    vector<int> best(n, INF);
    best[n - 1] = 0;
    for (int i = n - 2; i >= 0; --i) {
        int end = min(n - 1, i + nums[i]);
        for (int j = i + 1; j <= end; ++j) best[i] = min(best[i], best[j] + 1);
    }
    cout << best[0] << '\n';
}
