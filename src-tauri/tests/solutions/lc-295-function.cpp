class MedianFinder {
    priority_queue<int> low;                             // smaller half, max at top
    priority_queue<int, vector<int>, greater<int>> high; // larger half, min at top
public:
    MedianFinder() {}

    void addNum(int num) {
        low.push(num);
        high.push(low.top());
        low.pop();
        if (high.size() > low.size()) {
            low.push(high.top());
            high.pop();
        }
    }

    double findMedian() {
        if (low.size() > high.size()) return low.top();
        return ((long long)low.top() + high.top()) / 2.0;
    }
};
