int main() {
    int n;
    cin >> n;
    vector<int> nums(n);
    for (auto& value : nums) cin >> value;
    Solution().moveZeroes(nums);
    for (int value : nums) cout << value << ' ';
}
