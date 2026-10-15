class Solution {
    // Bounds are exclusive and widened to long long so INT_MIN/INT_MAX values work.
    bool check(TreeNode* node, long long lo, long long hi) {
        if (!node) return true;
        if (node->val <= lo || node->val >= hi) return false;
        return check(node->left, lo, node->val) && check(node->right, node->val, hi);
    }
public:
    bool isValidBST(TreeNode* root) {
        return check(root, LLONG_MIN, LLONG_MAX);
    }
};
