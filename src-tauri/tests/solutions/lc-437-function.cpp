class Solution {
    unordered_map<long long, int> seen; // prefix sums on the current root path
    int count = 0;
    void dfs(TreeNode* node, long long sum, long long target) {
        if (!node) return;
        sum += node->val;
        auto it = seen.find(sum - target);
        if (it != seen.end()) count += it->second;
        ++seen[sum];
        dfs(node->left, sum, target);
        dfs(node->right, sum, target);
        --seen[sum];
    }
public:
    // Prefix sums along the root path: O(n) average.
    int pathSum(TreeNode* root, int targetSum) {
        seen.clear();
        count = 0;
        seen[0] = 1;
        dfs(root, 0, targetSum);
        return count;
    }
};
