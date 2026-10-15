#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

// Closed-interval binary search returning the insertion point.
int main() {
    int n;
    cin >> n;
    vector<int> a(n);
    for (auto& x : a) cin >> x;
    int target;
    cin >> target;
    int lo = 0, hi = n - 1;
    while (lo <= hi) {
        int mid = (lo + hi) / 2;
        if (a[mid] == target) { cout << mid << '\n'; return 0; }
        if (a[mid] < target) lo = mid + 1;
        else hi = mid - 1;
    }
    cout << lo << '\n';
}
