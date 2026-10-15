//! One-time, explicit user-authorized migration from our Keychain account to local .env.
use std::io::Write;
use swe_interview_prep::credentials;
fn main(){if let Err(e)=run(){eprintln!("迁移未完成：{e}");std::process::exit(1);}}
fn run()->Result<(),String>{
    let path=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().ok_or("project path")?.join(".env");
    let key=credentials::read_key("deepseek")?;
    let mut options=std::fs::OpenOptions::new();options.write(true).create_new(true);
    #[cfg(unix)] {use std::os::unix::fs::OpenOptionsExt;options.mode(0o600);}
    let mut file=options.open(&path).map_err(|_|"无法创建 .env（已存在时不会覆盖）")?;
    let content=format!("DEEPSEEK_API_KEY={}\n",key.trim());
    file.write_all(content.as_bytes()).map_err(|_|"写入 .env 失败，系统凭据仍保留")?;
    file.sync_all().map_err(|_|"同步 .env 失败，系统凭据仍保留")?;
    if std::fs::read(&path).map_err(|_|"回读 .env 失败")?!=content.as_bytes(){return Err(".env 校验失败，未删除系统凭据".into());}
    credentials::delete_key("deepseek")?;
    println!("已转存至项目 .env（0600），并删除此前 DeepSeek 系统凭据；未输出 Key");
    Ok(())
}
