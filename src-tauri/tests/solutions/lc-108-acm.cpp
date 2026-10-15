#include <iostream>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

struct Node {
    int val;
    Node* left;
    Node* right;
    explicit Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

// Prints level order with trailing nulls trimmed (empty line for an empty tree).
void printTree(Node* root) {
    vector<string> out;
    vector<Node*> queue{root};
    for (size_t head = 0; head < queue.size(); ++head) {
        Node* node = queue[head];
        if (!node) { out.push_back("null"); continue; }
        out.push_back(to_string(node->val));
        queue.push_back(node->left);
        queue.push_back(node->right);
    }
    while (!out.empty() && out.back() == "null") out.pop_back();
    for (size_t i = 0; i < out.size(); ++i) cout << (i ? " " : "") << out[i];
    cout << '\n';
}

// Iterative divide and conquer with right-middle roots (a different valid tree).
int main() {
    int n;
    cin >> n;
    vector<int> nums(n);
    for (auto& x : nums) cin >> x;
    Node* root = nullptr;
    // (lo, hi, slot to fill)
    vector<tuple<int, int, Node**>> st{{0, n - 1, &root}};
    while (!st.empty()) {
        auto [lo, hi, slot] = st.back();
        st.pop_back();
        if (lo > hi) continue;
        int mid = lo + (hi - lo + 1) / 2;
        *slot = new Node(nums[mid]);
        st.push_back({lo, mid - 1, &(*slot)->left});
        st.push_back({mid + 1, hi, &(*slot)->right});
    }
    printTree(root);
}
