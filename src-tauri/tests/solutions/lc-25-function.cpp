class Solution {
public:
    // Iterative, O(1) extra space: check k nodes remain, reverse them, reattach.
    ListNode* reverseKGroup(ListNode* head, int k) {
        ListNode dummy(0, head);
        ListNode* before = &dummy;
        while (true) {
            ListNode* probe = before;
            for (int i = 0; i < k && probe; ++i) probe = probe->next;
            if (!probe) break;
            ListNode* first = before->next;
            ListNode* after = probe->next;
            ListNode *prev = after, *cur = first;
            while (cur != after) {
                ListNode* next = cur->next;
                cur->next = prev;
                prev = cur;
                cur = next;
            }
            before->next = probe;
            before = first;
        }
        return dummy.next;
    }
};
