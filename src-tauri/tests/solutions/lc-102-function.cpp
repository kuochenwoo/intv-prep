class Solution {
public:
    // BFS, one queue pass per level.
    vector<vector<int>> levelOrder(TreeNode* root) {
        vector<vector<int>> out;
        queue<TreeNode*> q;
        if (root) q.push(root);
        while (!q.empty()) {
            out.emplace_back();
            for (size_t k = q.size(); k > 0; --k) {
                TreeNode* node = q.front();
                q.pop();
                out.back().push_back(node->val);
                if (node->left) q.push(node->left);
                if (node->right) q.push(node->right);
            }
        }
        return out;
    }
};
