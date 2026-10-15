#include <iostream>
#include <string>
#include <utility>
#include <vector>
using namespace std;

struct Node {
    int val;
    Node* left;
    Node* right;
    explicit Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

// Builds a tree from LeetCode level-order tokens ("null" marks a missing child).
Node* readTree() {
    size_t m = 0;
    cin >> m;
    vector<string> tokens(m);
    for (auto& t : tokens) cin >> t;
    if (m == 0 || tokens[0] == "null") return nullptr;
    auto make = [&](size_t i) -> Node* { return i < m && tokens[i] != "null" ? new Node(stoi(tokens[i])) : nullptr; };
    Node* root = make(0);
    vector<Node*> queue{root};
    size_t next = 1;
    for (size_t head = 0; head < queue.size() && next < m; ++head) {
        queue[head]->left = make(next++);
        queue[head]->right = make(next++);
        if (queue[head]->left) queue.push_back(queue[head]->left);
        if (queue[head]->right) queue.push_back(queue[head]->right);
    }
    return root;
}

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

// Iterative: swap children of every node with an explicit stack.
int main() {
    Node* root = readTree();
    vector<Node*> st;
    if (root) st.push_back(root);
    while (!st.empty()) {
        Node* node = st.back();
        st.pop_back();
        swap(node->left, node->right);
        if (node->left) st.push_back(node->left);
        if (node->right) st.push_back(node->right);
    }
    printTree(root);
}
