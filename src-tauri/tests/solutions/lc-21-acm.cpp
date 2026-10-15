#include <iostream>
#include <vector>
using namespace std;

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int x) : val(x), next(nullptr) {}
};

ListNode* readList() {
    int n;
    cin >> n;
    ListNode *head = nullptr, **tail = &head;
    for (int i = 0; i < n; ++i) {
        int v;
        cin >> v;
        *tail = new ListNode(v);
        tail = &(*tail)->next;
    }
    return head;
}

// Recursive merge.
ListNode* merge(ListNode* a, ListNode* b) {
    if (!a) return b;
    if (!b) return a;
    if (a->val <= b->val) { a->next = merge(a->next, b); return a; }
    b->next = merge(a, b->next);
    return b;
}

int main() {
    ListNode* a = readList();
    ListNode* b = readList();
    bool first = true;
    for (ListNode* p = merge(a, b); p; p = p->next) {
        cout << (first ? "" : " ") << p->val;
        first = false;
    }
    cout << '\n';
}
