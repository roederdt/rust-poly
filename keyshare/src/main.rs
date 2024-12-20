use base64::prelude::*;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce,
};
use clap::{Parser, Subcommand};
use nn_secret_share::{self, XorSharer};
use poly::{
    new_from_slice, SecretSharer, ShamirSharer,
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
    /// Encrypts a secret and produces shares of the key
    Encode {
        /// The file with the secret to encrypt
        in_path: String,
        /// What you want to call the share files
        share_prefix: String,
        /// How many shares to create
        num_shares: usize,
        /// The existing directory where the files will go
        out_path: String,
    },
    /// Decrypts a secret from the ciphertext and shares of the key
    Decode {
        /// The directory where the shares and cipher_iv file are
        in_path: String,
        /// Prefix for share files 0 through num_shares - 1
        share_prefix: String,
        /// How many shares to read, from 0 through num_shares - 1`
        num_shares: usize,
        /// Where to write the decrypted secret
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
) -> Result<(String, String, Vec<(String, Option<String>)>), Error> {
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
    for i in 0..num_shares {
        b64_keys_list.push(enc_keys[i].to_b64());
    }

    Ok((nonce, ciphertext, b64_keys_list))
}

fn decode(
    nonce: &String,
    ciphertext: &String,
    b64_keys_list: &Vec<(String, Option<String>)>,
    threshold: u8,
) -> Result<String, Error> {
    let mut keys_list: Vec<Share> = Vec::new();
    if threshold == 0 {
        for i in 0..b64_keys_list.len() {
            let t = &b64_keys_list[i].0;

            keys_list.push(Share::XorShare(BASE64_STANDARD.decode(t)?.to_vec()));
        }
    } else {
        for i in 0..b64_keys_list.len() {
            let t1 = poly::GF2256::new(&new_from_slice(
                &BASE64_STANDARD.decode(&b64_keys_list[i].0)?,
            ));

            let t2 = poly::GF2256::new(&new_from_slice(
                &BASE64_STANDARD.decode(&b64_keys_list[i].1.clone().unwrap())?,
            ));

            keys_list.push(Share::ShamirShare { x: t1, y: t2 });
        }
    }

    let nonce = BASE64_STANDARD.decode(nonce)?;
    let ciphertext = BASE64_STANDARD.decode(ciphertext)?;

    let m;
    if threshold == 0 {
        let t = XorSharer::new(keys_list.len());
        m = t.decode(&keys_list).unwrap();
    } else {
        let t = ShamirSharer::new(keys_list.len(), threshold);
        m = t.decode(&keys_list).unwrap();
    }

    let cipher = ChaCha20Poly1305::new(Key::from_slice(&m));

    let plaintext =
        String::from_utf8(cipher.decrypt(Nonce::from_slice(&nonce), ciphertext.as_ref())?)?;

    Ok(plaintext)
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let threshold = cli.threshold.unwrap_or(0);
    match cli.command {
        Commands::Encode {
            in_path,
            num_shares,
            share_prefix,
            out_path,
        } => {
            if num_shares < threshold.into() {
                return Err(Error::InvalidArgError(String::from(format!(
                    "Threshold({threshold}) cannot be greater than the number of shares({num_shares})",
                ))));
            }
            let in_contents = fs::read_to_string(in_path)?;

            let (nonce, ciphertext, keys_vec) = encode(&in_contents, num_shares, threshold)?;

            if !(fs::exists(&out_path)?) {
                return Err(Error::InvalidArgError(String::from(
                    "Directory for output provided does not exist",
                )));
            }

            let iv_cipher = CipherIv { nonce, ciphertext };
            fs::write(
                format!("{out_path}/cipher_iv"),
                serde_json::to_string(&iv_cipher)?,
            )?;

            let mut keystring_vec = Vec::new();
            for i in 0..keys_vec.len() {
                keystring_vec.push(String::from(&keys_vec[i].0));
            }

            match &keys_vec[0].1 {
                Some(_) => {
                    for i in 0..keys_vec.len() {
                        keystring_vec[i] +=
                            format!(" {}", (&keys_vec[i].1).clone().unwrap()).as_str();
                    }
                }
                None => {}
            }

            for i in 0..keys_vec.len() {
                fs::write(
                    format!("{out_path}/{share_prefix}_{i}"),
                    format!(" {}", keystring_vec[i]),
                )?;
            }
        }

        Commands::Decode {
            in_path,
            share_prefix,
            num_shares,
            out_path,
        } => {
            if num_shares < threshold.into() {
                return Err(Error::InvalidArgError(String::from(format!(
                    "Threshold({threshold}) cannot be greater than the number of shares({num_shares})",
                ))));
            }

            let cipher_iv_string = fs::read_to_string(format!("{in_path}/cipher_iv"))?;
            let cipher_iv: CipherIv = serde_json::from_str(&cipher_iv_string)?;

            let nonce = cipher_iv.nonce;
            let ciphertext = cipher_iv.ciphertext;

            let mut keys_vec = Vec::new();
            let mut temp_file;
            match cli.threshold {
                Some(_) => {
                    for i in 0..num_shares {
                        temp_file = fs::read_to_string(format!("{in_path}/{share_prefix}_{i}"))?;
                        let temp_file: Vec<&str> = temp_file.split(" ").collect();
                        keys_vec
                            .push((String::from(temp_file[1]), Some(String::from(temp_file[2]))));
                    }
                }
                None => {
                    for i in 0..num_shares {
                        temp_file = fs::read_to_string(format!("{in_path}/{share_prefix}_{i}"))?;
                        keys_vec.push((temp_file, None));
                    }
                }
            }

            let plaintext = decode(&nonce, &ciphertext, &keys_vec, threshold)?;
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
        let decrypted = decode(&nonce, &ciphertext, &keys_vec, 3)?;
        assert_eq!(plaintext, decrypted);

        Ok(())
    }
}
