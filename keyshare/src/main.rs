use base64::prelude::*;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce,
};
use nn_secret_share;
use serde::{Deserialize, Serialize};
use std::fs;
use std::{env, num::ParseIntError};

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
#[derive(Serialize, Deserialize)]
struct CipherIv {
    ciphertext: String,
    nonce: String,
}

fn encode(in_contents: &String, num_shares: usize) -> Result<(String, String, Vec<String>), Error> {
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(&nonce, in_contents.as_bytes())?;

    let enc_keys = nn_secret_share::encode(&key.as_ref(), num_shares)?;
    let nonce = BASE64_STANDARD.encode(nonce);
    let ciphertext = BASE64_STANDARD.encode(ciphertext);

    let mut b64_keys_list = Vec::new();
    for i in 0..num_shares {
        b64_keys_list.push(BASE64_STANDARD.encode(&enc_keys[i]));
    }

    Ok((nonce, ciphertext, b64_keys_list))
}

fn decode(
    nonce: &String,
    ciphertext: &String,
    b64_keys_list: &Vec<String>,
) -> Result<String, Error> {
    let mut keys_list = Vec::new();
    for i in 0..b64_keys_list.len() {
        keys_list.push(BASE64_STANDARD.decode(b64_keys_list[i].trim())?);
    }

    let nonce = BASE64_STANDARD.decode(nonce)?;
    let ciphertext = BASE64_STANDARD.decode(ciphertext)?;
    let dec_keys = nn_secret_share::decode(&keys_list)?;
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&dec_keys));

    let plaintext =
        String::from_utf8(cipher.decrypt(Nonce::from_slice(&nonce), ciphertext.as_ref())?)?;

    Ok(plaintext)
}
fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Error::InvalidArgError(format!(
            "Must provide a subcommand argument"
        )));
    }
    let kind = &args[1];
    if kind == "encode" {
        if args.len() != 6 {
            return Err(Error::InvalidArgError(format!(
                "Must provide exactly 5 arguments. Provided {}",
                args.len() - 1
            )));
        }
        let infile = &args[2];
        let num_shares: usize = args[3].parse()?;
        let key_name = &args[4];
        let out_path = &args[5];
        let in_contents = fs::read_to_string(infile)?;

        let (nonce, ciphertext, keys_vec) = encode(&in_contents, num_shares)?;

        if !(fs::exists(out_path)?) {
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
            fs::write(format!("{out_path}/{key_name}{i}"), &keys_vec[i])?;
        }
    } else {
        if kind == "decode" {
            if args.len() != 6 {
                return Err(Error::InvalidArgError(format!(
                    "Must provide exactly 5 arguments. Provided {}",
                    args.len() - 1
                )));
            }
            let in_path = &args[2];
            let file_name = &args[3];
            let num_files = args[4].parse()?;
            let out_path = &args[5];

            let cipher_iv_string = fs::read_to_string(format!("{in_path}/cipher_iv"))?;

            let cipher_iv: CipherIv = serde_json::from_str(&cipher_iv_string)?;

            let nonce = cipher_iv.nonce;
            let ciphertext = cipher_iv.ciphertext;

            let mut keys_vec = Vec::new();
            let mut temp_file;
            for i in 0..num_files {
                temp_file = fs::read_to_string(format!("{in_path}/{file_name}{i}"))?;
                keys_vec.push(temp_file);
            }

            let plaintext = decode(&nonce, &ciphertext, &keys_vec)?;
            fs::write(out_path, plaintext)?;
        } else {
            return Err(Error::InvalidArgError(format!(
                "The subcommand argument must be encode/decode"
            )));
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
        dbg!(t.metadata())?;
        let plaintext = String::from("test");

        writeln!(t, "{}", plaintext)?;
        let num_shares = 5;
        let (nonce, ciphertext, keys_vec) = encode(&plaintext, num_shares)?;
        let decrypted = decode(&nonce, &ciphertext, &keys_vec)?;
        assert_eq!(plaintext, decrypted);

        Ok(())
    }
}
