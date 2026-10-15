#include <iostream>
#include <unordered_map>
#include <vector>
using namespace std;

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int x) : val(x), next(nullptr) {}
};

// Builds the list, then returns the first node seen twice (hash map node -> index).
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
    unordered_map<ListNode*, int> index;
    for (int i = 0; i < n; ++i) index[nodes[i]] = i;
    unordered_map<ListNode*, bool> seen;
    int answer = -1;
    for (ListNode* p = n ? nodes[0] : nullptr; p; p = p->next) {
        if (seen[p]) { answer = index[p]; break; }
        seen[p] = true;
    }
    cout << answer << '\n';
}
