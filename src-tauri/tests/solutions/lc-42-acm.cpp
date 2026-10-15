#include <iostream>
#include <stack>
#include <vector>
using namespace std;

// Monotonic stack, independent of the two-pointer Function-mode solution.
int main() {
    int n;
    cin >> n;
    vector<long long> h(n);
    for (auto& x : h) cin >> x;
    stack<int> st;
    long long water = 0;
    for (int i = 0; i < n; ++i) {
        while (!st.empty() && h[st.top()] < h[i]) {
            int bottom = st.top();
            st.pop();
            if (st.empty()) break;
            long long width = i - st.top() - 1;
            water += width * (min(h[st.top()], h[i]) - h[bottom]);
        }
        st.push(i);
    }
    cout << water << '\n';
}
