class Solution {
    int best = INT_MIN;
    // Best downward chain starting at node; updates the best bent path through node.
    int gain(TreeNode* node) {
        if (!node) return 0;
        int l = max(gain(node->left), 0), r = max(gain(node->right), 0);
        best = max(best, node->val + l + r);
        return node->val + max(l, r);
    }
public:
    int maxPathSum(TreeNode* root) {
        best = INT_MIN;
        gain(root);
        return best;
    }
};
