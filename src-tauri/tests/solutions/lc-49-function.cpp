class Solution {
public:
    vector<vector<string>> groupAnagrams(vector<string>& strs) {
        // Key each string by its sorted letters.
        unordered_map<string, vector<string>> groups;
        for (const string& s : strs) {
            string key = s;
            sort(key.begin(), key.end());
            groups[key].push_back(s);
        }
        vector<vector<string>> result;
        result.reserve(groups.size());
        for (auto& entry : groups) result.push_back(move(entry.second));
        return result;
    }
};
