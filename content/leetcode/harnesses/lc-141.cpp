// Cycle harness: links the tail to node `pos` (-1 = no cycle), then calls hasCycle.
int main() {
    using namespace judge_harness;
    vector<int> values = read_values<int>(read_value<size_t>());
    int pos = read_value<int>();
    if (pos < -1 || pos >= (int)values.size()) fail("invalid judge input");
    ListNode* head = make_list(values);
    vector<ListNode*> nodes;
    for (ListNode* p = head; p; p = p->next) nodes.push_back(p);
    if (pos >= 0) nodes.back()->next = nodes[pos];
    bool result = Solution().hasCycle(head);
    write(result);
}
