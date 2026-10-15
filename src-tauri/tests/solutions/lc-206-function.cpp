class Solution {
public:
    ListNode* reverseList(ListNode* head) {
        ListNode* previous = nullptr;
        while (head) {
            auto next = head->next;
            head->next = previous;
            previous = head;
            head = next;
        }
        return previous;
    }
};
