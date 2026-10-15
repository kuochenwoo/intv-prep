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

// Top-down merge sort: split at the middle with slow/fast pointers.
ListNode* sortList(ListNode* head) {
    if (!head || !head->next) return head;
    ListNode *slow = head, *fast = head->next;
    while (fast && fast->next) {
        slow = slow->next;
        fast = fast->next->next;
    }
    ListNode* right = slow->next;
    slow->next = nullptr;
    return merge(sortList(head), sortList(right));
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int n;
    cin >> n;
    ListNode *head = nullptr, **tail = &head;
    for (int i = 0; i < n; ++i) {
        int v;
        cin >> v;
        *tail = new ListNode(v);
        tail = &(*tail)->next;
    }
    bool first = true;
    for (ListNode* p = sortList(head); p; p = p->next) {
        cout << (first ? "" : " ") << p->val;
        first = false;
    }
    cout << '\n';
}
