class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int lowest = INT_MAX, best = 0;
        for (int p : prices) {
            lowest = min(lowest, p);
            best = max(best, p - lowest);
        }
        return best;
    }
};
