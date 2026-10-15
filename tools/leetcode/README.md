# Adding LeetCode problems

Each problem is four or five files:

| File | Purpose |
| --- | --- |
| `tools/leetcode/problems/lc-<n>.py` | Source of truth: metadata, test inputs, independent Python reference. Running it writes the JSON. |
| `content/leetcode/problems/lc-<n>.json` | Generated. Bundled into the app by `build.rs`. Never hand-edit; rerun the script. |
| `src-tauri/tests/solutions/lc-<n>-function.cpp` | Reference `class Solution` (Function mode). |
| `src-tauri/tests/solutions/lc-<n>-acm.cpp` | Independent full program (ACM mode) following `acmIoSpec`. |
| `content/leetcode/harnesses/lc-<n>.cpp` | Only when the generated harness cannot express the call (design classes, cycles, shared nodes). |

Examples: `problems/lc-42.py` (plain function) and `problems/lc-146.py` + `content/leetcode/harnesses/lc-146.cpp` (design class).

## Workflow

```sh
python3 tools/leetcode/problems/lc-<n>.py
INTV_CONTENT_DIR=content/leetcode src-tauri/target/debug/examples/verify_problem content/leetcode/problems/lc-<n>.json
```

`verify_problem` validates the JSON, compiles both templates, and judges both reference solutions on every test with the real Judge. A problem is done only when it prints `OK`. Rebuild the tool with `cargo build --manifest-path src-tauri/Cargo.toml --no-default-features --example verify_problem` after Rust changes. `cargo test` re-checks every bundled problem the same way.

## Content rules

- **Statement**: Simplified Chinese, independently written. Paraphrase; never copy LeetCode's text or official test data. Define node structures for list/tree problems.
- **Title**: the leetcode.cn Chinese title. `difficulty`: `easy` / `medium` / `hard`. Tags in Chinese.
- **Constraints**: the official constraints in Chinese, plus any local deviation (e.g. extra empty-input tests).
- **Complexity** (required for every problem, shown in the problem view):
  - `time`, `space`: the local training target. Space means auxiliary space; say whether input/output counts and whether recursion stack is included.
  - `officialRequirement`: what LeetCode's statement or follow-up actually requires, e.g. `官方要求 O(log(m+n)) 时间。`, or `官方 follow-up 要求 O(1) 额外空间。`, or `官方未强制复杂度要求。` Never present a recommended complexity as official.
  - `notes`: define size variables (n, m, …), average vs worst case, and any scaled-down local test sizes.
  - `sourceUrl`: `https://leetcode.com/problems/<slug>/`.
- **Limits**: default 1000 ms / 128 MiB / 64 KiB. Raise `outputLimitKb` when the correct output is large, and time only when measured reference runs need it; leave a comment in the `.py` saying why.
- **Tests**:
  - Published examples as samples (`sample: True`, `expected` given; `write` asserts the reference agrees).
  - Edge cases (minimum sizes, duplicates, negatives, empty where allowed) and a moderate random test.
  - One hidden test at the official maximum, id `max-size-<n>`. If that would make the JSON larger than about 600 KiB, use a representative scaled-down worst case `worst-case-<…>` and state local vs official size in `notes`.
  - Shape worst cases to stress naive solutions (sorted, all equal, deep trees, long chains).
  - Every expected value comes from the Python reference, written independently of the C++ (brute force for small inputs where practical). Use a fixed seed via `rng(seed)`.
- **Multiple valid answers**: use a checker (below), never an `exact` comparison against one arbitrary valid answer.
- **C++**: Function mode is only the class; common headers are already included. ACM mode is a full program with explicit standard headers. Do not use `<bits/stdc++.h>` (missing on macOS clang). Prefer a different algorithm in the two files so they cross-check each other. `functionTemplate` uses the LeetCode signature, compiles as-is (dummy return), comments in English.
- **acmIoSpec**: Chinese, and exactly the stdin the `io` spec produces plus the stdout the checker parses.
- **en**: an English block shown when the UI is in English: `{"title", "tags", "statement", "constraints", "complexity": {"time", "space", "officialRequirement", "notes"}, "acmIoSpec"}`. Use the official LeetCode English title and tag names, one English tag per Chinese tag and one English constraint per Chinese one, and no Chinese characters. `cargo test` enforces this.

## The `io` spec

Each problem JSON has an `io` block that drives Function-mode wrapping, ACM stdin and checking:

```json
"io": {
  "method": "twoSum",
  "params": [{ "name": "nums", "type": "int[]" }, { "name": "target", "type": "int" }],
  "returns": "int[]",
  "stdin": ["nums.length target", "nums"],
  "checker": "valid_pair"
}
```

- `output`: `"return"` (default), or a parameter name checked after an in-place call, e.g. `"returns": "void", "output": "nums"`.
- `harness` / `prelude`: see "Custom harnesses" below.

- Types: `int`, `long`, `double`, `bool`, `string` (one raw line), `int[]`, `long[]`, `string[]`, `int[][]`, `char[][]`, `string[][]`, `ListNode`, `TreeNode`; `returns` may be `void` with `output` naming the in-place parameter. `int` values must fit in 32 bits.
- `logicalInput`: the bare value for one parameter, otherwise an object keyed by parameter name.
- Default stdin per parameter, in order: scalar → one line; `string` → the raw line; 1-D array / `ListNode` → `n` line then the values on one line; `string[]` → `n` then one string per line; `int[][]` → row count then `k v1 … vk` per row; `char[][]` → row count then each row as one string; `string[][]` → group count, then `k` and `k` lines per group; `TreeNode` → token count then level-order tokens with `null`.
- `stdin` (optional) overrides the layout for scalars and 1-D arrays only: e.g. `["nums.length target", "nums"]`.
- stdout: scalars one token (`bool` as `true`/`false`); `string` the raw line; 1-D arrays, lists and trees the values on one line (no length; trees level order, trailing nulls trimmed); `string[]` count then lines; `int[][]` row count then each row's values on a line (no per-row length); `char[][]` row count then row strings; `string[][]` group count, then `k` and `k` lines per group.
- Checkers: `exact` (default; doubles within 1e-5 relative), `unordered` (outer order ignored), `unordered_nested` (outer and inner order ignored), `valid_pair` (Two Sum), `longest_palindrome` (any longest palindromic substring of param `s`), `balanced_bst` (any height-balanced BST whose in-order is param `nums`), `tokens` (`string[]` compared token by token, numbers within 1e-5; for design classes).

## Custom harnesses

Set `"harness": "lc-<n>.cpp"`. `params` still describe stdin (and the ACM protocol), `returns` the printed output type, `checker` the comparison. The file contains only `int main() { … }`; inside, `using namespace judge_harness;` provides:

- `read_value<T>()`, `read_values<T>(n)`, `read_line()`, `read_lines(n)`, `read_tokens(n)`, `read_int_grid()`, `read_char_grid()`, `read_string_grid()`
- `make_list(vector<int>)`, `make_tree(vector<string> level_order_tokens)` (when a `ListNode`/`TreeNode` type appears in `params` or `returns`)
- `write(x)` for every output type above, `fail(message)` (exit 2, message on stderr)
- `ListNode::judge_created` / `TreeNode::judge_created` count constructed nodes, for cycle guards.

`"prelude"` inserts C++ declarations before the user's code (e.g. a custom `Node` struct). Patterns:

- Design classes (`LRUCache`, `MinStack`, `Trie`, `MedianFinder`): params `ops: string[]` and `args: int[][]` (or `string[][]`), returns `string[]`, checker `tokens`; print `null` for constructors and void calls.
- Cycles (`hasCycle`, `detectCycle`): params `head: ListNode`, `pos: int`; the harness links the tail to node `pos`. Return node identity as an index (`-1` for null), never a value.
- Shared nodes (intersection, LCA, random pointers): pass indices or unique values in stdin and map results back to indices/values in the harness; check the user did not return or reuse an original node where a copy is required.
