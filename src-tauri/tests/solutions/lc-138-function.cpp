class Solution {
public:
    // Interleave copies after originals, wire random via orig->random->next,
    // then split the lists and restore the original. O(1) extra space.
    Node* copyRandomList(Node* head) {
        for (Node* p = head; p; p = p->next->next) {
            Node* c = new Node(p->val);
            c->next = p->next;
            p->next = c;
        }
        for (Node* p = head; p; p = p->next->next)
            p->next->random = p->random ? p->random->next : nullptr;
        Node dummy(0);
        Node* tail = &dummy;
        for (Node* p = head; p; p = p->next) {
            tail = tail->next = p->next;
            p->next = p->next->next;
        }
        return dummy.next;
    }
};
