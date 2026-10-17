// Typed I/O specification shared by the generated function-mode harness, the
// ACM stdin serializer, and the output checker. Adding a problem only needs an
// `io` block in its JSON; no per-problem Rust or C++ adapter.
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Io {
    pub method: String,
    pub params: Vec<Param>,
    /// A value type, or "void" when the result is an in-place parameter.
    pub returns: String,
    /// "return", or the name of the parameter checked after an in-place call.
    #[serde(default = "default_output")]
    pub output: String,
    /// Optional stdin layout. Each entry is one line of space-separated items:
    /// `name` (a scalar or a 1-D array's elements) or `name.length`. When
    /// omitted, every parameter uses its type's default layout in order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stdin: Vec<String>,
    #[serde(default = "default_checker")]
    pub checker: String,
    /// Custom Function-mode main() file in content/leetcode/harnesses/, for
    /// problems whose call shape the generator cannot express (design classes,
    /// cycles, shared nodes). `params` still describe stdin; `returns` the output.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub harness: Option<String>,
    /// C++ declarations inserted before the user's code in Function mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prelude: Option<String>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Param {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
}
fn default_output() -> String {
    "return".into()
}
fn default_checker() -> String {
    "exact".into()
}
pub const CHECKERS: &[&str] = &[
    "exact",
    "unordered",
    "unordered_nested",
    "valid_pair",
    "longest_palindrome",
    "balanced_bst",
    "tokens",
];

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Ty {
    Int,
    Long,
    Double,
    Bool,
    Str,
    IntArr,
    LongArr,
    StrArr,
    IntGrid,
    CharGrid,
    StrGrid,
    List,
    Tree,
}
impl Ty {
    pub fn parse(s: &str) -> Result<Ty, String> {
        Ok(match s {
            "int" => Ty::Int,
            "long" => Ty::Long,
            "double" => Ty::Double,
            "bool" => Ty::Bool,
            "string" => Ty::Str,
            "int[]" => Ty::IntArr,
            "long[]" => Ty::LongArr,
            "string[]" => Ty::StrArr,
            "int[][]" => Ty::IntGrid,
            "char[][]" => Ty::CharGrid,
            "string[][]" => Ty::StrGrid,
            "ListNode" => Ty::List,
            "TreeNode" => Ty::Tree,
            _ => return Err(format!("unknown type `{s}`")),
        })
    }
    fn cpp(self) -> &'static str {
        match self {
            Ty::Int => "int",
            Ty::Long => "long long",
            Ty::Double => "double",
            Ty::Bool => "bool",
            Ty::Str => "string",
            Ty::IntArr => "vector<int>",
            Ty::LongArr => "vector<long long>",
            Ty::StrArr => "vector<string>",
            Ty::IntGrid => "vector<vector<int>>",
            Ty::CharGrid => "vector<vector<char>>",
            Ty::StrGrid => "vector<vector<string>>",
            Ty::List => "ListNode*",
            Ty::Tree => "TreeNode*",
        }
    }
    fn scalar(self) -> bool {
        matches!(self, Ty::Int | Ty::Long | Ty::Double | Ty::Bool)
    }
    fn flat_array(self) -> bool {
        matches!(self, Ty::IntArr | Ty::LongArr | Ty::List)
    }
}

enum Item {
    Value(usize),
    Length(usize),
}
impl Io {
    fn param_types(&self) -> Vec<Ty> {
        self.params
            .iter()
            .map(|p| Ty::parse(&p.ty).unwrap())
            .collect()
    }
    fn index(&self, name: &str) -> Option<usize> {
        self.params.iter().position(|p| p.name == name)
    }
    /// The type the checker parses from stdout.
    pub fn output_type(&self) -> Ty {
        match self.output.as_str() {
            "return" => Ty::parse(&self.returns).unwrap(),
            name => Ty::parse(&self.params[self.index(name).unwrap()].ty).unwrap(),
        }
    }
    fn layout(&self) -> Option<Vec<Vec<Item>>> {
        if self.stdin.is_empty() {
            return None;
        }
        Some(
            self.stdin
                .iter()
                .map(|line| {
                    line.split_whitespace()
                        .map(|item| match item.strip_suffix(".length") {
                            Some(name) => Item::Length(self.index(name).unwrap()),
                            None => Item::Value(self.index(item).unwrap()),
                        })
                        .collect()
                })
                .collect(),
        )
    }
    pub fn validate(&self) -> Result<(), String> {
        if !is_identifier(&self.method) {
            return Err(format!("invalid method `{}`", self.method));
        }
        if self.params.is_empty() {
            return Err("at least one parameter is required".into());
        }
        for (i, p) in self.params.iter().enumerate() {
            if !is_identifier(&p.name) || self.params[..i].iter().any(|q| q.name == p.name) {
                return Err(format!("invalid or duplicate parameter `{}`", p.name));
            }
            Ty::parse(&p.ty)?;
        }
        if self.returns != "void" {
            Ty::parse(&self.returns)?;
        }
        match self.output.as_str() {
            "return" if self.returns == "void" => {
                return Err("`output` must name a parameter when `returns` is void".into())
            }
            "return" => {}
            name if self.index(name).is_none() => {
                return Err(format!("`output` names unknown parameter `{name}`"))
            }
            _ => {}
        }
        if !CHECKERS.contains(&self.checker.as_str()) {
            return Err(format!("unknown checker `{}`", self.checker));
        }
        if self.checker == "valid_pair"
            && (self.output_type() != Ty::IntArr
                || self.index("nums").is_none()
                || self.index("target").is_none())
        {
            return Err("valid_pair needs an int[] result and `nums`/`target` params".into());
        }
        let needs = |ty: Ty, param: &str, param_ty: Ty| {
            self.output_type() == ty
                && self
                    .index(param)
                    .is_some_and(|i| Ty::parse(&self.params[i].ty).ok() == Some(param_ty))
        };
        if self.checker == "longest_palindrome" && !needs(Ty::Str, "s", Ty::Str) {
            return Err("longest_palindrome needs a string result and a string `s`".into());
        }
        if self.checker == "balanced_bst" && !needs(Ty::Tree, "nums", Ty::IntArr) {
            return Err("balanced_bst needs a TreeNode result and an int[] `nums`".into());
        }
        if self.checker == "tokens" && self.output_type() != Ty::StrArr {
            return Err("tokens needs a string[] result".into());
        }
        if self.harness.as_deref().is_some_and(|h| {
            !h.ends_with(".cpp")
                || !h[..h.len() - 4]
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || "-_".contains(c))
        }) {
            return Err("harness must be a plain .cpp file name".into());
        }
        if !self.stdin.is_empty() {
            let types = self.param_types();
            let (mut values, mut lengths) = (vec![0; types.len()], vec![false; types.len()]);
            for item in self.stdin.iter().flat_map(|l| l.split_whitespace()) {
                let (name, length) = match item.strip_suffix(".length") {
                    Some(name) => (name, true),
                    None => (item, false),
                };
                let i = self
                    .index(name)
                    .ok_or_else(|| format!("stdin names unknown parameter `{name}`"))?;
                let ty = types[i];
                if length {
                    if !ty.flat_array() || lengths[i] || values[i] > 0 {
                        return Err(format!(
                            "`{item}` must be a 1-D array length read once before its values"
                        ));
                    }
                    lengths[i] = true;
                } else {
                    if !(ty.scalar() || ty.flat_array()) {
                        return Err(format!("`{name}` cannot appear in a custom stdin layout"));
                    }
                    if ty.flat_array() && !lengths[i] {
                        return Err(format!("`{name}.length` must come before `{name}`"));
                    }
                    values[i] += 1;
                }
            }
            if values.iter().any(|&n| n != 1) {
                return Err("custom stdin must read every parameter exactly once".into());
            }
        }
        Ok(())
    }
    /// Pick parameter `i` from a test's logical input.
    fn arg<'a>(&self, input: &'a Value, i: usize) -> Result<&'a Value, String> {
        if self.params.len() == 1 {
            Ok(input)
        } else {
            input
                .get(&self.params[i].name)
                .ok_or_else(|| format!("logicalInput is missing `{}`", self.params[i].name))
        }
    }
    /// Serialize a test's logical input to the documented stdin protocol.
    pub fn stdin_for(&self, input: &Value) -> Result<String, String> {
        let types = self.param_types();
        let mut out = String::new();
        match self.layout() {
            None => {
                for (i, ty) in types.iter().enumerate() {
                    write_value(*ty, self.arg(input, i)?, &mut out)?;
                }
            }
            Some(lines) => {
                for line in lines {
                    let mut words = vec![];
                    for item in line {
                        match item {
                            Item::Length(i) => {
                                words.push(array(self.arg(input, i)?)?.len().to_string())
                            }
                            Item::Value(i) => {
                                let v = self.arg(input, i)?;
                                if types[i].flat_array() {
                                    for x in array(v)? {
                                        words.push(scalar(types[i], x)?);
                                    }
                                } else {
                                    words.push(scalar(types[i], v)?);
                                }
                            }
                        }
                    }
                    out.push_str(&words.join(" "));
                    out.push('\n');
                }
            }
        }
        Ok(out)
    }
    pub fn check_expected(&self, expected: &Value) -> Result<(), String> {
        let mut sink = String::new();
        write_value(self.output_type(), expected, &mut sink)
    }
    /// Generate the C++ main() that reads stdin, calls the method, and prints the result.
    /// Shared reader/writer helpers plus List/Tree support, ending inside the
    /// open `judge_harness` namespace.
    fn helpers(&self) -> String {
        let mut helpers = String::from(HELPERS);
        if self.uses(Ty::List) {
            helpers += LIST_HELPERS;
        }
        if self.uses(Ty::Tree) {
            helpers += TREE_HELPERS;
        }
        helpers
    }
    /// Helpers followed by a hand-written main() (see `harness`).
    pub fn custom_main(&self, main: &str) -> String {
        format!("{}}}\n{main}", self.helpers())
    }
    pub fn cpp_main(&self) -> String {
        let types = self.param_types();
        let mut body = String::new();
        let var = |i: usize| format!("arg_{}", self.params[i].name);
        match self.layout() {
            None => {
                for (i, ty) in types.iter().enumerate() {
                    body += &format!("    {} {} = {};\n", ty.cpp(), var(i), reader(*ty));
                }
            }
            Some(lines) => {
                for (i, ty) in types.iter().enumerate() {
                    body += &format!("    {} {}{{}};\n", ty.cpp(), var(i));
                    if ty.flat_array() {
                        body += &format!("    size_t len_{} = 0;\n", self.params[i].name);
                    }
                }
                for item in lines.into_iter().flatten() {
                    body += &match item {
                        Item::Length(i) => {
                            format!("    len_{} = read_value<size_t>();\n", self.params[i].name)
                        }
                        Item::Value(i) if types[i] == Ty::List => format!(
                            "    {} = make_list(read_values<int>(len_{}));\n",
                            var(i),
                            self.params[i].name
                        ),
                        Item::Value(i) if types[i].flat_array() => format!(
                            "    {} = read_values<{}>(len_{});\n",
                            var(i),
                            if types[i] == Ty::LongArr {
                                "long long"
                            } else {
                                "int"
                            },
                            self.params[i].name
                        ),
                        Item::Value(i) => {
                            format!("    {} = read_value<{}>();\n", var(i), types[i].cpp())
                        }
                    };
                }
            }
        }
        let args = (0..types.len()).map(var).collect::<Vec<_>>().join(", ");
        let call = format!("Solution().{}({args})", self.method);
        body += &match self.output.as_str() {
            "return" => format!("    auto result = {call};\n    write(result);\n"),
            name => format!(
                "    {call};\n    write({});\n",
                var(self.index(name).unwrap())
            ),
        };
        let helpers = self.helpers();
        format!("{helpers}}}\nint main() {{\n    using namespace judge_harness;\n    cin >> boolalpha;\n{body}}}\n")
    }
    pub fn uses(&self, ty: Ty) -> bool {
        self.param_types().contains(&ty) || Ty::parse(&self.returns).ok() == Some(ty)
    }
    /// Compare program stdout with the expected value using the configured checker.
    pub fn accepts(&self, input: &Value, expected: &Value, output: &str) -> bool {
        let ty = self.output_type();
        let Some(actual) = parse_output(ty, output) else {
            return false;
        };
        match self.checker.as_str() {
            "valid_pair" => {
                let (Ok(nums), Some(target)) = (
                    self.arg(input, self.index("nums").unwrap()),
                    self.arg(input, self.index("target").unwrap())
                        .ok()
                        .and_then(Value::as_i64),
                ) else {
                    return false;
                };
                let (Some(nums), Some(pair)) = (nums.as_array(), actual.as_array()) else {
                    return false;
                };
                let at = |v: &Value| {
                    v.as_i64()
                        .filter(|&i| i >= 0 && (i as usize) < nums.len())
                        .and_then(|i| nums[i as usize].as_i64())
                };
                pair.len() == 2
                    && pair[0] != pair[1]
                    && matches!((at(&pair[0]), at(&pair[1])), (Some(a), Some(b)) if a + b == target)
            }
            "unordered" => same(&sorted(actual), &sorted(expected.clone())),
            "longest_palindrome" => {
                let (Some(s), Some(got), Some(want)) = (
                    self.arg(input, self.index("s").unwrap())
                        .ok()
                        .and_then(Value::as_str),
                    actual.as_str(),
                    expected.as_str(),
                ) else {
                    return false;
                };
                got.chars().count() == want.chars().count()
                    && s.contains(got)
                    && got.chars().eq(got.chars().rev())
            }
            "balanced_bst" => {
                let Some(nums) = self
                    .arg(input, self.index("nums").unwrap())
                    .ok()
                    .and_then(Value::as_array)
                else {
                    return false;
                };
                balanced_bst(&actual, nums)
            }
            "tokens" => match (actual.as_array(), expected.as_array()) {
                (Some(a), Some(b)) => {
                    a.len() == b.len()
                        && a.iter().zip(b).all(|(x, y)| {
                            let (x, y) = (x.as_str().unwrap_or(""), y.as_str().unwrap_or(""));
                            token_eq(x, y)
                        })
                }
                _ => false,
            },
            "unordered_nested" => same(
                &sorted(sort_inner(actual)),
                &sorted(sort_inner(expected.clone())),
            ),
            _ if ty == Ty::Tree => same(&actual, &trim_nulls(expected.clone())),
            _ => same(&actual, expected),
        }
    }
}
fn is_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}
fn reader(ty: Ty) -> String {
    match ty {
        Ty::Str => "read_line()".into(),
        Ty::IntArr => "read_array<int>(read_value<size_t>())".into(),
        Ty::LongArr => "read_array<long long>(read_value<size_t>())".into(),
        Ty::StrArr => "read_lines(read_value<size_t>())".into(),
        Ty::IntGrid => "read_int_grid()".into(),
        Ty::CharGrid => "read_char_grid()".into(),
        Ty::StrGrid => "read_string_grid()".into(),
        Ty::List => "make_list(read_array<int>(read_value<size_t>()))".into(),
        Ty::Tree => "make_tree(read_tokens(read_value<size_t>()))".into(),
        _ => format!("read_value<{}>()", ty.cpp()),
    }
}

// ---- stdin serialization ----
fn array(v: &Value) -> Result<&Vec<Value>, String> {
    v.as_array()
        .ok_or_else(|| format!("expected an array, got {v}"))
}
fn scalar(ty: Ty, v: &Value) -> Result<String, String> {
    let bad = || format!("{v} is not a valid {ty:?}");
    Ok(match ty {
        Ty::Int | Ty::IntArr | Ty::List => {
            let n = v.as_i64().ok_or_else(bad)?;
            i32::try_from(n).map_err(|_| bad())?.to_string()
        }
        Ty::Long | Ty::LongArr => v.as_i64().ok_or_else(bad)?.to_string(),
        Ty::Double => v.as_f64().ok_or_else(bad)?.to_string(),
        Ty::Bool => v.as_bool().ok_or_else(bad)?.to_string(),
        _ => return Err(bad()),
    })
}
fn line(v: &Value) -> Result<&str, String> {
    v.as_str()
        .filter(|s| !s.contains(['\n', '\r']))
        .ok_or_else(|| format!("{v} is not a single-line string"))
}
fn char_row(v: &Value) -> Result<String, String> {
    array(v)?
        .iter()
        .map(|c| match c.as_str() {
            Some(s) if s.chars().count() == 1 && !s.trim().is_empty() => Ok(s.to_string()),
            _ => Err(format!("{c} is not a single visible character")),
        })
        .collect()
}
fn write_value(ty: Ty, v: &Value, out: &mut String) -> Result<(), String> {
    let push_line = |out: &mut String, s: &str| {
        out.push_str(s);
        out.push('\n');
    };
    match ty {
        Ty::Int | Ty::Long | Ty::Double | Ty::Bool => push_line(out, &scalar(ty, v)?),
        Ty::Str => push_line(out, line(v)?),
        Ty::IntArr | Ty::LongArr | Ty::List => {
            let items = array(v)?;
            push_line(out, &items.len().to_string());
            let words: Result<Vec<_>, _> = items.iter().map(|x| scalar(ty, x)).collect();
            push_line(out, &words?.join(" "));
        }
        Ty::StrArr => {
            let items = array(v)?;
            push_line(out, &items.len().to_string());
            for s in items {
                push_line(out, line(s)?);
            }
        }
        Ty::IntGrid => {
            let rows = array(v)?;
            push_line(out, &rows.len().to_string());
            for row in rows {
                let row = array(row)?;
                let mut words = vec![row.len().to_string()];
                for x in row {
                    words.push(scalar(Ty::Int, x)?);
                }
                push_line(out, &words.join(" "));
            }
        }
        Ty::CharGrid => {
            let rows = array(v)?;
            push_line(out, &rows.len().to_string());
            for row in rows {
                push_line(out, &char_row(row)?);
            }
        }
        Ty::StrGrid => {
            let rows = array(v)?;
            push_line(out, &rows.len().to_string());
            for row in rows {
                let row = array(row)?;
                push_line(out, &row.len().to_string());
                for s in row {
                    push_line(out, line(s)?);
                }
            }
        }
        Ty::Tree => {
            let items = array(v)?;
            push_line(out, &items.len().to_string());
            let words: Result<Vec<_>, _> = items
                .iter()
                .map(|x| {
                    if x.is_null() {
                        Ok("null".into())
                    } else {
                        scalar(Ty::Int, x)
                    }
                })
                .collect();
            push_line(out, &words?.join(" "));
        }
    }
    Ok(())
}

// ---- stdout parsing and comparison ----
fn int(token: &str) -> Option<Value> {
    token.parse::<i64>().ok().map(Value::from)
}
fn parse_output(ty: Ty, output: &str) -> Option<Value> {
    let tokens = || output.split_whitespace();
    let one = || {
        let mut t = tokens();
        let first = t.next()?;
        t.next().is_none().then_some(first)
    };
    // Line-oriented types: a count line, then exactly that many structured lines.
    let mut lines = Lines(
        output
            .split('\n')
            .map(|l| l.strip_suffix('\r').unwrap_or(l)),
    );
    let value = match ty {
        Ty::Int | Ty::Long => int(one()?)?,
        Ty::Double => Value::from(one()?.parse::<f64>().ok().filter(|f| f.is_finite())?),
        Ty::Bool => Value::Bool(match one()? {
            "true" | "1" => true,
            "false" | "0" => false,
            _ => return None,
        }),
        Ty::Str => Value::String(
            output
                .strip_suffix('\n')
                .map(|s| s.strip_suffix('\r').unwrap_or(s))
                .unwrap_or(output)
                .to_string(),
        ),
        Ty::IntArr | Ty::LongArr | Ty::List => {
            Value::Array(tokens().map(int).collect::<Option<Vec<_>>>()?)
        }
        Ty::Tree => Value::Array(
            tokens()
                .map(|t| {
                    if t == "null" {
                        Some(Value::Null)
                    } else {
                        int(t)
                    }
                })
                .collect::<Option<Vec<_>>>()?,
        ),
        Ty::StrArr => {
            let n = lines.count()?;
            let items: Vec<Value> = (0..n)
                .map(|_| lines.next().map(|l| Value::String(l.into())))
                .collect::<Option<_>>()?;
            lines.rest_blank().then_some(Value::Array(items))?
        }
        Ty::IntGrid => {
            let n = lines.count()?;
            let rows: Vec<Value> = (0..n)
                .map(|_| {
                    let row = lines.next()?.split_whitespace().map(int);
                    Some(Value::Array(row.collect::<Option<_>>()?))
                })
                .collect::<Option<_>>()?;
            lines.rest_blank().then_some(Value::Array(rows))?
        }
        Ty::CharGrid => {
            let n = lines.count()?;
            let rows: Vec<Value> = (0..n)
                .map(|_| {
                    let row = lines.next()?.trim_end();
                    Some(Value::Array(
                        row.chars().map(|c| Value::String(c.into())).collect(),
                    ))
                })
                .collect::<Option<_>>()?;
            lines.rest_blank().then_some(Value::Array(rows))?
        }
        Ty::StrGrid => {
            let n = lines.count()?;
            let mut rows = vec![];
            for _ in 0..n {
                let k = lines.count()?;
                let row: Vec<Value> = (0..k)
                    .map(|_| lines.next().map(|l| Value::String(l.into())))
                    .collect::<Option<_>>()?;
                rows.push(Value::Array(row));
            }
            lines.rest_blank().then_some(Value::Array(rows))?
        }
    };
    Some(value)
}
struct Lines<I>(I);
impl<'a, I: Iterator<Item = &'a str>> Lines<I> {
    fn next(&mut self) -> Option<&'a str> {
        self.0.next()
    }
    fn count(&mut self) -> Option<usize> {
        self.0.next()?.trim().parse().ok()
    }
    fn rest_blank(&mut self) -> bool {
        self.0.all(|l| l.trim().is_empty())
    }
}
/// Level-order tree must be a height-balanced BST whose in-order walk is `nums`.
fn balanced_bst(tree: &Value, nums: &[Value]) -> bool {
    let Some(tokens) = tree.as_array() else {
        return false;
    };
    // Rebuild child links from LeetCode level order.
    let mut left = vec![None; tokens.len()];
    let mut right = vec![None; tokens.len()];
    let mut queue = vec![];
    if tokens.first().is_some_and(|v| !v.is_null()) {
        queue.push(0usize);
    } else if !tokens.is_empty() {
        return false;
    }
    let (mut head, mut next) = (0, 1);
    while head < queue.len() {
        let node = queue[head];
        head += 1;
        for side in [&mut left, &mut right] {
            if next < tokens.len() {
                if !tokens[next].is_null() {
                    side[node] = Some(next);
                    queue.push(next);
                }
                next += 1;
            }
        }
    }
    if next < tokens.len() {
        return false;
    }
    fn walk(
        i: Option<usize>,
        t: &[Value],
        l: &[Option<usize>],
        r: &[Option<usize>],
        out: &mut Vec<Value>,
    ) -> Option<i32> {
        let Some(i) = i else { return Some(0) };
        let a = walk(l[i], t, l, r, out)?;
        out.push(t[i].clone());
        let b = walk(r[i], t, l, r, out)?;
        ((a - b).abs() <= 1).then_some(1 + a.max(b))
    }
    let mut order = vec![];
    let root = (!queue.is_empty()).then_some(0);
    walk(root, tokens, &left, &right, &mut order).is_some() && order == nums
}
fn same(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) if x.is_f64() || y.is_f64() => {
            let (x, y) = (x.as_f64().unwrap(), y.as_f64().unwrap());
            (x - y).abs() <= DECIMAL_TOLERANCE
        }
        (Value::Array(x), Value::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(a, b)| same(a, b))
        }
        _ => a == b,
    }
}
fn sorted(v: Value) -> Value {
    match v {
        Value::Array(mut items) => {
            items.sort_by_cached_key(|x| x.to_string());
            Value::Array(items)
        }
        other => other,
    }
}
fn sort_inner(v: Value) -> Value {
    match v {
        Value::Array(items) => Value::Array(items.into_iter().map(sorted).collect()),
        other => other,
    }
}
fn trim_nulls(v: Value) -> Value {
    match v {
        Value::Array(mut items) => {
            while items.last().is_some_and(Value::is_null) {
                items.pop();
            }
            Value::Array(items)
        }
        other => other,
    }
}

// ---- C++ support code, emitted after the user's code ----
pub const LIST_STRUCT: &str = "struct ListNode {\n    int val;\n    ListNode* next;\n    inline static size_t judge_created = 0;\n    ListNode(int x = 0, ListNode* n = nullptr) : val(x), next(n) { ++judge_created; }\n};\n";
pub const TREE_STRUCT: &str = "struct TreeNode {\n    int val;\n    TreeNode* left;\n    TreeNode* right;\n    inline static size_t judge_created = 0;\n    TreeNode(int x = 0, TreeNode* l = nullptr, TreeNode* r = nullptr) : val(x), left(l), right(r) { ++judge_created; }\n};\n";
const HELPERS: &str = r#"#include <limits>
#include <iomanip>
#include <cstdlib>
namespace judge_harness {
using namespace std;
inline bool after_token = false;
[[noreturn]] inline void fail(const char* message) {
    cerr << message;
    exit(2);
}
template <class T> T read_value() {
    T value{};
    if (!(cin >> value)) fail("invalid judge input");
    after_token = true;
    return value;
}
template <class T> vector<T> read_values(size_t n) {
    vector<T> values(n);
    for (auto& value : values) value = read_value<T>();
    return values;
}
inline string read_line() {
    if (after_token) cin.ignore(numeric_limits<streamsize>::max(), '\n');
    after_token = false;
    string text;
    getline(cin, text);
    return text;
}
// Flat arrays have a values line even when empty. Grids use read_values instead:
// their row length and values share one line, so no extra line is consumed.
template <class T> vector<T> read_array(size_t n) {
    auto values = read_values<T>(n);
    if (n == 0) read_line();
    return values;
}
inline vector<string> read_lines(size_t n) {
    vector<string> lines(n);
    for (auto& line : lines) line = read_line();
    return lines;
}
inline vector<string> read_tokens(size_t n) { return read_array<string>(n); }
inline vector<vector<int>> read_int_grid() {
    vector<vector<int>> grid(read_value<size_t>());
    for (auto& row : grid) row = read_values<int>(read_value<size_t>());
    return grid;
}
inline vector<vector<char>> read_char_grid() {
    vector<vector<char>> grid(read_value<size_t>());
    for (auto& row : grid) {
        string text = read_line();
        row.assign(text.begin(), text.end());
    }
    return grid;
}
inline vector<vector<string>> read_string_grid() {
    vector<vector<string>> grid(read_value<size_t>());
    for (auto& row : grid) row = read_lines(read_value<size_t>());
    return grid;
}
inline void write(int v) { cout << v << '\n'; }
inline void write(long long v) { cout << v << '\n'; }
inline void write(double v) { cout << setprecision(12) << v << '\n'; }
inline void write(bool v) { cout << (v ? "true" : "false") << '\n'; }
inline void write(const string& v) { cout << v << '\n'; }
template <class T> void write_row(const vector<T>& v) {
    for (size_t i = 0; i < v.size(); ++i) cout << (i ? " " : "") << v[i];
    cout << '\n';
}
inline void write(const vector<int>& v) { write_row(v); }
inline void write(const vector<long long>& v) { write_row(v); }
inline void write(const vector<string>& v) {
    cout << v.size() << '\n';
    for (auto& s : v) cout << s << '\n';
}
inline void write(const vector<vector<int>>& v) {
    cout << v.size() << '\n';
    for (auto& row : v) write_row(row);
}
inline void write(const vector<vector<char>>& v) {
    cout << v.size() << '\n';
    for (auto& row : v) cout << string(row.begin(), row.end()) << '\n';
}
inline void write(const vector<vector<string>>& v) {
    cout << v.size() << '\n';
    for (auto& row : v) write(row);
}
"#;
const LIST_HELPERS: &str = r#"inline ListNode* make_list(const vector<int>& values) {
    ListNode dummy;
    ListNode* tail = &dummy;
    for (int value : values) tail = tail->next = new ListNode(value);
    return dummy.next;
}
inline void write(ListNode* node) {
    // Every node was constructed somewhere; visiting more means a cycle.
    vector<int> values;
    for (; node; node = node->next) {
        if (values.size() >= ListNode::judge_created) fail("invalid cyclic/long list");
        values.push_back(node->val);
    }
    write_row(values);
}
"#;
const TREE_HELPERS: &str = r#"inline TreeNode* make_tree(const vector<string>& tokens) {
    if (tokens.empty() || tokens[0] == "null") return nullptr;
    auto node = [&](size_t i) { return i < tokens.size() && tokens[i] != "null" ? new TreeNode(stoi(tokens[i])) : nullptr; };
    TreeNode* root = node(0);
    vector<TreeNode*> queue{root};
    size_t next = 1;
    for (size_t head = 0; head < queue.size() && next < tokens.size(); ++head) {
        queue[head]->left = node(next++);
        queue[head]->right = node(next++);
        for (TreeNode* child : {queue[head]->left, queue[head]->right})
            if (child) queue.push_back(child);
    }
    return root;
}
inline void write(TreeNode* root) {
    vector<string> tokens;
    vector<TreeNode*> queue{root};
    for (size_t head = 0; head < queue.size(); ++head) {
        if (queue.size() > 2 * TreeNode::judge_created + 1) fail("invalid cyclic/shared tree");
        TreeNode* node = queue[head];
        if (!node) {
            tokens.push_back("null");
            continue;
        }
        tokens.push_back(to_string(node->val));
        queue.push_back(node->left);
        queue.push_back(node->right);
    }
    while (!tokens.empty() && tokens.back() == "null") tokens.pop_back();
    write_row(tokens);
}
"#;

/// Answers within 1e-5 count, as on LeetCode. The tolerance is absolute, so a median
/// of large values that is off by 0.5 is still wrong.
const DECIMAL_TOLERANCE: f64 = 1e-5;

/// Integer tokens must match exactly (an overflow or off-by-one is wrong however
/// large the value); decimals use the absolute tolerance; anything else compares as text.
fn token_eq(x: &str, y: &str) -> bool {
    if let (Ok(p), Ok(q)) = (x.parse::<i128>(), y.parse::<i128>()) {
        return p == q;
    }
    match (x.parse::<f64>(), y.parse::<f64>()) {
        (Ok(p), Ok(q)) => (p - q).abs() <= DECIMAL_TOLERANCE,
        _ => x == y,
    }
}

#[cfg(test)]
mod tests {
    use super::token_eq;

    #[test]
    fn integer_tokens_match_exactly_and_decimals_use_absolute_tolerance() {
        assert!(token_eq("-2147483648", "-2147483648"));
        assert!(!token_eq("-2147483649", "-2147483648"));
        assert!(!token_eq("1440956708", "1440956707"));
        assert!(!token_eq("-100000.5", "-100000.0"));
        assert!(token_eq("-100000", "-100000.0"));
        assert!(token_eq("2.500001", "2.5"));
        assert!(token_eq("null", "null"));
        assert!(!token_eq("true", "false"));
    }
}
