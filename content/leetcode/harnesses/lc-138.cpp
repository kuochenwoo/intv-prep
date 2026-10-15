// Random-pointer harness: rows are [val, randomIndex] (-1 = null). Prints the
// copy in the same format and rejects copies that reuse or modify original nodes.
int main() {
    using namespace judge_harness;
    vector<vector<int>> rows = read_int_grid();
    size_t n = rows.size();
    vector<Node*> orig(n);
    for (size_t i = 0; i < n; ++i) {
        if (rows[i].size() != 2 || rows[i][1] < -1 || rows[i][1] >= (int)n) fail("invalid judge input");
        orig[i] = new Node(rows[i][0]);
    }
    for (size_t i = 0; i < n; ++i) {
        orig[i]->next = i + 1 < n ? orig[i + 1] : nullptr;
        orig[i]->random = rows[i][1] < 0 ? nullptr : orig[rows[i][1]];
    }
    Node* copy = Solution().copyRandomList(n ? orig[0] : nullptr);
    for (size_t i = 0; i < n; ++i) {
        Node* random = rows[i][1] < 0 ? nullptr : orig[rows[i][1]];
        if (orig[i]->val != rows[i][0] || orig[i]->next != (i + 1 < n ? orig[i + 1] : nullptr) || orig[i]->random != random)
            fail("the original list was modified");
    }
    unordered_set<Node*> originals(orig.begin(), orig.end());
    unordered_map<Node*, int> index;
    vector<Node*> nodes;
    for (Node* p = copy; p; p = p->next) {
        if (originals.count(p)) fail("the copy contains an original node");
        if (index.count(p) || nodes.size() > n) fail("the copy is cyclic or too long");
        index[p] = (int)nodes.size();
        nodes.push_back(p);
    }
    vector<vector<int>> out;
    for (Node* p : nodes) {
        int r = -1;
        if (p->random) {
            auto it = index.find(p->random);
            if (it == index.end()) fail("a random pointer leaves the copied list");
            r = it->second;
        }
        out.push_back({p->val, r});
    }
    write(out);
}
