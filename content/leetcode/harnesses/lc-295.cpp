// Design-class harness: replays LeetCode's operation list against MedianFinder.
int main() {
    using namespace judge_harness;
    vector<string> ops = read_lines(read_value<size_t>());
    vector<vector<int>> args = read_int_grid();
    if (args.size() != ops.size()) fail("invalid judge input");
    vector<string> out;
    MedianFinder* finder = nullptr;
    for (size_t i = 0; i < ops.size(); ++i) {
        if (ops[i] == "MedianFinder") {
            delete finder;
            finder = new MedianFinder();
            out.push_back("null");
        } else if (!finder) {
            fail("operation before constructor");
        } else if (ops[i] == "addNum") {
            if (args[i].size() != 1) fail("invalid judge input");
            finder->addNum(args[i][0]);
            out.push_back("null");
        } else if (ops[i] == "findMedian") {
            ostringstream text;
            text << setprecision(12) << finder->findMedian();
            out.push_back(text.str());
        } else {
            fail("unknown operation");
        }
    }
    write(out);
}
