use getrandom;

#[derive(Clone, Debug)]
pub struct Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "noooooooo")?;
        Ok(())
    }
}

impl From<getrandom::Error> for Error {
    fn from(_err: getrandom::Error) -> Self {
        Self
    }
}

impl std::error::Error for Error {}

pub fn encode(bytes: &[u8], share_c: usize) -> Result<Vec<Vec<u8>>, Error> {
    let mut vec = Vec::new();
    let mut buf = vec![0; bytes.len()];
    if share_c == 0 {
        return Err(Error);
    }
    for _ in 0..share_c - 1 {
        getrandom::getrandom(buf.as_mut_slice())?;
        vec.push(buf.clone());
    }
    let mut t: Vec<u8> = bytes.to_vec();
    for i in 0..vec.len() {
        for j in 0..vec[i].len() {
            t[j] ^= vec[i][j];
        }
    }
    vec.push(t);
    Ok(vec)
}

pub fn decode(bytes: &Vec<Vec<u8>>) -> Result<Vec<u8>, Error> {
    if bytes.len() == 0 {
        return Err(Error);
    }
    let mut t = bytes[0].clone();
    for i in 1..bytes.len() {
        if bytes[i].len() != bytes[i - 1].len() {
            return Err(Error);
        }
        for j in 0..bytes[i].len() {
            t[j] ^= bytes[i][j];
        }
    }
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_preserves_length() -> Result<(), Error> {
        let t = [0, 1, 0, 0, 1, 0, 0, 1];
        let newt = encode(&t, 1)?;
        assert_eq!(newt[0].len(), 8);
        Ok(())
    }

    #[test]
    fn encode_preserves_length_when_zero() -> Result<(), Error> {
        let t = &[];
        let newt = encode(t, 1)?;
        assert_eq!(newt[0].len(), 0);
        Ok(())
    }

    #[test]
    fn encode_length_matches_input() -> Result<(), Error> {
        let t = [0, 1, 0, 0, 1, 0, 0, 1];
        let newt = encode(&t, 5)?;
        assert_eq!(newt.len(), 5);
        Ok(())
    }

    #[test]
    fn decode_returns_original() -> Result<(), Error> {
        let t = [0, 1, 0, 0, 1, 0, 0, 1];
        let enct = encode(&t, 5)?;
        let dect = decode(&enct)?;
        assert_eq!(t, dect.as_slice());
        Ok(())
    }

    #[test]
    fn decode_returns_with_hex() -> Result<(), Error> {
        let t = [0x5f, 0xbe, 0xff, 0x11];
        let enct = encode(&t, 5)?;
        let dect = decode(&enct)?;
        assert_eq!(t, dect.as_slice());
        Ok(())
    }

    #[test]
    fn decode_actually_works() -> Result<(), Error> {
        let t = [0, 1, 0, 0, 1, 0, 0, 1];
        let mut enct = encode(&t, 3)?;
        enct[0][0] += 15;
        let dect = decode(&enct)?;
        assert_ne!(t[0], dect.as_slice()[0]);
        for i in 1..dect.len() {
            assert_eq!(t[i], dect.as_slice()[i]);
        }
        Ok(())
    }
    #[test]
    #[should_panic]
    fn modified_decode_is_incorrect() {
        let t = [0, 1, 0, 0, 1, 0, 0, 1];
        let mut enct = encode(&t, 5).unwrap();
        enct[0][1] = 15;
        let dect = decode(&enct).unwrap();
        assert_eq!(t, dect.as_slice());
    }
}
