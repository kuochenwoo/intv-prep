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

// Pre-order DFS with an explicit (node, depth) stack, right child pushed first.
int main() {
    Node* root = readTree();
    vector<vector<int>> levels;
    vector<pair<Node*, size_t>> st;
    if (root) st.push_back({root, 0});
    while (!st.empty()) {
        auto [node, d] = st.back();
        st.pop_back();
        if (d == levels.size()) levels.emplace_back();
        levels[d].push_back(node->val);
        if (node->right) st.push_back({node->right, d + 1});
        if (node->left) st.push_back({node->left, d + 1});
    }
    cout << levels.size() << '\n';
    for (auto& row : levels) {
        for (size_t i = 0; i < row.size(); ++i) cout << (i ? " " : "") << row[i];
        cout << '\n';
    }
}
