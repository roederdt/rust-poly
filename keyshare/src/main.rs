use base64::prelude::*;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce,
};
use clap::{Parser, Subcommand};
use nn_secret_share::{self, XorSharer};
use poly::{
    SecretSharer, ShamirSharer,
    Share::{self},
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::num::ParseIntError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Parsing int error {0}")]
    ParsingError(#[from] ParseIntError),
    #[error("Invalid argument:{0}")]
    InvalidArgError(String),
    #[error("ChaCha error: {0}")]
    ChaChaError(#[from] chacha20poly1305::Error),
    #[error("Base64 error: {0}")]
    Base64Error(#[from] base64::DecodeError),
    #[error("NNShare decode error: {0}")]
    NnError(#[from] nn_secret_share::Error),
    #[error("UTF8 decode error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("serde_json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    threshold: Option<u8>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encode {
        infile: String,
        num_shares: usize,
        key_name: String,
        out_path: String,
    },
    Decode {
        in_path: String,
        file_name: String,
        num_files: usize,
        out_path: String,
    },
}
#[derive(Serialize, Deserialize)]
struct CipherIv {
    ciphertext: String,
    nonce: String,
}

fn encode(
    in_contents: &String,
    num_shares: usize,
    threshold: u8,
) -> Result<(String, String, Vec<String>), Error> {
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(&nonce, in_contents.as_bytes())?;

    let enc_keys;
    if threshold == 0 {
        let t = XorSharer::new(num_shares);
        enc_keys = t.encode(&key.as_ref()).unwrap();
    } else {
        let t = poly::ShamirSharer::new(num_shares, threshold);
        enc_keys = t.encode(&key.as_ref()).unwrap();
    }
    let nonce = BASE64_STANDARD.encode(nonce);
    let ciphertext = BASE64_STANDARD.encode(ciphertext);

    let mut b64_keys_list = Vec::new();
    if threshold == 0 {
        for i in 0..num_shares {
            b64_keys_list.push(serde_json::to_string(&enc_keys[i])?);
        }
    } else {
        for i in 0..num_shares {
            b64_keys_list.push(serde_json::to_string(&enc_keys[i])?);
        }
    }

    Ok((nonce, ciphertext, b64_keys_list))
}

fn decode(
    nonce: &String,
    ciphertext: &String,
    b64_keys_list: &Vec<String>,
    kind: &String,
) -> Result<String, Error> {
    let mut keys_list: Vec<Share> = Vec::new();
    for i in 0..b64_keys_list.len() {
        keys_list.push(serde_json::from_str(&b64_keys_list[i].trim())?);
    }

    let nonce = BASE64_STANDARD.decode(nonce)?;
    let ciphertext = BASE64_STANDARD.decode(ciphertext)?;

    let m;
    if kind == "nn" {
        let t = XorSharer::new(keys_list.len());
        m = t.decode(&keys_list).unwrap();
    } else {
        let t = ShamirSharer::new(keys_list.len(), keys_list.len() as u8);
        m = t.decode(&keys_list).unwrap();
    }
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&m));

    let plaintext =
        String::from_utf8(cipher.decrypt(Nonce::from_slice(&nonce), ciphertext.as_ref())?)?;

    Ok(plaintext)
}
fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Encode {
            infile,
            num_shares,
            key_name,
            out_path,
        } => {
            let in_contents = fs::read_to_string(infile)?;

            let (nonce, ciphertext, keys_vec) = encode(&in_contents, num_shares, 3)?;
            // cli.threshold.unwrap_or(0)
            if !(fs::exists(&out_path)?) {
                return Err(Error::InvalidArgError(String::from(
                    "Directory for output provided does not exists",
                )));
            }

            let iv_cipher = CipherIv { nonce, ciphertext };
            fs::write(
                format!("{out_path}/cipher_iv"),
                serde_json::to_string(&iv_cipher)?,
            )?;

            for i in 0..keys_vec.len() {
                fs::write(format!("{out_path}/{key_name}_{i}"), &keys_vec[i])?;
            }
        }

        Commands::Decode {
            in_path,
            file_name,
            num_files,
            out_path,
        } => {
            let cipher_iv_string = fs::read_to_string(format!("{in_path}/cipher_iv"))?;

            let cipher_iv: CipherIv = serde_json::from_str(&cipher_iv_string)?;

            let nonce = cipher_iv.nonce;
            let ciphertext = cipher_iv.ciphertext;

            let mut keys_vec = Vec::new();
            let mut temp_file;
            for i in 0..num_files {
                temp_file = fs::read_to_string(format!("{in_path}/{file_name}_{i}"))?;
                keys_vec.push(temp_file);
            }
            let kind: String;
            match cli.threshold {
                Some(_) => kind = String::from("t"),
                _ => kind = String::from("nn"),
            }

            let plaintext = decode(&nonce, &ciphertext, &keys_vec, &kind)?;
            fs::write(out_path, plaintext)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempfile;
    #[test]
    fn encode_and_decode() -> Result<(), Error> {
        let mut t = tempfile()?;
        let plaintext = String::from("Hello, World!");

        writeln!(t, "{}", plaintext)?;
        let num_shares = 5;
        let (nonce, ciphertext, keys_vec) = encode(&plaintext, num_shares, 3)?;
        let decrypted = decode(&nonce, &ciphertext, &keys_vec, &String::from("t"))?;
        assert_eq!(plaintext, decrypted);

        Ok(())
    }
}
