// Read from stdin (never argv); save via the currently selected credential storage.
use std::io::{self, BufRead};
fn main() {
    let provider = std::env::args().nth(1).unwrap_or_else(|| "deepseek".into());
    let mut key = String::new();
    io::stdin().lock().read_line(&mut key).expect("stdin");
    match swe_interview_prep::credentials::save_key(&provider, &key) {
        Ok(()) => {
            println!("API Key 已保存到当前测试用 .env（未输出 Key，未写入 SQLite 或 Keychain）")
        }
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}
