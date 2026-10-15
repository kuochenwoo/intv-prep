// Shared-node harness: listB's tail reuses listA's nodes from skipA on.
// Prints the returned node's index in list A (-1 for nullptr).
int main() {
    using namespace judge_harness;
    int intersectVal = read_value<int>();
    vector<int> a = read_values<int>(read_value<size_t>());
    vector<int> b = read_values<int>(read_value<size_t>());
    size_t skipA = read_value<size_t>(), skipB = read_value<size_t>();
    bool shared = intersectVal != 0;
    if (skipA > a.size() || skipB > b.size()) fail("invalid judge input");
    if (shared != (skipA < a.size()) || shared != (skipB < b.size()) || a.size() - skipA != b.size() - skipB)
        fail("invalid judge input");
    ListNode* headA = make_list(a);
    vector<ListNode*> nodesA;
    for (ListNode* p = headA; p; p = p->next) nodesA.push_back(p);
    ListNode* headB = make_list(vector<int>(b.begin(), b.begin() + skipB));
    if (shared) {
        if (!headB) {
            headB = nodesA[skipA];
        } else {
            ListNode* tail = headB;
            while (tail->next) tail = tail->next;
            tail->next = nodesA[skipA];
        }
    }
    unordered_map<ListNode*, int> index;
    for (size_t i = 0; i < nodesA.size(); ++i) index[nodesA[i]] = (int)i;
    ListNode* result = Solution().getIntersectionNode(headA, headB);
    if (!result) {
        write(-1);
        return 0;
    }
    auto it = index.find(result);
    if (it == index.end()) fail("returned node is not in list A");
    write(it->second);
}
