#include <iostream>
#include <unordered_set>
#include <vector>
using namespace std;

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int x) : val(x), next(nullptr) {}
};

// Builds the shared structure, then finds the junction with a hash set of A's nodes.
int main() {
    int intersectVal;
    size_t m, n, skipA, skipB;
    cin >> intersectVal >> m;
    vector<int> a(m);
    for (auto& x : a) cin >> x;
    cin >> n;
    vector<int> b(n);
    for (auto& x : b) cin >> x;
    cin >> skipA >> skipB;
    vector<ListNode*> nodesA;
    for (size_t i = 0; i < m; ++i) {
        nodesA.push_back(new ListNode(a[i]));
        if (i) nodesA[i - 1]->next = nodesA[i];
    }
    ListNode dummy(0);
    ListNode* tail = &dummy;
    for (size_t i = 0; i < skipB; ++i) tail = tail->next = new ListNode(b[i]);
    if (skipB < n) tail->next = nodesA[skipA];
    unordered_set<ListNode*> seen(nodesA.begin(), nodesA.end());
    ListNode* hit = nullptr;
    for (ListNode* p = dummy.next; p && !hit; p = p->next)
        if (seen.count(p)) hit = p;
    int answer = -1;
    for (size_t i = 0; i < m; ++i)
        if (nodesA[i] == hit) answer = (int)i;
    cout << answer << '\n';
}
