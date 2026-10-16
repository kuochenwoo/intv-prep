//! Verify the same minisign payload and version binding required by the updater.
use base64::Engine;
use minisign_verify::{PublicKey, Signature};
use std::{env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        return Err("usage: verify_update <tauri-config> <archive> <signature> <version>".into());
    }
    let config: serde_json::Value = serde_json::from_slice(&fs::read(&args[1])?)?;
    let key = config["plugins"]["updater"]["pubkey"].as_str().ok_or("missing updater public key")?;
    let decode = |text: &str| -> Result<String, Box<dyn Error>> {
        Ok(String::from_utf8(base64::engine::general_purpose::STANDARD.decode(text.trim())?)?)
    };
    let public_key = PublicKey::decode(&decode(key)?)?;
    let signature = Signature::decode(&decode(&fs::read_to_string(&args[3])?)?)?;
    public_key.verify(&fs::read(&args[2])?, &signature, true)?;
    let version = signature.trusted_comment().split('\t').find_map(|field| field.strip_prefix("version:"));
    if version != Some(args[4].as_str()) {
        return Err("signature is not bound to the requested version".into());
    }
    println!("Updater signature and signed version verified.");
    Ok(())
}
