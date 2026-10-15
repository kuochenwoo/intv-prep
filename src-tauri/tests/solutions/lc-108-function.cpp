class Solution {
    TreeNode* make(const vector<int>& nums, int lo, int hi) {
        if (lo > hi) return nullptr;
        int mid = lo + (hi - lo) / 2; // left-middle as root
        TreeNode* node = new TreeNode(nums[mid]);
        node->left = make(nums, lo, mid - 1);
        node->right = make(nums, mid + 1, hi);
        return node;
    }
public:
    TreeNode* sortedArrayToBST(vector<int>& nums) {
        return make(nums, 0, (int)nums.size() - 1);
    }
};
