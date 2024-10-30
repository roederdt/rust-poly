use base64::prelude::*;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use nn_secret_share;
use std::fs;
use std::{env, num::ParseIntError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Parsing int error {0}")]
    ParsingError(#[from] ParseIntError),
    #[error("Invalid argument")]
    InvalidArgError,
    #[error("ChaCha error: {0}")]
    ChaChaError(#[from] chacha20poly1305::Error),
    #[error("Base64 error: {0}")]
    Base64Error(#[from] base64::DecodeError),
}
fn main() -> Result<(), Error> {
    assert_eq!(BASE64_STANDARD.decode(b"+uwgVQA=")?, b"\xFA\xEC\x20\x55\0");
    assert_eq!(BASE64_STANDARD.encode(b"\xFF\xEC\x20\x55\0"), "/+wgVQA=");

    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        return Err(Error::InvalidArgError);
    }
    let file_path = &args[1];
    let num_shares = &args[2];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message

    let ciphertext = cipher.encrypt(&nonce, contents.as_bytes())?;
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref())?;

    let enc_keys = nn_secret_share::encode(&key, num_shares.parse()?);

    Ok(())
}
