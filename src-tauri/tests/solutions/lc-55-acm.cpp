#include <iostream>
#include <vector>
using namespace std;

// Backward greedy: track the leftmost index known to reach the end.
int main() {
    int n;
    cin >> n;
    vector<long long> nums(n);
    for (auto& x : nums) cin >> x;
    int good = n - 1;
    for (int i = n - 2; i >= 0; --i)
        if (i + nums[i] >= good) good = i;
    cout << (good == 0 ? "true" : "false") << '\n';
}
