// Read from stdin (never argv/environment/files); only write the named OS credential.
use std::io::{self, BufRead};
fn main() {
    let provider = std::env::args().nth(1).unwrap_or_else(|| "deepseek".into());
    let mut key = String::new();
    io::stdin().lock().read_line(&mut key).expect("stdin");
    match swe_interview_prep::credentials::save_key(&provider, &key) {
        Ok(()) => println!("API Key 已保存到系统凭据存储（未写入普通配置或数据库）"),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}
