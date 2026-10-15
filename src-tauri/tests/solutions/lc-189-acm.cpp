#include <iostream>
#include <string>
#include <vector>
using namespace std;

// Cyclic replacement: follow each cycle of i -> (i + k) mod n, gcd(n, k) cycles in total.
int main() {
    int n;
    long long k;
    cin >> n >> k;
    vector<int> a(n);
    for (auto& x : a) cin >> x;
    int shift = (int)(k % n), moved = 0;
    for (int start = 0; moved < n; ++start) {
        int i = start, carry = a[start];
        do {
            int j = (i + shift) % n;
            int next = a[j];
            a[j] = carry;
            carry = next;
            i = j;
            ++moved;
        } while (i != start);
    }
    string out;
    for (int i = 0; i < n; ++i) {
        if (i) out += ' ';
        out += to_string(a[i]);
    }
    cout << out << '\n';
}
