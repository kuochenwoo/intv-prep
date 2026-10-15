#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

// std::equal_range.
int main() {
    int n;
    cin >> n;
    vector<int> a(n);
    for (auto& x : a) cin >> x;
    int target;
    cin >> target;
    auto range = equal_range(a.begin(), a.end(), target);
    if (range.first == range.second) cout << "-1 -1\n";
    else cout << (range.first - a.begin()) << ' ' << (range.second - a.begin() - 1) << '\n';
}
