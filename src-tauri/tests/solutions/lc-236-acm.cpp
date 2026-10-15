#include <iostream>
#include <string>
#include <unordered_map>
#include <unordered_set>
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

// Parent pointers via iterative traversal, then walk up from p and q.
int main() {
    Node* root = readTree();
    int pv, qv;
    cin >> pv >> qv;
    unordered_map<Node*, Node*> parent{{root, nullptr}};
    Node *p = nullptr, *q = nullptr;
    vector<Node*> st{root};
    while (!st.empty()) {
        Node* node = st.back();
        st.pop_back();
        if (node->val == pv) p = node;
        if (node->val == qv) q = node;
        for (Node* c : {node->left, node->right})
            if (c) { parent[c] = node; st.push_back(c); }
    }
    unordered_set<Node*> ancestors;
    for (Node* x = p; x; x = parent[x]) ancestors.insert(x);
    Node* x = q;
    while (!ancestors.count(x)) x = parent[x];
    cout << x->val << '\n';
}
