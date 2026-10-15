// Design-class harness: replays LeetCode's operation list against Trie.
int main() {
    using namespace judge_harness;
    vector<string> ops = read_lines(read_value<size_t>());
    vector<vector<string>> args = read_string_grid();
    if (args.size() != ops.size()) fail("invalid judge input");
    vector<string> out;
    Trie* trie = nullptr;
    auto bool_text = [](bool b) { return string(b ? "true" : "false"); };
    for (size_t i = 0; i < ops.size(); ++i) {
        if (ops[i] == "Trie") {
            delete trie;
            trie = new Trie();
            out.push_back("null");
        } else if (!trie) {
            fail("operation before constructor");
        } else if (args[i].size() != 1) {
            fail("invalid judge input");
        } else if (ops[i] == "insert") {
            trie->insert(args[i][0]);
            out.push_back("null");
        } else if (ops[i] == "search") {
            out.push_back(bool_text(trie->search(args[i][0])));
        } else if (ops[i] == "startsWith") {
            out.push_back(bool_text(trie->startsWith(args[i][0])));
        } else {
            fail("unknown operation");
        }
    }
    write(out);
}
