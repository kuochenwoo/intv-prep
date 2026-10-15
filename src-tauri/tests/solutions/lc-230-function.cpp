class Solution {
public:
    // Iterative in-order, stopping at the k-th visited node.
    int kthSmallest(TreeNode* root, int k) {
        vector<TreeNode*> st;
        TreeNode* cur = root;
        while (cur || !st.empty()) {
            while (cur) {
                st.push_back(cur);
                cur = cur->left;
            }
            cur = st.back();
            st.pop_back();
            if (--k == 0) return cur->val;
            cur = cur->right;
        }
        return -1;
    }
};
