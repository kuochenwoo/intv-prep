int main() {
    int n, target;
    cin >> n >> target;
    vector<int> nums(n);
    for (auto& value : nums) cin >> value;
    auto answer = Solution().twoSum(nums, target);
    for (int index : answer) cout << index << ' ';
}
