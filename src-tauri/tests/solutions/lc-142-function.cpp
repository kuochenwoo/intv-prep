class Solution {
public:
    // Floyd: after slow and fast meet, a pointer from head and one from the
    // meeting point advance in step and meet at the cycle entry.
    ListNode *detectCycle(ListNode *head) {
        ListNode *slow = head, *fast = head;
        while (fast && fast->next) {
            slow = slow->next;
            fast = fast->next->next;
            if (slow == fast) {
                for (ListNode* p = head; p != slow; p = p->next) slow = slow->next;
                return slow;
            }
        }
        return nullptr;
    }
};
