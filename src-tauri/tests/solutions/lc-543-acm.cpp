#include <algorithm>
#include <iostream>
#include <string>
#include <unordered_map>
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

// Iterative post-order via reversed pre-order; heights stored per node.
int main() {
    Node* root = readTree();
    vector<Node*> order, st{root};
    while (!st.empty()) {
        Node* node = st.back();
        st.pop_back();
        order.push_back(node);
        if (node->left) st.push_back(node->left);
        if (node->right) st.push_back(node->right);
    }
    unordered_map<Node*, int> h;
    h[nullptr] = 0;
    int best = 0;
    for (auto it = order.rbegin(); it != order.rend(); ++it) {
        int l = h[(*it)->left], r = h[(*it)->right];
        best = max(best, l + r);
        h[*it] = 1 + max(l, r);
    }
    cout << best << '\n';
}
