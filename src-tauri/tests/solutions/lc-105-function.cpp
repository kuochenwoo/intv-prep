class Solution {
    unordered_map<int, int> pos;
    size_t next = 0;
    // Pre-order index walks forward; the in-order range [lo, hi] bounds the subtree.
    TreeNode* make(const vector<int>& preorder, int lo, int hi) {
        if (lo > hi) return nullptr;
        TreeNode* node = new TreeNode(preorder[next++]);
        int mid = pos[node->val];
        node->left = make(preorder, lo, mid - 1);
        node->right = make(preorder, mid + 1, hi);
        return node;
    }
public:
    TreeNode* buildTree(vector<int>& preorder, vector<int>& inorder) {
        pos.clear();
        next = 0;
        for (int i = 0; i < (int)inorder.size(); ++i) pos[inorder[i]] = i;
        return make(preorder, 0, (int)inorder.size() - 1);
    }
};
