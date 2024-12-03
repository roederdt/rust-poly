use crate::gf_2_256;
use crate::interpolate;
use crate::new_from_slice;
use crate::Poly;
pub use gf_2_256::GF2256;

use getrandom;

#[derive(Clone)]
pub enum Share {
    ShamirShare { x: GF2256, y: GF2256 },
    XorShare,
}

impl Share {
    pub fn return_x_shamir(self) -> Result<GF2256, Error> {
        match self {
            Share::ShamirShare { x, y: _ } => Ok(x),
            Share::XorShare => Err(Error),
        }
    }

    pub fn return_y_shamir(self) -> Result<GF2256, Error> {
        match self {
            Share::ShamirShare { x: _, y } => Ok(y),
            Share::XorShare => Err(Error),
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

pub fn encode(secret: &[u8], threshold: usize) -> Result<Vec<Share>, Error> {
    let secret = GF2256::new(&new_from_slice(secret));
    let mut ys: Vec<GF2256> = vec![secret.clone()];
    let mut buf = vec![0; 256];
    for _i in 0..threshold - 1 {
        getrandom::getrandom(buf.as_mut_slice())?;
        ys.push(GF2256::new(&new_from_slice(&buf)));
    }
    let secret_poly = Poly::new(ys);
    let mut xs: Vec<GF2256> = Vec::new();

    for i in 0..threshold - 1 {
        xs.push(GF2256::new(&new_from_slice(&i.to_le_bytes())));
    }

    let mut ys = Vec::new();
    for i in 0..threshold {
        let t = secret_poly.evaluate(&xs[i]);
        ys.push(t);
    }

    let shares: Vec<Share> = xs
        .into_iter()
        .zip(ys.into_iter())
        .map(|(x, y)| Share::ShamirShare { x, y })
        .collect();

    Ok(shares)
}
pub fn decode(shares: &Vec<Share>) -> Result<GF2256, Error> {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for i in 0..xs.len() {
        xs.push(shares[i].clone().return_x_shamir()?.clone());
        ys.push(shares[i].clone().return_y_shamir()?.clone());
    }

    let t = interpolate(&xs, &ys)?;
    Ok(t.values[0].clone())
}
