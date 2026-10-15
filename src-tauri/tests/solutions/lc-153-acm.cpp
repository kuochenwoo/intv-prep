#include <iostream>
#include <vector>
using namespace std;

// Binary search for the first element smaller than nums[0].
int main() {
    int n;
    cin >> n;
    vector<int> a(n);
    for (auto& x : a) cin >> x;
    if (a[0] <= a[n - 1]) { cout << a[0] << '\n'; return 0; }
    int lo = 1, hi = n - 1;
    while (lo < hi) {
        int mid = (lo + hi) / 2;
        if (a[mid] < a[0]) hi = mid;
        else lo = mid + 1;
    }
    cout << a[lo] << '\n';
}
