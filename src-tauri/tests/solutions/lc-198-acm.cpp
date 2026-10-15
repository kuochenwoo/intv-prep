#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

// best[i] = best total when house i is the last robbed: nums[i] + max(best[i-2], best[i-3]).
int main() {
    int n;
    cin >> n;
    vector<int> nums(n);
    for (auto& x : nums) cin >> x;
    vector<int> best(n, 0);
    int answer = 0;
    for (int i = 0; i < n; ++i) {
        int before = 0;
        if (i >= 2) before = max(before, best[i - 2]);
        if (i >= 3) before = max(before, best[i - 3]);
        best[i] = nums[i] + before;
        answer = max(answer, best[i]);
    }
    cout << answer << '\n';
}
