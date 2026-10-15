#include <iostream>
#include <unordered_map>
using namespace std;
int main() {
    int n, target;
    cin >> n >> target;
    unordered_map<int, int> seen;
    for (int i = 0; i < n; ++i) {
        int value;
        cin >> value;
        if (seen.count(target - value)) {
            cout << seen[target - value] << ' ' << i << '\n';
            return 0;
        }
        seen[value] = i;
    }
}
