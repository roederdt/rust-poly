use base64::prelude::*;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce,
};
use nn_secret_share;
use std::{env, num::ParseIntError};
use std::{fs, str::Utf8Error};

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
fn encode(infile: &String, num_shares: usize) -> Result<(String, String, Vec<String>, Key), Error> {
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

    Ok((nonce, ciphertext, b64_keys_list, key))
}

fn decode(
    nonce: &String,
    ciphertext: &String,
    b64_keys_list: &Vec<String>,
    key: &Key,
) -> Result<String, Error> {
    let mut keys_list = Vec::new();
    for i in 0..b64_keys_list.len() {
        keys_list.push(BASE64_STANDARD.decode(b64_keys_list[i].clone())?);
    }
    let nonce = BASE64_STANDARD.decode(nonce)?;
    let ciphertext = BASE64_STANDARD.decode(ciphertext)?;
    let dec_keys = nn_secret_share::decode(&keys_list)?;

    let cipher = ChaCha20Poly1305::new(Key::from_slice(&dec_keys));
    assert_eq!(&dec_keys, key.as_slice());
    let plaintext =
        String::from_utf8(cipher.decrypt(Nonce::from_slice(&nonce), ciphertext.as_ref())?)?;

    Ok(plaintext)
}
fn main() -> Result<(), Error> {
    assert_eq!(BASE64_STANDARD.decode(b"+uwgVQA=")?, b"\xFA\xEC\x20\x55\0");
    assert_eq!(BASE64_STANDARD.encode(b"\xFF\xEC\x20\x55\0"), "/+wgVQA=");

    let args: Vec<String> = env::args().collect();

    if args.len() != 6 {
        return Err(Error::InvalidArgError);
    }
    let infile = &args[1];
    let num_shares: usize = args[2].parse()?;
    let key_name = &args[3];
    let outfile = &args[4];
    let kind = &args[5];

    let in_contents = fs::read_to_string(infile).expect(&format!("{infile}"));

    let (nonce, ciphertext, keys_vec, key) = encode(infile, num_shares)?;

    let plaintext = decode(&nonce, &ciphertext, &keys_vec, &key)?;
    assert_eq!(&plaintext, &in_contents);
    fs::write("./target/debug/plaintext", plaintext).expect("Unable to write file");
    fs::write("./target/debug/ciphertext_output", ciphertext).expect("Unable to write file");
    Ok(())
}
