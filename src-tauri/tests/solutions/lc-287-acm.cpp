#include <iostream>
#include <vector>
using namespace std;

// Binary search on the value: count(x <= mid) > mid means the duplicate is <= mid.
int main() {
    int len;
    cin >> len;
    vector<int> a(len);
    for (auto& x : a) cin >> x;
    int lo = 1, hi = len - 1;
    while (lo < hi) {
        int mid = lo + (hi - lo) / 2, count = 0;
        for (int x : a) count += x <= mid;
        if (count > mid) hi = mid;
        else lo = mid + 1;
    }
    cout << lo << '\n';
}
