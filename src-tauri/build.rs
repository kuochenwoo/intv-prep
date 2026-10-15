fn main() {
    embed_problems();
    #[cfg(feature = "desktop")]
    tauri_build::build()
}
// Embed every problem JSON so adding a problem needs no Rust change.
fn embed_problems() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../content/leetcode/problems");
    println!("cargo:rerun-if-changed={}", dir.display());
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .expect("content/leetcode/problems")
        .map(|e| e.expect("problem dir entry").path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    files.sort();
    let mut out = String::from("static PROBLEM_FILES: &[(&str, &str)] = &[\n");
    for path in files {
        let path = path.canonicalize().expect("problem path");
        let name = path.file_name().unwrap().to_string_lossy();
        out += &format!(
            "    ({name:?}, include_str!({:?})),\n",
            path.display().to_string()
        );
    }
    out += "];\n";
    let dest = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("problem_files.rs");
    std::fs::write(dest, out).expect("write problem_files.rs");
}
