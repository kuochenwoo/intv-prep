#include <algorithm>
#include <climits>
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

// Iterative: index nodes in BFS order, then process them in reverse (children first).
int main() {
    Node* root = readTree();
    vector<Node*> order{root};
    for (size_t i = 0; i < order.size(); ++i) {
        if (order[i]->left) order.push_back(order[i]->left);
        if (order[i]->right) order.push_back(order[i]->right);
    }
    // Store each node's downward gain in its val field once processed (values no longer needed).
    long long best = LLONG_MIN;
    for (size_t i = order.size(); i-- > 0;) {
        Node* node = order[i];
        long long l = node->left ? max(node->left->val, 0) : 0;
        long long r = node->right ? max(node->right->val, 0) : 0;
        best = max(best, node->val + l + r);
        node->val = (int)(node->val + max(l, r));
    }
    cout << best << '\n';
}
