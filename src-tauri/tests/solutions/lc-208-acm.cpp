#include <iostream>
#include <memory>
#include <string>
#include <unordered_map>
#include <vector>
using namespace std;

// Trie with hash-map children (sparse), independent of the array-backed Function-mode trie.
struct Node {
    unordered_map<char, unique_ptr<Node>> next;
    bool end = false;
};

int main() {
    size_t m;
    cin >> m;
    vector<string> ops(m);
    for (auto& op : ops) cin >> op;
    cin >> m;
    vector<vector<string>> args(m);
    for (auto& a : args) {
        size_t k;
        cin >> k;
        a.resize(k);
        for (auto& s : a) cin >> s;
    }
    unique_ptr<Node> root;
    auto find = [&](const string& s) -> Node* {
        Node* cur = root.get();
        for (char c : s) {
            auto it = cur->next.find(c);
            if (it == cur->next.end()) return nullptr;
            cur = it->second.get();
        }
        return cur;
    };
    cout << m << '\n';
    for (size_t i = 0; i < m; ++i) {
        if (ops[i] == "Trie") {
            root = make_unique<Node>();
            cout << "null\n";
        } else if (ops[i] == "insert") {
            Node* cur = root.get();
            for (char c : args[i][0]) {
                auto& slot = cur->next[c];
                if (!slot) slot = make_unique<Node>();
                cur = slot.get();
            }
            cur->end = true;
            cout << "null\n";
        } else if (ops[i] == "search") {
            Node* node = find(args[i][0]);
            cout << (node && node->end ? "true" : "false") << '\n';
        } else {
            cout << (find(args[i][0]) ? "true" : "false") << '\n';
        }
    }
}
