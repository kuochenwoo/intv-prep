class Solution {
public:
    // BFS: keep the last node of each level.
    vector<int> rightSideView(TreeNode* root) {
        vector<int> out;
        queue<TreeNode*> q;
        if (root) q.push(root);
        while (!q.empty()) {
            size_t k = q.size();
            for (size_t i = 0; i < k; ++i) {
                TreeNode* node = q.front();
                q.pop();
                if (i + 1 == k) out.push_back(node->val);
                if (node->left) q.push(node->left);
                if (node->right) q.push(node->right);
            }
        }
        return out;
    }
};
