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

// Iterative in-order: values must be strictly increasing.
int main() {
    Node* cur = readTree();
    vector<Node*> st;
    Node* prev = nullptr;
    bool ok = true;
    while (ok && (cur || !st.empty())) {
        while (cur) {
            st.push_back(cur);
            cur = cur->left;
        }
        cur = st.back();
        st.pop_back();
        if (prev && prev->val >= cur->val) ok = false;
        prev = cur;
        cur = cur->right;
    }
    cout << (ok ? "true" : "false") << '\n';
}
