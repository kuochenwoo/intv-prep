class Solution {
public:
    string decodeString(string s) {
        // Two stacks: repeat counts and the prefix built before each '['.
        stack<int> counts;
        stack<string> prefixes;
        string cur;
        int k = 0;
        for (char c : s) {
            if (isdigit((unsigned char)c)) k = k * 10 + (c - '0');
            else if (c == '[') {
                counts.push(k);
                prefixes.push(std::move(cur));
                cur.clear();
                k = 0;
            } else if (c == ']') {
                string out = std::move(prefixes.top());
                prefixes.pop();
                int times = counts.top();
                counts.pop();
                out.reserve(out.size() + cur.size() * times);
                for (int i = 0; i < times; ++i) out += cur;
                cur = std::move(out);
            } else cur += c;
        }
        return cur;
    }
};
