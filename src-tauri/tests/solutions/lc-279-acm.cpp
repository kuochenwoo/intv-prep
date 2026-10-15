#include <iostream>
using namespace std;

// Number theory: the answer is 1..4 (Lagrange); it is 4 exactly when n = 4^a (8b + 7) (Legendre).
bool isSquare(int x) {
    int r = 0;
    while ((r + 1) * (r + 1) <= x) ++r;
    return r * r == x;
}

int main() {
    int n;
    cin >> n;
    if (isSquare(n)) { cout << 1 << '\n'; return 0; }
    for (int a = 1; a * a < n; ++a)
        if (isSquare(n - a * a)) { cout << 2 << '\n'; return 0; }
    int m = n;
    while (m % 4 == 0) m /= 4;
    cout << (m % 8 == 7 ? 4 : 3) << '\n';
}
