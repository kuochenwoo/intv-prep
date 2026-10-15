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

// Morris traversal: O(1) extra space, independent of the stack-based Function solution.
int main() {
    Node* cur = readTree();
    vector<int> out;
    while (cur) {
        if (!cur->left) {
            out.push_back(cur->val);
            cur = cur->right;
            continue;
        }
        Node* pred = cur->left;
        while (pred->right && pred->right != cur) pred = pred->right;
        if (!pred->right) {
            pred->right = cur;
            cur = cur->left;
        } else {
            pred->right = nullptr;
            out.push_back(cur->val);
            cur = cur->right;
        }
    }
    for (size_t i = 0; i < out.size(); ++i) cout << (i ? " " : "") << out[i];
    cout << '\n';
}
