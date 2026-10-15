// Generated-harness coverage for the Hot 100 shapes that the bundled P0 problems
// do not use yet: trees, grids, strings, bool/double results, in-place 2-D
// mutation, and order-insensitive checkers. Each case compiles real C++20.
use serde_json::{json, Value};
use swe_interview_prep::{
    code_runner::judge,
    content::{self, Problem},
};
fn problem(io: Value, tests: Value) -> Problem {
    serde_json::from_value(json!({
        "id": "lc-test", "title": "t", "difficulty": "easy", "tags": [], "statement": "",
        "constraints": [], "examples": [],
        "complexity": {"time": "", "space": "", "officialRequirement": "", "notes": "", "sourceUrl": ""},
        "limits": {"timeLimitMs": 2000, "memoryLimitMb": 256, "outputLimitKb": 64},
        "cpp": {"functionTemplate": "", "acmTemplate": "", "acmIoSpec": ""},
        "io": io,
        "tests": tests,
    }))
    .unwrap()
}
fn case(input: Value, expected: Value) -> Value {
    json!({"id": format!("c{}", input.to_string().len()), "logicalInput": input, "expected": expected, "isSample": true})
}
fn accepted(p: &Problem, code: &str) {
    content::validate(p).unwrap();
    let r = judge(p, "function", code, false, "").unwrap();
    assert_eq!(
        r.status,
        "accepted",
        "{}\n{:#?}",
        r.diagnostics,
        r.cases
            .iter()
            .map(|c| (&c.status, &c.input, &c.actual, &c.stderr))
            .collect::<Vec<_>>()
    );
    assert_eq!(r.passed, p.tests.len());
}
#[test]
fn trees_round_trip_through_level_order() {
    let p = problem(
        json!({"method": "invertTree", "params": [{"name": "root", "type": "TreeNode"}], "returns": "TreeNode"}),
        json!([
            {"id": "a", "logicalInput": [4, 2, 7, 1, 3, 6, 9], "expected": [4, 7, 2, 9, 6, 3, 1], "isSample": true},
            {"id": "b", "logicalInput": [], "expected": [], "isSample": false},
            {"id": "c", "logicalInput": [1, null, 2], "expected": [1, 2, null], "isSample": false},
        ]),
    );
    assert_eq!(content::input(&p, &p.tests[2]), "3\n1 null 2\n");
    accepted(&p, "class Solution{public:TreeNode* invertTree(TreeNode* r){if(!r)return r;swap(r->left,r->right);invertTree(r->left);invertTree(r->right);return r;}};");
    let r = judge(
        &p,
        "function",
        "class Solution{public:TreeNode* invertTree(TreeNode* r){return r;}};",
        false,
        "",
    )
    .unwrap();
    assert_eq!((r.status.as_str(), r.passed), ("wrong_answer", 1));
    let r = judge(
        &p,
        "function",
        "class Solution{public:TreeNode* invertTree(TreeNode* r){if(r)r->left=r;return r;}};",
        true,
        "",
    )
    .unwrap();
    assert_eq!(r.status, "runtime_error");
    assert!(r.cases[0].stderr.contains("cyclic"));
    let depth = problem(
        json!({"method": "maxDepth", "params": [{"name": "root", "type": "TreeNode"}], "returns": "int"}),
        json!([case(json!([3, 9, 20, null, null, 15, 7]), json!(3))]),
    );
    accepted(&depth, "class Solution{public:int maxDepth(TreeNode* r){return r?1+max(maxDepth(r->left),maxDepth(r->right)):0;}};");
}
#[test]
fn char_grids_strings_and_unordered_groups() {
    let islands = problem(
        json!({"method": "numIslands", "params": [{"name": "grid", "type": "char[][]"}], "returns": "int"}),
        json!([case(
            json!([["1", "1", "0"], ["0", "1", "0"], ["0", "0", "1"]]),
            json!(2)
        )]),
    );
    assert_eq!(
        content::input(&islands, &islands.tests[0]),
        "3\n110\n010\n001\n"
    );
    accepted(&islands, "class Solution{void f(vector<vector<char>>&g,int i,int j){if(i<0||j<0||i>=(int)g.size()||j>=(int)g[0].size()||g[i][j]!='1')return;g[i][j]='0';f(g,i+1,j);f(g,i-1,j);f(g,i,j+1);f(g,i,j-1);}public:int numIslands(vector<vector<char>>& g){int n=0;for(int i=0;i<(int)g.size();++i)for(int j=0;j<(int)g[i].size();++j)if(g[i][j]=='1'){++n;f(g,i,j);}return n;}};");
    let anagrams = problem(
        json!({"method": "groupAnagrams", "params": [{"name": "strs", "type": "string[]"}], "returns": "string[][]", "checker": "unordered_nested"}),
        json!([
            case(json!(["eat","tea","tan","ate","nat","bat"]), json!([["bat"],["nat","tan"],["ate","eat","tea"]])),
            {"id": "empty-string", "logicalInput": [""], "expected": [[""]], "isSample": false},
        ]),
    );
    accepted(&anagrams, "class Solution{public:vector<vector<string>> groupAnagrams(vector<string>& s){map<string,vector<string>> m;for(auto&x:s){auto k=x;sort(k.begin(),k.end());m[k].push_back(x);}vector<vector<string>> r;for(auto&[k,v]:m)r.push_back(v);return r;}};");
    let p = &anagrams;
    assert!(!content::compare(
        p,
        &p.tests[0],
        "3\n1\nbat\n2\nnat\ntan\n2\nate\neat\n"
    ));
    let decode = problem(
        json!({"method": "decodeString", "params": [{"name": "s", "type": "string"}], "returns": "string"}),
        json!([case(json!("a b"), json!("a b")), {"id": "e", "logicalInput": "", "expected": "", "isSample": false}]),
    );
    accepted(
        &decode,
        "class Solution{public:string decodeString(string s){return s;}};",
    );
}
#[test]
fn bool_double_multi_param_and_in_place_matrix() {
    let jump = problem(
        json!({"method": "canJump", "params": [{"name": "nums", "type": "int[]"}], "returns": "bool"}),
        json!([case(json!([2,3,1,1,4]), json!(true)), {"id": "f", "logicalInput": [3,2,1,0,4], "expected": false, "isSample": false}]),
    );
    accepted(&jump, "class Solution{public:bool canJump(vector<int>& a){int r=0;for(int i=0;i<(int)a.size();++i){if(i>r)return false;r=max(r,i+a[i]);}return true;}};");
    let median = problem(
        json!({"method": "findMedianSortedArrays", "params": [{"name": "nums1", "type": "int[]"}, {"name": "nums2", "type": "int[]"}], "returns": "double"}),
        json!([case(json!({"nums1": [1,3], "nums2": [2]}), json!(2.0)), {"id": "g", "logicalInput": {"nums1": [1,2], "nums2": [3,4]}, "expected": 2.5, "isSample": false}]),
    );
    assert_eq!(content::input(&median, &median.tests[0]), "2\n1 3\n1\n2\n");
    accepted(&median, "class Solution{public:double findMedianSortedArrays(vector<int>& a,vector<int>& b){vector<int> c=a;c.insert(c.end(),b.begin(),b.end());sort(c.begin(),c.end());int n=c.size();return n%2?c[n/2]:(c[n/2-1]+c[n/2])/2.0;}};");
    let rotate = problem(
        json!({"method": "rotate", "params": [{"name": "matrix", "type": "int[][]"}], "returns": "void", "output": "matrix"}),
        json!([case(
            json!([[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
            json!([[7, 4, 1], [8, 5, 2], [9, 6, 3]])
        )]),
    );
    accepted(&rotate, "class Solution{public:void rotate(vector<vector<int>>& m){int n=m.size();for(int i=0;i<n;++i)for(int j=i+1;j<n;++j)swap(m[i][j],m[j][i]);for(auto&r:m)reverse(r.begin(),r.end());}};");
    let subsets = problem(
        json!({"method": "subsets", "params": [{"name": "nums", "type": "int[]"}], "returns": "int[][]", "checker": "unordered_nested"}),
        json!([case(json!([1, 2]), json!([[], [1], [2], [1, 2]]))]),
    );
    accepted(&subsets, "class Solution{public:vector<vector<int>> subsets(vector<int>& a){vector<vector<int>> r{{}};for(int x:a){int n=r.size();for(int i=0;i<n;++i){auto s=r[i];s.insert(s.begin(),x);r.push_back(s);}}return r;}};");
}
#[test]
fn invalid_specs_and_tests_are_rejected_at_load() {
    let io = |extra: Value| {
        let mut base = json!({"method": "f", "params": [{"name": "nums", "type": "int[]"}, {"name": "k", "type": "int"}], "returns": "int"});
        base.as_object_mut()
            .unwrap()
            .extend(extra.as_object().unwrap().clone());
        base
    };
    let good = json!([case(json!({"nums": [1], "k": 1}), json!(1))]);
    assert!(content::validate(&problem(io(json!({})), good.clone())).is_ok());
    for bad in [
        json!({"returns": "vector<int>"}),
        json!({"returns": "void"}),
        json!({"output": "missing"}),
        json!({"checker": "fuzzy"}),
        json!({"method": "not valid"}),
        json!({"stdin": ["nums k"]}),
        json!({"stdin": ["nums.length nums"]}),
        json!({"stdin": ["nums.length k nums k"]}),
    ] {
        assert!(
            content::validate(&problem(io(bad.clone()), good.clone())).is_err(),
            "{bad}"
        );
    }
    for tests in [
        json!([case(json!({"nums": [1]}), json!(1))]),
        json!([case(json!({"nums": [1.5], "k": 1}), json!(1))]),
        json!([case(json!({"nums": [1], "k": 3000000000i64}), json!(1))]),
        json!([case(json!({"nums": [1], "k": 1}), json!("x"))]),
        json!([]),
    ] {
        assert!(
            content::validate(&problem(io(json!({})), tests.clone())).is_err(),
            "{tests}"
        );
    }
}
#[test]
fn bundled_library_is_sorted_cached_and_hides_tests_from_the_ui() {
    let ids: Vec<_> = content::problems().iter().map(|p| p.id.as_str()).collect();
    let numbers: Vec<u32> = ids.iter().map(|id| id[3..].parse().unwrap()).collect();
    assert!(numbers.windows(2).all(|w| w[0] < w[1]));
    for p in content::problems() {
        assert!(p.tests.is_empty());
        assert!(std::ptr::eq(p, content::find(&p.id).unwrap()));
        assert!(!content::problem(&p.id).unwrap().tests.is_empty());
    }
    let listed = serde_json::to_value(content::problems()).unwrap();
    assert!(listed[0].get("tests").is_none());
    assert_eq!(listed[0]["io"]["method"], "twoSum");
}
