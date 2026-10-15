// Cycle harness: links the tail to node `pos` (-1 = no cycle), calls detectCycle,
// and prints the returned node's index (-1 for nullptr).
int main() {
    using namespace judge_harness;
    vector<int> values = read_values<int>(read_value<size_t>());
    int pos = read_value<int>();
    if (pos < -1 || pos >= (int)values.size()) fail("invalid judge input");
    ListNode* head = make_list(values);
    vector<ListNode*> nodes;
    for (ListNode* p = head; p; p = p->next) nodes.push_back(p);
    if (pos >= 0) nodes.back()->next = nodes[pos];
    unordered_map<ListNode*, int> index;
    for (size_t i = 0; i < nodes.size(); ++i) index[nodes[i]] = (int)i;
    ListNode* result = Solution().detectCycle(head);
    if (!result) {
        write(-1);
        return 0;
    }
    auto it = index.find(result);
    if (it == index.end()) fail("returned node is not in the list");
    write(it->second);
}
