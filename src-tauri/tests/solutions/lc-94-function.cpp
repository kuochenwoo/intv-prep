class Solution {
public:
    // Iterative in-order with an explicit stack (the follow-up).
    vector<int> inorderTraversal(TreeNode* root) {
        vector<int> out;
        vector<TreeNode*> st;
        TreeNode* cur = root;
        while (cur || !st.empty()) {
            while (cur) {
                st.push_back(cur);
                cur = cur->left;
            }
            cur = st.back();
            st.pop_back();
            out.push_back(cur->val);
            cur = cur->right;
        }
        return out;
    }
};
