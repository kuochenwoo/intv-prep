#include <iostream>
#include <vector>
using namespace std;
int main() {
    int n, target;
    cin >> n >> target;
    vector<int> nums(n);
    for (auto& value : nums) cin >> value;
    for (int i = 0; i < n; ++i) {
        for (int j = i + 1; j < n; ++j) {
            if (nums[i] + nums[j] == target) {
                cout << i << ' ' << j << '\n';
                return 0;
            }
        }
    }
}
