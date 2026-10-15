#include <iostream>
using namespace std;

// Combinatorial count: choose k two-steps among n-k moves, summed over k.
int main() {
    int n;
    cin >> n;
    long long total = 0;
    for (int k = 0; 2 * k <= n; ++k) {
        int moves = n - k;
        long long c = 1; // C(moves, k), built incrementally and exactly
        for (int i = 0; i < k; ++i) c = c * (moves - i) / (i + 1);
        total += c;
    }
    cout << total << '\n';
}
