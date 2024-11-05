use base64::prelude::*;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce,
};
use nn_secret_share;
use std::fs;
use std::{env, num::ParseIntError};

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("Parsing int error {0}")]
    ParsingError(#[from] ParseIntError),
    #[error("Invalid argument")]
    InvalidArgError,
    #[error("ChaCha error: {0}")]
    ChaChaError(#[from] chacha20poly1305::Error),
    #[error("Base64 error: {0}")]
    Base64Error(#[from] base64::DecodeError),
    #[error("NNShare decode error: {0}")]
    NNError(#[from] nn_secret_share::Error),
    #[error("UTF8 decode error: {0}")]
    UTF8Error(#[from] std::string::FromUtf8Error),
}
fn encode(infile: &String, num_shares: usize) -> Result<(String, String, Vec<String>), Error> {
    let in_contents = fs::read_to_string(infile).expect(&format!("{infile}"));
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(&nonce, in_contents.as_bytes())?;

    let enc_keys = nn_secret_share::encode(&key, num_shares)?;
    let nonce = BASE64_STANDARD.encode(nonce);
    let ciphertext = BASE64_STANDARD.encode(ciphertext);
    let mut b64_keys_list = Vec::new();
    for i in 0..num_shares {
        b64_keys_list.push(BASE64_STANDARD.encode(enc_keys[i].clone()));
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
        keys_list.push(BASE64_STANDARD.decode(b64_keys_list[i].clone())?);
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
    assert_eq!(BASE64_STANDARD.decode(b"+uwgVQA=")?, b"\xFA\xEC\x20\x55\0");
    assert_eq!(BASE64_STANDARD.encode(b"\xFF\xEC\x20\x55\0"), "/+wgVQA=");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Error::InvalidArgError);
    }
    let kind = &args[1];
    if kind == "encode" {
        if args.len() != 6 {
            return Err(Error::InvalidArgError);
        }
        let infile = &args[2];
        let num_shares: usize = args[3].parse()?;
        let key_name = &args[4];
        let outfile = &args[5];

        let (nonce, ciphertext, keys_vec) = encode(infile, num_shares)?;
        if !(fs::exists(format!("./target/debug/{key_name}")).unwrap()) {
            fs::create_dir(format!("./target/debug/{key_name}")).unwrap();
        }
        fs::write(format!("./target/debug/{key_name}/nonce"), nonce).expect("Unable to write file");
        fs::write(format!("./target/debug/{key_name}/{outfile}"), ciphertext)
            .expect("Unable to write file");
        for i in 0..keys_vec.len() {
            fs::write(
                format!("./target/debug/{key_name}/{key_name}{i}"),
                keys_vec[i].clone(),
            )
            .expect("Unable to write file");
        }
    } else {
        if kind == "decode" {
            let infile = &args[2];
            let ciphertext = fs::read_to_string(&args[3]).expect(&format!("{infile}")); // derived from in_contents(not done yet)
            let file_name = &args[4];
            let num_files = args[5].parse()?;
            let nonce = fs::read_to_string(format!("./target/debug/{file_name}/nonce"))
                .expect(&format!("{infile}"));
            let mut keys_vec = Vec::new();
            let mut temp_file;
            for i in 0..num_files {
                temp_file =
                    fs::read_to_string(format!("./target/debug/{file_name}/{file_name}{i}"))
                        .expect(&format!("{infile}"));
                keys_vec.push(temp_file);
            }

            let plaintext = decode(&nonce, &ciphertext, &keys_vec)?;
            fs::write("./target/debug/plaintext", plaintext).expect("Unable to write file");
        } else {
            return Err(Error::InvalidArgError);
        }
    }
    Ok(())
}
