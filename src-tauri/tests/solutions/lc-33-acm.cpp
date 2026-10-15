#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

// Find the rotation point first, then binary search the right segment.
int main() {
    int n;
    cin >> n;
    vector<int> a(n);
    for (auto& x : a) cin >> x;
    int target;
    cin >> target;
    int lo = 0, hi = n - 1;
    while (lo < hi) {
        int mid = (lo + hi) / 2;
        if (a[mid] > a[hi]) lo = mid + 1;
        else hi = mid;
    }
    int pivot = lo; // index of the minimum
    auto begin = a.begin(), end = a.end();
    if (target >= a[pivot] && target <= a[n - 1]) begin = a.begin() + pivot;
    else end = a.begin() + pivot;
    auto it = lower_bound(begin, end, target);
    cout << ((it != end && *it == target) ? int(it - a.begin()) : -1) << '\n';
}
