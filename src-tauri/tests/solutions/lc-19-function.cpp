class Solution {
public:
    // One pass: lead pointer moves n steps ahead, then both advance together.
    ListNode* removeNthFromEnd(ListNode* head, int n) {
        ListNode dummy(0, head);
        ListNode *fast = &dummy, *slow = &dummy;
        for (int i = 0; i < n; ++i) fast = fast->next;
        while (fast->next) {
            fast = fast->next;
            slow = slow->next;
        }
        ListNode* gone = slow->next;
        slow->next = gone->next;
        delete gone;
        return dummy.next;
    }
};
