class Solution {
public:
    // Iterative splice with a dummy head.
    ListNode* mergeTwoLists(ListNode* list1, ListNode* list2) {
        ListNode dummy;
        ListNode* tail = &dummy;
        while (list1 && list2) {
            ListNode*& smaller = list1->val <= list2->val ? list1 : list2;
            tail = tail->next = smaller;
            smaller = smaller->next;
        }
        tail->next = list1 ? list1 : list2;
        return dummy.next;
    }
};
