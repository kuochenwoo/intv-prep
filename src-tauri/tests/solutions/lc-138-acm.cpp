#include <iostream>
#include <unordered_map>
#include <vector>
using namespace std;

struct Node {
    int val;
    Node* next;
    Node* random;
    Node(int v) : val(v), next(nullptr), random(nullptr) {}
};

// Hash map original -> copy, then serialize the copy by walking it.
int main() {
    int n;
    cin >> n;
    vector<Node*> orig(n);
    vector<int> rnd(n);
    for (int i = 0; i < n; ++i) {
        int k, v;
        cin >> k >> v >> rnd[i];
        orig[i] = new Node(v);
    }
    for (int i = 0; i < n; ++i) {
        orig[i]->next = i + 1 < n ? orig[i + 1] : nullptr;
        orig[i]->random = rnd[i] >= 0 ? orig[rnd[i]] : nullptr;
    }
    unordered_map<Node*, Node*> copyOf{{nullptr, nullptr}};
    for (Node* p = n ? orig[0] : nullptr; p; p = p->next) copyOf[p] = new Node(p->val);
    for (Node* p = n ? orig[0] : nullptr; p; p = p->next) {
        copyOf[p]->next = copyOf[p->next];
        copyOf[p]->random = copyOf[p->random];
    }
    unordered_map<Node*, int> index;
    vector<Node*> nodes;
    for (Node* p = copyOf[n ? orig[0] : nullptr]; p; p = p->next) {
        index[p] = (int)nodes.size();
        nodes.push_back(p);
    }
    cout << nodes.size() << '\n';
    for (Node* p : nodes) cout << p->val << ' ' << (p->random ? index[p->random] : -1) << '\n';
}
