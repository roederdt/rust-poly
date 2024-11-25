use crate::new_from_slice;
use crate::Poly;
use lazy_static::lazy_static;
use z2z::Z2z;

lazy_static! {
    static ref IRRED: Poly<Z2z> = new_from_slice(&vec![
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0,
        2, 0, 1,
    ]);
}

#[derive(Clone, PartialEq, Debug)]
pub struct GF2256(Poly<Z2z>);

impl GF2256 {
    pub fn new(inner: &Poly<Z2z>) -> Self {
        GF2256(inner.modulus(&IRRED))
    }
}

impl std::ops::Add for GF2256 {
    type Output = Self;

    fn add(self, rhs: GF2256) -> Self::Output {
        let t = self.0 + rhs.0;
        GF2256(t.modulus(&IRRED))
    }
}
impl std::ops::Sub for GF2256 {
    type Output = Self;
    fn sub(self, rhs: GF2256) -> Self::Output {
        let t = self.0 - rhs.0;
        GF2256(t.modulus(&IRRED))
    }
}

impl std::ops::Mul for GF2256 {
    type Output = Self;
    fn mul(self, rhs: GF2256) -> Self::Output {
        let t = self.0 * rhs.0;
        GF2256(t.modulus(&IRRED))
    }
}

impl std::ops::Div for GF2256 {
    type Output = Self;
    fn div(self, rhs: GF2256) -> Self::Output {
        let t = rhs.0.inv_mod(&IRRED);
        GF2256((self.0 * t).modulus(&IRRED))
    }
}

impl num::Zero for GF2256 {
    fn zero() -> Self {
        GF2256(Poly::new(vec![Z2z::Zero]))
    }
    fn is_zero(&self) -> bool {
        if self.0.values[self.0.values.len() - 1] == Z2z::Zero {
            return true;
        }
        false
    }
}

impl num::One for GF2256 {
    fn one() -> Self {
        GF2256(Poly::new(vec![Z2z::One]))
    }
    fn is_one(&self) -> bool {
        if self.0.values[self.0.values.len() - 1] == Z2z::One {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use num::{One, Zero};

    use super::*;

    #[test]
    fn one_plus_one() {
        let t = GF2256::one();
        let f = GF2256::one();
        assert!((t + f).is_zero());
    }

    #[test]
    fn new_simple() {
        let p = new_from_slice(&vec![0, 1]);
        let t = GF2256::new(&p);
        assert_eq!(t.0, p);
    }

    #[test]
    fn inverse_of_inverse() {
        let p = new_from_slice(&vec![0, 1]);
        let t = GF2256::new(&p);
        assert_eq!(t.0.inv_mod(&IRRED).inv_mod(&IRRED), t.0);
    }

    #[test]
    fn distributive() {
        let p = new_from_slice(&vec![0, 1]);
        let a = GF2256::new(&p);
        let t = new_from_slice(&vec![2, 3, 4]);
        let b = GF2256::new(&t);
        let o = new_from_slice(&vec![1, 2, 3, 4]);
        let c = GF2256::new(&o);
        assert_eq!(
            a.clone() * (b.clone() + c.clone()),
            (a.clone() * b.clone()) + (a * c)
        );
    }

    #[test]
    fn large_poly_reduces() {
        let t = new_from_slice(&vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0,
            0, 2, 0, 1, 0, 1, 0, 1, 2, 1, 3,
        ]);
        let a = GF2256::new(&t);
        assert!(a.0.values.len() <= IRRED.values.len());
    }

    #[test]
    fn inverses_work() {
        let t = new_from_slice(&vec![1, 0, 1]);
        let a = GF2256(t);
        assert!((a.clone() / a.clone()).is_one());
    }

    #[test]
    #[should_panic]
    fn zero_has_no_inverse() {
        let t = new_from_slice(&vec![1, 0, 1]);
        let a = GF2256(t);
        let _p = a / GF2256::zero();
    }

    #[test]
    fn passing_irred_gets_zero() {
        let t = new_from_slice(&vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0,
            0, 2, 0, 1,
        ]);
        let a = GF2256::new(&t);
        assert_eq!(a, GF2256::zero());
    }

    #[test]
    fn one_is_unit_of_multiplication() {
        let t = new_from_slice(&vec![1, 2, 3, 4]);
        let a = GF2256::new(&t);
        assert_eq!(a, a.clone() * GF2256::one());
    }

    #[test]
    fn zero_is_unit_of_addition() {
        let t = new_from_slice(&vec![1, 2, 3, 4]);
        let a = GF2256::new(&t);
        assert_eq!(a, a.clone() + GF2256::zero());
    }

    #[test]
    fn zero_times_x() {
        let t = new_from_slice(&vec![1, 2, 3, 4]);
        let a = GF2256::new(&t);
        assert_eq!(GF2256::zero(), a.clone() * GF2256::zero());
    }
}