fn main() {
    embed_problems();
    #[cfg(feature = "desktop")]
    tauri_build::build()
}
// Embed every problem JSON and custom harness so adding a problem needs no Rust change.
fn embed_problems() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../content/leetcode");
    let mut out = String::new();
    for (name, dir, ext) in [
        ("PROBLEM_FILES", "problems", "json"),
        ("HARNESS_FILES", "harnesses", "cpp"),
    ] {
        let dir = root.join(dir);
        println!("cargo:rerun-if-changed={}", dir.display());
        let mut files: Vec<_> = std::fs::read_dir(&dir)
            .map(|entries| {
                entries
                    .map(|e| e.expect("content dir entry").path())
                    .filter(|p| p.extension().is_some_and(|x| x == ext))
                    .collect()
            })
            .unwrap_or_default();
        files.sort();
        out += &format!("static {name}: &[(&str, &str)] = &[\n");
        for path in files {
            let path = path.canonicalize().expect("content path");
            let file = path.file_name().unwrap().to_string_lossy();
            out += &format!(
                "    ({file:?}, include_str!({:?})),\n",
                path.display().to_string()
            );
        }
        out += "];\n";
    }
    let dest = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("problem_files.rs");
    std::fs::write(dest, out).expect("write problem_files.rs");
}
