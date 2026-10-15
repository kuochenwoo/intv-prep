#include <iostream>
#include <vector>
using namespace std;

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int x) : val(x), next(nullptr) {}
};

ListNode* merge(ListNode* a, ListNode* b) {
    ListNode dummy(0);
    ListNode* tail = &dummy;
    while (a && b) {
        ListNode*& s = a->val <= b->val ? a : b;
        tail = tail->next = s;
        s = s->next;
    }
    tail->next = a ? a : b;
    return dummy.next;
}

// Iterative pairwise divide and conquer: O(N log k).
int main() {
    int k;
    cin >> k;
    vector<ListNode*> lists(k, nullptr);
    for (auto& head : lists) {
        int len;
        cin >> len;
        ListNode** tail = &head;
        for (int i = 0; i < len; ++i) {
            int v;
            cin >> v;
            *tail = new ListNode(v);
            tail = &(*tail)->next;
        }
    }
    for (size_t step = 1; step < lists.size(); step *= 2)
        for (size_t i = 0; i + step < lists.size(); i += 2 * step)
            lists[i] = merge(lists[i], lists[i + step]);
    bool first = true;
    for (ListNode* p = lists.empty() ? nullptr : lists[0]; p; p = p->next) {
        cout << (first ? "" : " ") << p->val;
        first = false;
    }
    cout << '\n';
}
