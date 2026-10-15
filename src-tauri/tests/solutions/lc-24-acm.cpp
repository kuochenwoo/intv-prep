#include <iostream>
#include <vector>
using namespace std;

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int x) : val(x), next(nullptr) {}
};

// Recursive: swap the first pair, recurse on the rest.
ListNode* swapPairs(ListNode* head) {
    if (!head || !head->next) return head;
    ListNode* second = head->next;
    head->next = swapPairs(second->next);
    second->next = head;
    return second;
}

int main() {
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
    for (ListNode* p = swapPairs(head); p; p = p->next) {
        cout << (first ? "" : " ") << p->val;
        first = false;
    }
    cout << '\n';
}
