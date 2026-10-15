#include <algorithm>
#include <iostream>
using namespace std;

// Closed form C(m + n - 2, k) built incrementally; each prefix product is an exact binomial.
int main() {
    long long m, n;
    cin >> m >> n;
    long long total = m + n - 2, k = min(m, n) - 1, value = 1;
    for (long long i = 1; i <= k; ++i) value = value * (total - k + i) / i;
    cout << value << '\n';
}
