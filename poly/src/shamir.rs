use crate::gf_2_256;
use crate::interpolate;
use crate::new_from_slice;
use crate::Poly;
use crate::PolyTraits;
pub use gf_2_256::GF2256;

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

impl From<std::fmt::Error> for Error {
    fn from(_err: std::fmt::Error) -> Self {
        Self
    }
}

pub fn shamir_secret_encode<T: PolyTraits<T> + num::Zero + num::One>(
    req_shares: usize,
    secret: GF2256,
) -> Result<Poly<GF2256>, Error> {
    let mut ys: Vec<GF2256> = vec![secret];
    let mut buf = vec![0; 256];
    for _i in 0..req_shares - 1 {
        getrandom::getrandom(buf.as_mut_slice())?;
        ys.push(GF2256::new(&new_from_slice(&buf)));
    }
    let secret_poly = Poly::new(ys.clone());
    let mut xs: Vec<GF2256> = Vec::new();

    for i in 0..req_shares - 1 {
        let temp_x: Vec<u8> = format!("{i:b}")
            .chars()
            .rev()
            .map(|x| x.to_digit(10).unwrap() as u8)
            .collect();
        println!("{temp_x:?}");
        xs.push(GF2256::new(&new_from_slice(&temp_x)));
    }

    let mut ys = Vec::new();
    for i in 0..req_shares {
        let t = secret_poly.evaluate(&xs[i]);
        ys.push(t);
    }

    let p = interpolate(&xs, &ys)?;
    Ok(p)
}

pub fn shamir_secret_decode(xs: Vec<GF2256>, ys: Vec<GF2256>) -> Result<GF2256, Error> {
    let t = interpolate(&xs, &ys)?;
    Ok(t.values[0].clone())
}
