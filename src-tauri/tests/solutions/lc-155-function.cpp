class MinStack {
    vector<int> values;
    vector<int> mins; // mins.back() is the minimum of values; equal values are pushed too
public:
    MinStack() {}

    void push(int val) {
        values.push_back(val);
        if (mins.empty() || val <= mins.back()) mins.push_back(val);
    }

    void pop() {
        if (values.back() == mins.back()) mins.pop_back();
        values.pop_back();
    }

    int top() {
        return values.back();
    }

    int getMin() {
        return mins.back();
    }
};
