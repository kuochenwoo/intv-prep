int main() {
    int n;
    cin >> n;
    ListNode dummy;
    auto tail = &dummy;
    for (int i = 0, value; i < n; ++i) {
        cin >> value;
        tail->next = new ListNode(value);
        tail = tail->next;
    }
    auto result = Solution().reverseList(dummy.next);
    int count = 0;
    while (result) {
        if (++count > n) {
            cerr << "invalid cyclic/long list";
            return 2;
        }
        cout << result->val << ' ';
        result = result->next;
    }
}
