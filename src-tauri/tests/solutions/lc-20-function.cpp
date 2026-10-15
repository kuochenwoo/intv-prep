class Solution {
public:
    bool isValid(string s) {
        // Push the expected closing bracket for every opener.
        if (s.size() % 2) return false;
        stack<char> st;
        for (char c : s) {
            if (c == '(') st.push(')');
            else if (c == '[') st.push(']');
            else if (c == '{') st.push('}');
            else {
                if (st.empty() || st.top() != c) return false;
                st.pop();
            }
        }
        return st.empty();
    }
};
