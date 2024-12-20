use crate::gf_2_256;
use crate::interpolate;
use crate::new_from_slice;
use crate::Poly;
use base64::prelude::*;
pub use gf_2_256::GF2256;
use serde::Deserialize;
use serde::Serialize;

use getrandom;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Share {
    ShamirShare { x: GF2256, y: GF2256 },
    XorShare(Vec<u8>),
}

impl Share {
    pub fn to_serde_safe(&self) -> Result<(), Error> {
        Ok(())
    }
    pub fn get_x_shamir(&self) -> Result<GF2256, Error> {
        match self {
            Share::ShamirShare { x, y: _ } => Ok(x.clone()),
            Share::XorShare(_) => Err(Error),
        }
    }

    pub fn get_y_shamir(&self) -> Result<GF2256, Error> {
        match self {
            Share::ShamirShare { x: _, y } => Ok(y.clone()),
            Share::XorShare(_) => Err(Error),
        }
    }

    pub fn len_xor(&self) -> Result<usize, Error> {
        match self {
            Share::ShamirShare { x: _, y: _ } => Err(Error),
            Share::XorShare(a) => Ok(a.len()),
        }
    }
    pub fn to_b64(&self) -> (String, Option<String>) {
        match self {
            Self::ShamirShare { x, y } => (
                BASE64_STANDARD.encode(x.to_bytes()),
                Some(BASE64_STANDARD.encode(y.to_bytes())),
            ),
            Self::XorShare(t) => (BASE64_STANDARD.encode(t), None),
        }
    }
}
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

impl From<std::fmt::Error> for Error {
    fn from(_err: std::fmt::Error) -> Self {
        Self
    }
}

pub trait SecretSharer {
    fn encode(&self, secret: &[u8]) -> Result<Vec<Share>, Error>;
    fn decode(&self, shares: &Vec<Share>) -> Result<Vec<u8>, Error>;
}

pub struct ShamirSharer {
    share_count: usize,
    threshold: u8,
}
impl ShamirSharer {
    pub fn new(share_count: usize, threshold: u8) -> Self {
        ShamirSharer {
            share_count,
            threshold,
        }
    }
}

impl SecretSharer for ShamirSharer {
    fn encode(&self, secret: &[u8]) -> Result<Vec<Share>, Error> {
        if secret.len() > 32 {
            return Err(Error);
        }
        let secret = GF2256::new(&new_from_slice(secret));
        let mut coeffs: Vec<GF2256> = vec![secret.clone()];
        let mut buf = vec![0; 256];
        for _i in 0..self.threshold - 1 {
            getrandom::getrandom(buf.as_mut_slice())?;
            coeffs.push(GF2256::new(&new_from_slice(&buf)));
        }
        let secret_poly = Poly::new(coeffs);
        let mut xs: Vec<GF2256> = Vec::new();
        let mut ys = Vec::new();

        for i in 1..self.share_count + 1 {
            let x = GF2256::new(&new_from_slice(&i.to_le_bytes()));
            ys.push(secret_poly.evaluate(&x));
            xs.push(x);
        }

        let shares: Vec<Share> = xs
            .into_iter()
            .zip(ys.into_iter())
            .map(|(x, y)| Share::ShamirShare { x, y })
            .collect();

        Ok(shares)
    }
    fn decode(&self, shares: &Vec<Share>) -> Result<Vec<u8>, Error> {
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        for i in 0..shares.len() {
            xs.push(shares[i].get_x_shamir()?);
            ys.push(shares[i].get_y_shamir()?);
        }

        let t = interpolate(&xs, &ys)?;

        Ok(t.values[0].to_32_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_one() -> Result<(), Error> {
        let secret = String::from("Hello");
        let secret = secret.as_bytes();
        let sharer = ShamirSharer::new(1, 1);
        let t = sharer.encode(&secret)?;
        assert_eq!(secret, &(sharer.decode(&t)?));
        Ok(())
    }

    #[test]
    fn simple_thresold() -> Result<(), Error> {
        let secret = String::from("Hello");
        let secret = secret.as_bytes();
        let sharer = ShamirSharer::new(8, 4);
        let t = sharer.encode(&secret)?;
        assert_eq!(secret, &(sharer.decode(&t)?));
        Ok(())
    }

    #[test]
    fn actual_thresold_test() -> Result<(), Error> {
        let secret = String::from("Hello");
        let secret = secret.as_bytes();
        let threshold: usize = 4;
        let sharer = ShamirSharer::new(8, threshold as u8);
        let mut temp = Vec::new();
        let t = sharer.encode(&secret)?;
        for i in 0..threshold {
            temp.push(t[i].clone());
        }
        assert_eq!(secret, &(sharer.decode(&temp)?));
        Ok(())
    }

    #[test]
    #[should_panic]
    fn fails_under_thresold_test() {
        let secret = String::from("Hello");
        let secret = secret.as_bytes();
        let threshold: usize = 4;
        let sharer = ShamirSharer::new(8, threshold as u8);
        let mut temp = Vec::new();
        let t = sharer.encode(&secret).unwrap();
        for i in 0..threshold - 1 {
            temp.push(t[i].clone());
        }
        assert_eq!(secret, &(sharer.decode(&temp).unwrap()));
    }

    #[test]
    #[should_panic]
    fn too_long() {
        let secret =
            String::from("Hello, world! This sentence is far too long to properly evaluate");
        let secret = secret.as_bytes();
        let sharer = ShamirSharer::new(8, 4);
        let t = sharer.encode(&secret).unwrap();
        assert_eq!(secret, &(sharer.decode(&t).unwrap()));
    }

    #[test]
    #[should_panic]
    fn empty_secret() {
        let secret = String::from("");
        let secret = secret.as_bytes();
        let sharer = ShamirSharer::new(8, 4);
        let t = sharer.encode(&secret).unwrap();
        assert_eq!(secret, &(sharer.decode(&t).unwrap()));
    }
    #[test]
    fn test_with_specific_failing_value() -> Result<(), Error> {
        let secret = [
            65, 249, 210, 194, 244, 137, 169, 213, 24, 81, 80, 232, 164, 9, 44, 174, 218, 231, 170,
            146, 53, 205, 14, 253, 232, 110, 53, 34, 82, 214, 238, 0,
        ];
        let threshold: usize = 3;
        let sharer = ShamirSharer::new(5, threshold as u8);
        let t = sharer.encode(&secret)?;
        assert_eq!(secret, *(sharer.decode(&t)?));
        Ok(())
    }
}
