mod euclidean;
mod shamir;
pub use euclidean::euclidean;
pub use shamir::ShamirShare;
mod lagrange;
pub use lagrange::interpolate;
mod gf_2_256;
pub use gf_2_256::GF2256;
use num::{rational::Rational64, Zero};
use z2z::Z2z;

// Struct that represents a polynomial
// by all coefficients(in order from highest power to lowest power)
#[derive(Clone, PartialEq, Eq)]
pub struct Poly<T> {
    values: Vec<T>,
}

pub trait PolyTraits<T>:
    std::ops::Add<Output = T>
    + std::ops::Sub<Output = T>
    + std::ops::Mul<Output = T>
    + std::ops::Div<Output = T>
    + std::cmp::PartialEq
    + Clone
{
}
impl PolyTraits<Rational64> for Rational64 {}

impl PolyTraits<Z2z> for Z2z {}

impl PolyTraits<i64> for i64 {}

impl<T: PolyTraits<T> + num::Zero + num::One> Poly<T> {
    // Creates new Poly from a vector of type T
    pub fn new(coeffs: Vec<T>) -> Self {
        if coeffs.len() == 0 {
            Poly {
                values: vec![T::zero()],
            }
        } else {
            Poly {
                values: coeffs.as_slice().to_vec(),
            }
            .remove_trail()
        }
    }

    // Removes trailing zeros, so that the polynomials don't end up like 0x^7+0x^6... ...+15
    fn remove_trail(self) -> Self {
        let mut values: Vec<T> = self
            .values
            .clone()
            .into_iter()
            .rev()
            .skip_while(|x| *x == T::zero())
            .collect();
        values.reverse();

        if values.len() == 0 {
            values.push(T::zero());
        }

        Poly { values }
    }
    fn normalize(self) -> Self {
        let mut nself = self;
        for i in 0..nself.values.len() {
            nself.values[i] =
                nself.values[i].clone() / nself.values[nself.values.len() - 1].clone();
        }
        return nself;
    }

    // returns the leading coefficient of the function
    pub fn leading_coeff(self) -> T {
        if self.values.len() == 0 {
            return T::zero();
        }
        self.values[0].clone()
    }

    // normalizes the Poly by some T
    pub fn normalize_from_value(self, value: &T) -> Poly<T> {
        let mut nself = self;
        for i in 0..nself.values.len() {
            nself.values[i] = nself.values[i].clone() / value.clone();
        }
        return nself;
    }

    // mod function for Poly
    pub fn modulus(&self, poly2: &Poly<T>) -> Poly<T> {
        (self.div(poly2)).1
    }

    // returns the inverse of self in mod poly2
    pub fn inv_mod(&self, poly2: &Poly<T>) -> Poly<T> {
        if self.is_zero() {
            panic!("zero has no inverse");
        }
        let (s, _t, _rem) = euclidean(self, poly2);
        s
    }

    // divides some T from each coefficient
    // (T division rather than Poly division)
    pub fn coeff_div(self, divisor: T) -> Poly<T> {
        let t = self
            .values
            .into_iter()
            .map(|x| x / divisor.clone())
            .collect();
        Poly::new(t)
    }

    // muls by some T rather than some Poly
    pub fn coeff_mul(self, divisor: T) -> Poly<T> {
        let t = self
            .values
            .into_iter()
            .map(|x| x * divisor.clone())
            .collect();
        Poly::new(t)
    }

    // division by reference rather than by move
    pub fn div(&self, poly2: &Poly<T>) -> (Self, Self) {
        let mut dividend = self.values.clone();
        let divisor = &poly2.values;
        let div_len = divisor.len();
        if (div_len == 0) || (divisor[div_len - 1] == T::zero()) {
            panic!("Division by zero error");
        }
        if dividend.len() < divisor.len() {
            return (Poly::new(vec![T::zero()]), Poly::new(dividend));
        }
        let mut temp: Vec<T> = vec![T::zero(); dividend.len() + 1 - div_len];
        let mut t;

        for x in ((div_len - 1)..dividend.len()).rev() {
            t = dividend[x].clone() / divisor[div_len - 1].clone();
            temp[x + 1 - div_len] = t.clone();
            for y in 0..div_len {
                dividend[x - y] =
                    dividend[x - y].clone() - (t.clone() * divisor[div_len - y - 1].clone());
            }
        }

        (Poly::new(temp), Poly::new(dividend))
    }

    // Evaluates Poly at a given T
    pub fn evaluate(&self, value: &T) -> T {
        let mut sum = T::zero();
        let mut val = T::one();
        for i in 0..self.values.len() {
            // get the x^power
            for _j in 0..i {
                val = val * value.clone();
            }
            // multiply by coeff, sum and reset
            val = val * self.values[i].clone();
            sum = sum + val.clone();
            val = T::one();
        }
        return sum;
    }
}

// Creates a Poly<Rational64> from a Vec of ints
pub fn from_integer_slice(coeffs: &Vec<i64>) -> Poly<Rational64> {
    Poly::new(
        coeffs
            .into_iter()
            .map(|&x| Rational64::from_integer(x))
            .collect::<Vec<Rational64>>(),
    )
}
impl<T: std::fmt::Display> std::fmt::Debug for Poly<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for i in (2..self.values.len()).rev() {
            write!(f, "{}x^{} + ", self.values[i], i)?;
        }
        if self.values.len() >= 2 {
            write!(f, "{}x + ", self.values[1])?;
        }
        write!(f, "{}", self.values[0])?;
        Ok(())
    }
}

// Creates a Poly<Z2Z> from slice
pub fn new_from_slice(slice: &[u8]) -> Poly<Z2z> {
    let mut bits: Vec<Z2z> = Vec::new();
    for x in slice {
        for offset in 0..8 {
            let mask = 1 << offset;
            match x & mask {
                0 => bits.push(Z2z::Zero),
                _ => bits.push(Z2z::One),
            }
        }
    }
    Poly::new(bits)
}

impl<T: std::fmt::Display + num::Zero + Clone + num::One + PartialEq> std::fmt::Display
    for Poly<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut t: Vec<(T, usize)> = Vec::new();
        for i in 0..self.values.len() {
            if !self.values[i].is_zero() {
                t.push((self.values[i].clone(), i));
            }
        }
        if t.len() == 0 {
            t.push((T::zero(), 0));
        }
        for i in (1..t.len()).rev() {
            //checks if any of the remaining terms(before the final term) is the x^1 term
            if t[i].1 == 1 {
                if t[i].0.is_one() {
                    write!(f, "x + ")?;
                } else {
                    write!(f, "{}x + ", t[i].0)?;
                }
            } else {
                if t[i].0.is_one() {
                    write!(f, "x^{} + ", t[i].1)?;
                } else {
                    write!(f, "{}x^{} + ", t[i].0, t[i].1)?;
                }
            }
        }
        match t[0].1 {
            0 => write!(f, "{}", t[0].0)?,
            1 => {
                if t[0].0.is_one() {
                    write!(f, "x")?
                } else {
                    write!(f, "{}x", t[0].0)?
                }
            }
            _ => {
                if t[0].0.is_one() {
                    write!(f, "x^{}", t[0].1)?
                } else {
                    write!(f, "{}x^{}", t[0].0, t[0].1)?
                }
            }
        }
        Ok(())
    }
}

impl<T: PolyTraits<T> + num::Zero + num::One> num::Zero for Poly<T> {
    fn zero() -> Self {
        Poly::new(vec![T::zero()])
    }
    fn is_zero(&self) -> bool {
        if self.values[self.values.len() - 1] == T::zero() {
            return true;
        }
        false
    }
}

impl<T: PolyTraits<T> + num::Zero + num::One> num::One for Poly<T> {
    fn one() -> Self {
        Poly::new(vec![T::one()])
    }
    fn is_one(&self) -> bool {
        if self.values[self.values.len() - 1] == T::one() {
            return true;
        }
        false
    }
}

// Implements Add for the Poly
impl<T: PolyTraits<T> + num::Zero + num::One> std::ops::Add for Poly<T> {
    type Output = Poly<T>;

    fn add(self, poly2: Poly<T>) -> Self {
        let mut higher;
        let lower;
        if self.values.len() > poly2.values.len() {
            higher = self;
            lower = poly2;
        } else {
            higher = poly2;
            lower = self;
        }

        for x in 0..lower.values.len() {
            higher.values[x] = higher.values[x].clone() + lower.values[x].clone();
        }

        higher.remove_trail()
    }
}

//Implements Sub for the Poly
impl<T: PolyTraits<T> + num::Zero + num::One> std::ops::Sub for Poly<T> {
    type Output = Poly<T>;

    fn sub(self, poly2: Poly<T>) -> Self {
        let h_first;
        let mut higher;
        let lower;
        if self.values.len() >= poly2.values.len() {
            higher = self;
            lower = poly2;
            h_first = true;
        } else {
            higher = poly2;
            lower = self;
            h_first = false;
        }

        for x in 0..higher.values.len() {
            higher.values[x] = if h_first {
                higher.values[x].clone() - lower.values.get(x).unwrap_or(&T::zero()).clone()
            } else {
                lower.values.get(x).unwrap_or(&T::zero()).clone() - higher.values[x].clone()
            };
        }

        higher.remove_trail()
    }
}

impl<T: PolyTraits<T> + num::Zero + num::One> std::ops::Mul for Poly<T> {
    type Output = Poly<T>;

    fn mul(self, poly2: Poly<T>) -> Poly<T> {
        let new_power = self.values.len() + poly2.values.len() - 1;
        // Allocate a new vec of the required length
        let mut accum = vec![T::zero(); new_power];
        // Loops through both vecs and mults them(added to the stuff already in there)
        for x in 0..self.values.len() {
            for y in 0..poly2.values.len() {
                accum[x + y] =
                    accum[x + y].clone() + (self.values[x].clone() * poly2.values[y].clone());
            }
        }
        Poly::new(accum)
    }
}

impl<T: PolyTraits<T> + num::Zero + num::One> std::ops::Div for Poly<T> {
    type Output = (Self, Self);

    fn div(self, poly2: Poly<T>) -> Self::Output {
        let mut dividend = self.values;
        let divisor = poly2.values;
        let div_len = divisor.len();
        if (div_len == 0) || (divisor[div_len - 1] == T::zero()) {
            panic!("Division by zero error");
        }
        if dividend.len() < divisor.len() {
            return (Poly::new(vec![T::zero()]), Poly::new(dividend));
        }
        let mut temp: Vec<T> = vec![T::zero(); dividend.len() + 1 - div_len];
        let mut t;

        for x in ((div_len - 1)..dividend.len()).rev() {
            t = dividend[x].clone() / divisor[div_len - 1].clone();
            temp[x + 1 - div_len] = t.clone();
            for y in 0..div_len {
                dividend[x - y] =
                    dividend[x - y].clone() - (t.clone() * divisor[div_len - y - 1].clone());
            }
        }

        (Poly::new(temp), Poly::new(dividend))
    }
}

#[cfg(test)]
mod tests {
    use num::One;

    use super::*;

    #[test]
    fn check_add_len() {
        assert_eq!(
            (from_integer_slice(&vec![3, 2, 1]) + from_integer_slice(&vec![4, 3, 2, 1]))
                .values
                .len(),
            4
        );
    }
    #[test]
    fn check_add_exact() {
        assert_eq!(
            (from_integer_slice(&vec![3, 2, 1]) + from_integer_slice(&vec![4, 3, 2, 1])),
            from_integer_slice(&vec![7, 5, 3, 1])
        );
    }

    #[test]
    fn add_zero() {
        assert_eq!(
            (from_integer_slice(&vec![3, 2, 1]) + from_integer_slice(&vec![0])),
            from_integer_slice(&vec![3, 2, 1])
        );
    }

    #[test]
    fn check_sub_len() {
        assert_eq!(
            (from_integer_slice(&vec![3, 2, 1]) - from_integer_slice(&vec![4, 3, 2, 1]))
                .values
                .len(),
            4
        );
    }
    #[test]
    fn check_sub_exact() {
        assert_eq!(
            (from_integer_slice(&vec![4, 2, 1]) - from_integer_slice(&vec![4, 3, 2, 1])),
            from_integer_slice(&vec![0, -1, -1, -1])
        );
    }

    #[test]
    fn sub_zero() {
        assert_eq!(
            (from_integer_slice(&vec![3, 2, 1]) - from_integer_slice(&vec![0])),
            from_integer_slice(&vec![3, 2, 1])
        );
    }

    #[test]
    fn add_then_sub_is_same() {
        assert_eq!(
            (from_integer_slice(&vec![4, 2, 1]) - from_integer_slice(&vec![4, 3, 2, 1])
                + from_integer_slice(&vec![4, 3, 2, 1])),
            from_integer_slice(&vec![4, 2, 1])
        );
    }
    #[test]
    fn check_new_len() {
        let tvec = vec![1, 2, 3, 4, 5];
        assert_eq!(from_integer_slice(&tvec).values.len(), 5);
    }

    #[test]
    fn check_new_exact() {
        let tvec = vec![1, 2, 3, 4, 5];
        assert_eq!(
            from_integer_slice(&tvec),
            from_integer_slice(&vec![1, 2, 3, 4, 5])
        );
    }
    #[test]
    fn check_new_when_empty() {
        let tvec = Vec::new();
        assert_eq!(from_integer_slice(&tvec), from_integer_slice(&vec![0]));
    }

    #[test]
    fn add_zero_leaves_result_unchanged() {
        // f(x) = 0;
        let p1 = from_integer_slice(&vec![0]);
        // f(x) = x;
        let p2 = from_integer_slice(&vec![0, 1]);
        let p3 = p1 + p2.clone();
        assert_eq!(p3, p2);
    }

    #[test]
    fn check_mul_len() {
        let tvec = vec![1, 2, 3];
        assert_eq!(
            (from_integer_slice(&tvec) * from_integer_slice(&tvec))
                .values
                .len(),
            5
        );
    }

    #[test]
    fn check_mul_exact() {
        let tvec = vec![1, 2, 3];
        assert_eq!(
            (from_integer_slice(&tvec) * from_integer_slice(&tvec)),
            from_integer_slice(&vec![1, 4, 10, 12, 9])
        );
    }

    #[test]
    fn mul_by_zero() {
        let tvec = vec![1, 2, 3];
        assert_eq!(
            (from_integer_slice(&tvec) * from_integer_slice(&vec![0])),
            from_integer_slice(&vec![0])
        );
    }

    #[test]
    fn add_to_just_zero() {
        assert_eq!(
            (from_integer_slice(&vec![-5]) + from_integer_slice(&vec![5]))
                .values
                .len(),
            1
        );
    }
    #[test]
    fn check_div_len() {
        assert_eq!(
            (from_integer_slice(&vec![0, 1, 2, 3]) / from_integer_slice(&vec![0, 1]))
                .0
                .values
                .len(),
            3
        );
    }
    #[test]
    fn check_div_exact() {
        assert_eq!(
            (from_integer_slice(&vec![-4, 0, -2, 1]) / from_integer_slice(&vec![-3, 1])),
            (
                from_integer_slice(&vec![3, 1, 1]),
                from_integer_slice(&vec![5])
            )
        );
    }

    #[test]
    fn div_by_longer_poly() {
        assert_eq!(
            (from_integer_slice(&vec![4, 0, 0, 1])
                / from_integer_slice(&vec![6, 0, 0, 0, 0, 0, 0, 1])),
            (
                from_integer_slice(&vec![0]),
                from_integer_slice(&vec![4, 0, 0, 1])
            )
        );
    }

    #[test]
    fn div_by_equal_len() {
        // x^3 + 4 = (x^3 + 6) * 1 - 2
        assert_eq!(
            (from_integer_slice(&vec![4, 0, 0, 1]) / from_integer_slice(&vec![6, 0, 0, 1])),
            (from_integer_slice(&vec![1]), from_integer_slice(&vec![-2]))
        );
    }

    #[test]
    fn simple_div_by_equal_len() {
        // x^3 + 4 = (x^3 + 6) * 1 - 2
        assert_eq!(
            (from_integer_slice(&vec![8, 0, 0, 2]) / from_integer_slice(&vec![4, 0, 0, 1])),
            (from_integer_slice(&vec![2]), from_integer_slice(&vec![0]))
        );
    }

    #[test]
    fn multiple_zeros_become_one() {
        let z = Rational64::from_integer(0);
        assert_eq!(
            Poly {
                values: vec![z, z, z, z, z, z, z, z, z, z, z, z]
            }
            .remove_trail(),
            Poly { values: vec![z] }
        )
    }

    #[test]
    #[should_panic(expected = "Division by zero error")]
    fn div_by_zero_panic() {
        let _ = from_integer_slice(&vec![4, 0, 0, 1]) / from_integer_slice(&vec![0]);
    }

    #[test]
    #[should_panic(expected = "Division by zero error")]
    fn div_by_zero_panic_wrong_constuctor() {
        let x = Rational64::from_integer(0);
        let _ = from_integer_slice(&vec![4, 0, 0, 1])
            / Poly {
                values: vec![x, x, x],
            };
    }

    #[test]
    fn poly_display() {
        assert_eq!(
            format!("{}", from_integer_slice(&vec![1, 2, 3, 4])),
            String::from("4x^3 + 3x^2 + 2x + 1")
        );
    }

    #[test]
    fn poly_display_zero() {
        assert_eq!(
            format!("{}", from_integer_slice(&vec![0])),
            String::from("0")
        );
    }

    #[test]
    fn poly_display_with_inner_zeros() {
        assert_eq!(
            format!("{}", from_integer_slice(&vec![1, 2, 0, 4, 5])),
            String::from("5x^4 + 4x^3 + 2x + 1")
        );
    }

    #[test]
    fn poly_display_with_no_zero_term() {
        assert_eq!(
            format!("{}", from_integer_slice(&vec![0, 2, 3, 4])),
            String::from("4x^3 + 3x^2 + 2x")
        );
    }

    #[test]
    fn poly_display_with_frac() {
        let tvec = vec![
            Rational64::from_integer(1),
            Rational64::from_integer(2),
            Rational64::new(5, 3),
        ];
        assert_eq!(
            format!("{:?}", Poly::new(tvec)),
            String::from("5/3x^2 + 2x + 1")
        );
    }

    #[test]
    fn poly_display_with_frac_with_negative_numer_and_denom() {
        let tvec = vec![
            Rational64::from_integer(1),
            Rational64::from_integer(2),
            Rational64::new(-5, -3),
        ];
        assert_eq!(
            format!("{:?}", Poly::new(tvec)),
            String::from("5/3x^2 + 2x + 1")
        );
    }

    #[test]
    fn poly_debug() {
        assert_eq!(
            format!("{:?}", from_integer_slice(&vec![1, 2, 3, 4])),
            String::from("4x^3 + 3x^2 + 2x + 1")
        );
    }

    #[test]
    fn poly_debug_with_zeros_in_middle() {
        assert_eq!(
            format!("{:?}", from_integer_slice(&vec![1, 0, 0, 0, 4, 5, 6])),
            String::from("6x^6 + 5x^5 + 4x^4 + 0x^3 + 0x^2 + 0x + 1")
        );
    }

    #[test]
    fn poly_debug_with_zero_at_end() {
        assert_eq!(
            format!("{:?}", from_integer_slice(&vec![0, 0, 0, 0, 4, 5, 6])),
            String::from("6x^6 + 5x^5 + 4x^4 + 0x^3 + 0x^2 + 0x + 0")
        );
    }

    #[test]
    fn new_from_slice_small() {
        assert_eq!(
            format!("{}", new_from_slice(&vec![7, 0xf])),
            String::from("x^11 + x^10 + x^9 + x^8 + x^2 + x + 1")
        );
    }

    #[test]
    fn irred_poly_for_inv_test() {
        assert_eq!(
            format!("{}", new_from_slice(&vec![0x13])),
            String::from("x^4 + x + 1")
        );
    }

    #[test]
    fn irred_poly_gcd() {
        let t = new_from_slice(&vec![5]);
        let irred = new_from_slice(&vec![0x13]);
        assert!((euclidean(&t, &irred).2).is_one());
    }

    #[test]
    fn inv_mod() {
        let t = new_from_slice(&vec![5]);
        let irred = new_from_slice(&vec![0x13]);
        let inv = Poly::inv_mod(&t, &irred);
        assert_eq!(format!("{}", inv), "x^3 + x + 1");
        assert!(Poly::modulus(&(inv * t), &irred).is_one())
    }

    #[test]
    fn inv_mod_gf_2_256() {
        let t = new_from_slice(&vec![5]);
        let irred = new_from_slice(&vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0,
            0, 2, 0, 1,
        ]);
        let inv = Poly::inv_mod(&t, &irred);
        assert_eq!(format!("{}", irred), "x^256 + x^241 + x^178 + x^121 + 1");
        assert_eq!(
            format!("{}", inv),
            "x^254 + x^252 + x^250 + x^248 + x^246 + x^244 + x^242 + x^240 + x^239 + x^238 + x^237 + x^236 + x^235 + x^234 + x^233 + x^232 + x^231 + x^230 + x^229 + x^228 + x^227 + x^226 + x^225 + x^224 + x^223 + x^222 + x^221 + x^220 + x^219 + x^218 + x^217 + x^216 + x^215 + x^214 + x^213 + x^212 + x^211 + x^210 + x^209 + x^208 + x^207 + x^206 + x^205 + x^204 + x^203 + x^202 + x^201 + x^200 + x^199 + x^198 + x^197 + x^196 + x^195 + x^194 + x^193 + x^192 + x^191 + x^190 + x^189 + x^188 + x^187 + x^186 + x^185 + x^184 + x^183 + x^182 + x^181 + x^180 + x^179 + x^178 + x^177 + x^175 + x^173 + x^171 + x^169 + x^167 + x^165 + x^163 + x^161 + x^159 + x^157 + x^155 + x^153 + x^151 + x^149 + x^147 + x^145 + x^143 + x^141 + x^139 + x^137 + x^135 + x^133 + x^131 + x^129 + x^127 + x^125 + x^123 + x^121"
        );
        assert!(Poly::modulus(&(inv * t), &irred).is_one())
    }

    #[test]
    fn coeff_div_simple() {
        let t = Poly::new(vec![2, 4, 6, 8]);
        let temp = t.coeff_div(2);
        assert_eq!(temp, Poly::new(vec![1, 2, 3, 4]));
    }

    #[test]
    fn coeff_mul_simple() {
        let t = Poly::new(vec![2, 4, 6, 8]);
        let temp = t.coeff_mul(3);
        assert_eq!(temp, Poly::new(vec![6, 12, 18, 24]));
    }

    #[test]
    fn evaluate_poly_ints() {
        let t = from_integer_slice(&vec![1, 2, 3]);
        assert_eq!(
            t.evaluate(&Rational64::from_integer(3)),
            Rational64::from_integer(34)
        );
    }

    #[test]
    fn evaluate_zero() {
        let t = from_integer_slice(&vec![0]);
        assert_eq!(t.evaluate(&Rational64::zero()), Rational64::zero());
    }

    #[test]
    fn evaluate_one() {
        let t = from_integer_slice(&vec![1]);
        assert_eq!(t.evaluate(&Rational64::one()), Rational64::one());
    }

    #[test]
    fn evaluate_with_middle_zeros() {
        let t = from_integer_slice(&vec![1, 0, 0, 0, 1]);
        assert_eq!(
            t.evaluate(&Rational64::from_integer(4)),
            Rational64::from_integer(257)
        );
    }
}
