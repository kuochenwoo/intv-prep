// Design-class harness: replays LeetCode's operation list against LRUCache.
int main() {
    using namespace judge_harness;
    vector<string> ops = read_lines(read_value<size_t>());
    vector<vector<int>> args = read_int_grid();
    if (args.size() != ops.size()) fail("invalid judge input");
    vector<string> out;
    LRUCache* cache = nullptr;
    for (size_t i = 0; i < ops.size(); ++i) {
        if (ops[i] == "LRUCache") {
            delete cache;
            cache = new LRUCache(args[i][0]);
            out.push_back("null");
        } else if (!cache) {
            fail("operation before constructor");
        } else if (ops[i] == "put") {
            cache->put(args[i][0], args[i][1]);
            out.push_back("null");
        } else if (ops[i] == "get") {
            out.push_back(to_string(cache->get(args[i][0])));
        } else {
            fail("unknown operation");
        }
    }
    write(out);
}
