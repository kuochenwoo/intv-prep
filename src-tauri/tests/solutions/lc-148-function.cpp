class Solution {
public:
    // Bottom-up merge sort: O(n log n) time, O(1) extra space.
    ListNode* sortList(ListNode* head) {
        int n = 0;
        for (ListNode* p = head; p; p = p->next) ++n;
        ListNode dummy(0, head);
        for (int width = 1; width < n; width *= 2) {
            ListNode* tail = &dummy;
            ListNode* cur = dummy.next;
            while (cur) {
                ListNode* left = cur;
                ListNode* right = split(left, width);
                cur = split(right, width);
                tail = merge(left, right, tail);
            }
        }
        return dummy.next;
    }

private:
    // Cut after `count` nodes; return the remainder.
    static ListNode* split(ListNode* head, int count) {
        for (int i = 1; head && i < count; ++i) head = head->next;
        if (!head) return nullptr;
        ListNode* rest = head->next;
        head->next = nullptr;
        return rest;
    }
    // Merge a and b after tail; return the new tail.
    static ListNode* merge(ListNode* a, ListNode* b, ListNode* tail) {
        while (a && b) {
            if (a->val <= b->val) { tail->next = a; a = a->next; }
            else { tail->next = b; b = b->next; }
            tail = tail->next;
        }
        tail->next = a ? a : b;
        while (tail->next) tail = tail->next;
        return tail;
    }
};
