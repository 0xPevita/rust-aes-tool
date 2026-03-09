use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use clap::{Parser, Subcommand};
use std::fs;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

#[derive(Parser)]
#[command(name = "aes-tool", about = "AES-256-GCM file encryption/decryption")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encrypt {
        #[arg(short, long)] input: String,
        #[arg(short, long)] output: String,
        #[arg(short, long)] key: String,
    },
    Decrypt {
        #[arg(short, long)] input: String,
        #[arg(short, long)] output: String,
        #[arg(short, long)] key: String,
    },
    Keygen,
}

fn encrypt(input: &str, output: &str, key_b64: &str) -> Result<(), Box<dyn std::error::Error>> {
    let key_bytes = BASE64.decode(key_b64)?;
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_bytes));
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ct = cipher.encrypt(&nonce, fs::read(input)?.as_ref()).map_err(|e| format!("{e}"))?;
    let mut out = nonce.to_vec();
    out.extend_from_slice(&ct);
    fs::write(output, &out)?;
    println!("✅ Encrypted: {} → {} ({} bytes)", input, output, out.len());
    Ok(())
}

fn decrypt(input: &str, output: &str, key_b64: &str) -> Result<(), Box<dyn std::error::Error>> {
    let key_bytes = BASE64.decode(key_b64)?;
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_bytes));
    let data = fs::read(input)?;
    let (nonce_b, ct) = data.split_at(12);
    let pt = cipher.decrypt(Nonce::from_slice(nonce_b), ct)
        .map_err(|_| "Wrong key or corrupt file")?;
    fs::write(output, &pt)?;
    println!("✅ Decrypted: {} → {} ({} bytes)", input, output, pt.len());
    Ok(())
}

fn main() {
    let result = match Cli::parse().command {
        Commands::Keygen => {
            let k = Aes256Gcm::generate_key(OsRng);
            println!("🔑 Key: {}", BASE64.encode(&k));
            Ok(())
        }
        Commands::Encrypt { input, output, key } => encrypt(&input, &output, &key),
        Commands::Decrypt { input, output, key } => decrypt(&input, &output, &key),
    };
    if let Err(e) = result { eprintln!("❌ {}", e); }
}
