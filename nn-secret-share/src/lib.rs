use getrandom;
use poly::Share;

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

pub struct XorSharer {
    share_count: usize,
}
impl XorSharer {
    pub fn new(share_count: usize) -> Self {
        XorSharer { share_count }
    }
}

impl std::error::Error for Error {}
impl poly::SecretSharer for XorSharer {
    fn encode(&self, secret: &[u8]) -> Result<Vec<Share>, poly::Error> {
        let mut vec = Vec::new();
        let mut buf = vec![0; secret.len()];
        if self.share_count == 0 {
            return Err(poly::Error);
        }
        for _ in 0..self.share_count - 1 {
            getrandom::getrandom(buf.as_mut_slice())?;
            vec.push(buf.clone());
        }
        let mut t: Vec<u8> = secret.to_vec();
        for i in 0..vec.len() {
            for j in 0..vec[i].len() {
                t[j] ^= vec[i][j];
            }
        }
        let mut nvec = Vec::new();
        nvec.push(Share::XorShare(t));
        for item in vec {
            nvec.push(Share::XorShare(item));
        }
        Ok(nvec)
    }

    fn decode(&self, shares: &Vec<Share>) -> Result<Vec<u8>, poly::Error> {
        if shares.len() == 0 {
            return Err(poly::Error);
        }
        let Share::XorShare(mut t) = shares[0].clone() else {
            return Err(poly::Error);
        };
        for i in 1..shares.len() {
            let Share::XorShare(ref si) = shares[i] else {
                return Err(poly::Error);
            };
            let Share::XorShare(ref sim1) = shares[i - 1] else {
                return Err(poly::Error);
            };
            if si.len() != sim1.len() {
                return Err(poly::Error);
            }
            for j in 0..si.len() {
                t[j] ^= si[j];
            }
        }
        Ok(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poly::SecretSharer;

    #[test]
    fn encode_preserves_length() -> Result<(), poly::Error> {
        let t = [0, 1, 0, 0, 1, 0, 0, 1];
        let n = XorSharer::new(4);
        let newt = n.encode(&t)?;
        assert_eq!(newt[0].len_xor()?, 8);
        Ok(())
    }

    #[test]
    fn encode_preserves_length_when_zero() -> Result<(), poly::Error> {
        let t = [];
        let n = XorSharer::new(4);
        let newt = n.encode(&t)?;
        assert_eq!(newt[0].len_xor()?, 0);
        Ok(())
    }

    #[test]
    fn encode_length_matches_input() -> Result<(), poly::Error> {
        let t = [0, 1, 0, 0, 1, 0, 0, 1];
        let n = XorSharer::new(4);
        let newt = n.encode(&t)?;
        assert_eq!(newt.len(), 8);
        Ok(())
    }

    #[test]
    fn decode_returns_original() -> Result<(), poly::Error> {
        let t = [0, 1, 0, 0, 1, 0, 0, 1];
        let n = XorSharer::new(4);
        let newt = n.encode(&t)?;
        let dect = n.decode(&newt)?;
        assert_eq!(dect, t);
        Ok(())
    }

    #[test]
    fn decode_returns_with_hex() -> Result<(), poly::Error> {
        let t = [0x5f, 0xbe, 0xff, 0x11];
        let n = XorSharer::new(4);
        let newt = n.encode(&t)?;
        let dect = n.decode(&newt)?;
        assert_eq!(dect, t);
        Ok(())
    }

    #[test]
    fn decode_actually_works() -> Result<(), poly::Error> {
        let t = [0, 1, 0, 0, 1, 0, 0, 1];
        let n = XorSharer::new(4);
        let mut newt = n.encode(&t)?;
        let Share::XorShare(ref mut temp) = newt[0] else {
            return Err(poly::Error);
        };
        temp[0] += 15;
        let dect = n.decode(&newt)?;
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
        let n = XorSharer::new(4);
        let mut newt = n.encode(&t).unwrap();
        let Share::XorShare(ref mut temp) = newt[0] else {
            panic!("adjnanvd;oic");
        };
        temp[0] += 15;
        let dect = n.decode(&newt).unwrap();
        assert_ne!(t, dect.as_slice());
    }
}
