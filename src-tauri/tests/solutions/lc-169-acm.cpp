#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

// The majority element always occupies the median position.
int main() {
    int n;
    cin >> n;
    vector<int> a(n);
    for (auto& x : a) cin >> x;
    nth_element(a.begin(), a.begin() + n / 2, a.end());
    cout << a[n / 2] << '\n';
}
