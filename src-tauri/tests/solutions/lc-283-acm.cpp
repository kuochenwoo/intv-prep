#include <iostream>
#include <vector>
using namespace std;
int main() {
    int n, zeroes = 0;
    cin >> n;
    for (int i = 0; i < n; ++i) {
        int value;
        cin >> value;
        if (value) cout << value << ' ';
        else ++zeroes;
    }
    while (zeroes--) cout << "0 ";
    cout << '\n';
}
