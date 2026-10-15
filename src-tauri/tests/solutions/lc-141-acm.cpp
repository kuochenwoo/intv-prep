#include <iostream>
#include <unordered_set>
#include <vector>
using namespace std;

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int x) : val(x), next(nullptr) {}
};

// Builds the (possibly cyclic) list and detects a revisit with a hash set.
int main() {
    int n, pos;
    cin >> n;
    vector<ListNode*> nodes(n);
    for (int i = 0; i < n; ++i) {
        int v;
        cin >> v;
        nodes[i] = new ListNode(v);
        if (i) nodes[i - 1]->next = nodes[i];
    }
    cin >> pos;
    if (pos >= 0) nodes[n - 1]->next = nodes[pos];
    unordered_set<ListNode*> seen;
    bool cycle = false;
    for (ListNode* p = n ? nodes[0] : nullptr; p; p = p->next) {
        if (!seen.insert(p).second) { cycle = true; break; }
    }
    cout << (cycle ? "true" : "false") << '\n';
}
