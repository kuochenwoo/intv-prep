class Solution {
public:
    // Min-heap of list heads: O(N log k).
    ListNode* mergeKLists(vector<ListNode*>& lists) {
        auto cmp = [](ListNode* a, ListNode* b) { return a->val > b->val; };
        priority_queue<ListNode*, vector<ListNode*>, decltype(cmp)> heap(cmp);
        for (ListNode* l : lists)
            if (l) heap.push(l);
        ListNode dummy;
        ListNode* tail = &dummy;
        while (!heap.empty()) {
            ListNode* node = heap.top();
            heap.pop();
            tail = tail->next = node;
            if (node->next) heap.push(node->next);
        }
        tail->next = nullptr;
        return dummy.next;
    }
};
