#include <iostream>
#include <vector>
using namespace std;
int main() {
    int n;
    cin >> n;
    vector<int> values(n);
    for (auto& value : values) cin >> value;
    for (int i = n - 1; i >= 0; --i) cout << values[i] << ' ';
    cout << '\n';
}
