// Profile these two known implementations, not arbitrary submitted code.
// Operation and auxiliary-entry counts are deterministic. Timing is observational.
#include <chrono>
#include <iostream>
#include <unordered_map>
#include <vector>
using namespace std;
struct Stats {
    int first = -1, second = -1;
    long long operations = 0;
    size_t auxiliaryEntries = 0;
};
Stats brute(const vector<int>& nums, int target) {
    Stats result;
    for (int i = 0; i < (int)nums.size(); ++i) {
        for (int j = i + 1; j < (int)nums.size(); ++j) {
            ++result.operations; // One candidate pair checked.
            if (nums[i] + nums[j] == target) {
                result.first = i;
                result.second = j;
                return result;
            }
        }
    }
    return result;
}
Stats hashed(const vector<int>& nums, int target) {
    Stats result;
    unordered_map<int, int> seen;
    for (int i = 0; i < (int)nums.size(); ++i) {
        ++result.operations; // One hash lookup (not the number of bucket probes).
        auto found = seen.find(target - nums[i]);
        if (found != seen.end()) {
            result.first = found->second;
            result.second = i;
            return result;
        }
        seen[nums[i]] = i;
        result.auxiliaryEntries = seen.size(); // Unique keys, monotonically increasing.
    }
    return result;
}
int main() {
    cout << "algorithm,n,operations,auxiliary_entries,algorithm_ns\n";
    for (int n : {1000, 2000, 4000, 10000}) {
        vector<int> nums(n);
        for (int i = 0; i < n - 2; ++i) nums[i] = i;
        nums[n - 2] = 100000;
        nums[n - 1] = 200000; // Unique valid pair is the last pair visited.
        for (bool useHash : {false, true}) {
            auto start = chrono::steady_clock::now();
            auto result = useHash ? hashed(nums, 300000) : brute(nums, 300000);
            auto ns = chrono::duration_cast<chrono::nanoseconds>(
                chrono::steady_clock::now() - start).count();
            if (result.first != n - 2 || result.second != n - 1) return 1;
            cout << (useHash ? "hash" : "double_for") << ',' << n << ','
                 << result.operations << ',' << result.auxiliaryEntries << ',' << ns << '\n';
        }
    }
}
