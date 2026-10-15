class Solution {
    bool mirror(TreeNode* a, TreeNode* b) {
        if (!a || !b) return a == b;
        return a->val == b->val && mirror(a->left, b->right) && mirror(a->right, b->left);
    }
public:
    // Recursive pairwise mirror check.
    bool isSymmetric(TreeNode* root) {
        return !root || mirror(root->left, root->right);
    }
};
