// Builds one ListNode chain per row of `lists` and calls mergeKLists.
int main() {
    using namespace judge_harness;
    vector<vector<int>> rows = read_int_grid();
    vector<ListNode*> lists;
    for (auto& row : rows) lists.push_back(make_list(row));
    ListNode* result = Solution().mergeKLists(lists);
    write(result);
}
