class Solution {
public:
    // Find the middle, reverse the second half in place, compare, then restore.
    bool isPalindrome(ListNode* head) {
        ListNode *slow = head, *fast = head;
        while (fast->next && fast->next->next) {
            slow = slow->next;
            fast = fast->next->next;
        }
        ListNode* second = reverse(slow->next);
        bool same = true;
        for (ListNode *p = head, *q = second; q; p = p->next, q = q->next)
            if (p->val != q->val) { same = false; break; }
        slow->next = reverse(second);
        return same;
    }

private:
    static ListNode* reverse(ListNode* head) {
        ListNode* prev = nullptr;
        while (head) {
            ListNode* next = head->next;
            head->next = prev;
            prev = head;
            head = next;
        }
        return prev;
    }
};
