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

vector<int> view;

// DFS visiting right before left: the first node seen at each depth is visible.
void dfs(Node* node, size_t depth) {
    if (!node) return;
    if (depth == view.size()) view.push_back(node->val);
    dfs(node->right, depth + 1);
    dfs(node->left, depth + 1);
}

int main() {
    dfs(readTree(), 0);
    for (size_t i = 0; i < view.size(); ++i) cout << (i ? " " : "") << view[i];
    cout << '\n';
}
