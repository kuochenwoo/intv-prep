#include <iostream>
#include <queue>
#include <vector>
using namespace std;

// BFS from 0: the level at which `amount` is first reached is the fewest coins.
int main() {
    int m;
    cin >> m;
    vector<long long> coins(m);
    for (auto& c : coins) cin >> c;
    long long amount;
    cin >> amount;
    vector<int> dist(amount + 1, -1);
    dist[0] = 0;
    queue<long long> q;
    q.push(0);
    while (!q.empty() && dist[amount] < 0) {
        long long v = q.front();
        q.pop();
        for (long long c : coins) {
            long long w = v + c;
            if (w <= amount && dist[w] < 0) {
                dist[w] = dist[v] + 1;
                q.push(w);
            }
        }
    }
    cout << dist[amount] << '\n';
}
