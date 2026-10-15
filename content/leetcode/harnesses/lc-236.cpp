// LCA harness: p and q arrive as (unique) node values. Map them to the original
// nodes, call the solution, and print the returned node's value. A null or
// foreign node prints "null", which the int checker rejects.
int main() {
    using namespace judge_harness;
    TreeNode* root = make_tree(read_tokens(read_value<size_t>()));
    int pv = read_value<int>(), qv = read_value<int>();
    TreeNode *p = nullptr, *q = nullptr;
    unordered_set<TreeNode*> original;
    vector<TreeNode*> stack;
    if (root) stack.push_back(root);
    while (!stack.empty()) {
        TreeNode* node = stack.back();
        stack.pop_back();
        original.insert(node);
        if (node->val == pv) p = node;
        if (node->val == qv) q = node;
        if (node->left) stack.push_back(node->left);
        if (node->right) stack.push_back(node->right);
    }
    if (!p || !q) fail("invalid judge input");
    TreeNode* answer = Solution().lowestCommonAncestor(root, p, q);
    if (!answer || !original.count(answer)) {
        write(string("null"));
        return 0;
    }
    write(answer->val);
}
