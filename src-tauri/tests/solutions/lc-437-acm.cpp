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

// O(n·h): from every node, count downward paths starting there.
long long countFrom(Node* node, long long need) {
    if (!node) return 0;
    long long rest = need - node->val;
    return (rest == 0) + countFrom(node->left, rest) + countFrom(node->right, rest);
}

int main() {
    Node* root = readTree();
    long long target;
    cin >> target;
    long long total = 0;
    vector<Node*> st;
    if (root) st.push_back(root);
    while (!st.empty()) {
        Node* node = st.back();
        st.pop_back();
        total += countFrom(node, target);
        if (node->left) st.push_back(node->left);
        if (node->right) st.push_back(node->right);
    }
    cout << total << '\n';
}
