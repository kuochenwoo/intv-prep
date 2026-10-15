#include <iostream>
#include <vector>
using namespace std;

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int x) : val(x), next(nullptr) {}
};

// Collect node pointers, then relink each full group in reverse order.
int main() {
    int n, k;
    cin >> n;
    vector<ListNode*> nodes(n);
    for (auto& p : nodes) {
        int v;
        cin >> v;
        p = new ListNode(v);
    }
    cin >> k;
    vector<ListNode*> order;
    for (int i = 0; i < n; i += k) {
        if (i + k <= n)
            for (int j = i + k - 1; j >= i; --j) order.push_back(nodes[j]);
        else
            for (int j = i; j < n; ++j) order.push_back(nodes[j]);
    }
    for (size_t i = 0; i < order.size(); ++i) order[i]->next = i + 1 < order.size() ? order[i + 1] : nullptr;
    bool first = true;
    for (ListNode* p = order.empty() ? nullptr : order[0]; p; p = p->next) {
        cout << (first ? "" : " ") << p->val;
        first = false;
    }
    cout << '\n';
}
