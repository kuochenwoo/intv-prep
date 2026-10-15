#include <iostream>
#include <string>
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

// Stack-based construction without a hash map: walk pre-order, pop while the
// stack top matches the next in-order value, attaching right children.
int main() {
    int n;
    cin >> n;
    vector<int> pre(n);
    for (auto& x : pre) cin >> x;
    cin >> n;
    vector<int> in(n);
    for (auto& x : in) cin >> x;
    Node* root = new Node(pre[0]);
    vector<Node*> st{root};
    size_t j = 0;
    for (int i = 1; i < n; ++i) {
        Node* node = st.back();
        if (node->val != in[j]) {
            node->left = new Node(pre[i]);
            st.push_back(node->left);
        } else {
            while (!st.empty() && st.back()->val == in[j]) {
                node = st.back();
                st.pop_back();
                ++j;
            }
            node->right = new Node(pre[i]);
            st.push_back(node->right);
        }
    }
    printTree(root);
}
