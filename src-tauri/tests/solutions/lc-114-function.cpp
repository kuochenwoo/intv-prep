class Solution {
public:
    // O(1) extra space: splice each left subtree between the node and its right subtree.
    void flatten(TreeNode* root) {
        for (TreeNode* cur = root; cur; cur = cur->right) {
            if (!cur->left) continue;
            TreeNode* tail = cur->left;
            while (tail->right) tail = tail->right;
            tail->right = cur->right;
            cur->right = cur->left;
            cur->left = nullptr;
        }
    }
};
