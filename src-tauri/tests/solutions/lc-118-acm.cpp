#include <iostream>
#include <vector>
using namespace std;

// Multiplicative binomial formula per row: C(i, j+1) = C(i, j) * (i-j) / (j+1).
int main() {
    int n;
    cin >> n;
    cout << n << '\n';
    for (int i = 0; i < n; ++i) {
        long long c = 1;
        for (int j = 0; j <= i; ++j) {
            cout << (j ? " " : "") << c;
            c = c * (i - j) / (j + 1);
        }
        cout << '\n';
    }
}
