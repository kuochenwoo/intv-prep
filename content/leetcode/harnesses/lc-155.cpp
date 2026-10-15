// Design-class harness: replays LeetCode's operation list against MinStack.
int main() {
    using namespace judge_harness;
    vector<string> ops = read_lines(read_value<size_t>());
    vector<vector<int>> args = read_int_grid();
    if (args.size() != ops.size()) fail("invalid judge input");
    vector<string> out;
    MinStack* st = nullptr;
    for (size_t i = 0; i < ops.size(); ++i) {
        if (ops[i] == "MinStack") {
            delete st;
            st = new MinStack();
            out.push_back("null");
        } else if (!st) {
            fail("operation before constructor");
        } else if (ops[i] == "push") {
            if (args[i].size() != 1) fail("invalid judge input");
            st->push(args[i][0]);
            out.push_back("null");
        } else if (ops[i] == "pop") {
            st->pop();
            out.push_back("null");
        } else if (ops[i] == "top") {
            out.push_back(to_string(st->top()));
        } else if (ops[i] == "getMin") {
            out.push_back(to_string(st->getMin()));
        } else {
            fail("unknown operation");
        }
    }
    write(out);
}
