class Trie {
    // Node pool with fixed 26-way child arrays; index 0 is the root.
    vector<array<int, 26>> child;
    vector<bool> terminal;

    int walk(const string& s) const {
        int node = 0;
        for (char c : s) {
            node = child[node][c - 'a'];
            if (node == 0) return -1;
        }
        return node;
    }

public:
    Trie() : child(1), terminal(1, false) { child[0].fill(0); }

    void insert(string word) {
        int node = 0;
        for (char c : word) {
            int& next = child[node][c - 'a'];
            if (next == 0) {
                next = child.size();
                child.emplace_back();
                child.back().fill(0);
                terminal.push_back(false);
            }
            node = child[node][c - 'a'];
        }
        terminal[node] = true;
    }

    bool search(string word) {
        int node = walk(word);
        return node >= 0 && terminal[node];
    }

    bool startsWith(string prefix) { return walk(prefix) >= 0; }
};
