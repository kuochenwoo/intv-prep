#include <iostream>
#include <vector>
using namespace std;

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int x) : val(x), next(nullptr) {}
};

// Two passes: count the length, then unlink node (len - n).
int main() {
    int sz, n;
    cin >> sz;
    ListNode *head = nullptr, **tail = &head;
    for (int i = 0; i < sz; ++i) {
        int v;
        cin >> v;
        *tail = new ListNode(v);
        tail = &(*tail)->next;
    }
    cin >> n;
    int len = 0;
    for (ListNode* p = head; p; p = p->next) ++len;
    ListNode** link = &head;
    for (int i = 0; i < len - n; ++i) link = &(*link)->next;
    *link = (*link)->next;
    bool first = true;
    for (ListNode* p = head; p; p = p->next) {
        cout << (first ? "" : " ") << p->val;
        first = false;
    }
    cout << '\n';
}
