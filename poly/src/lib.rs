mod euclidean;
use euclidean::euclidean;
use num::rational::Rational64;
use z2z::Z2z;
// Struct that represents a polynomial
// with its highest power, all coefficients(in order from highest power to lowest power),
// and any remainders it might have.
#[derive(Clone, PartialEq, Eq)]
pub struct Poly<T> {
    values: Vec<T>,
}
impl<
        T: std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + num::Zero
            + num::One
            + std::cmp::PartialEq
            + std::fmt::Display
            + Clone,
    > Poly<T>
{
    // Creates new Poly from Rational64 vector
    pub fn new(coeffs: Vec<T>) -> Self {
        if coeffs.len() == 0 {
            Poly {
                values: vec![T::zero()],
            }
        } else {
            Poly { values: coeffs }.remove_trail()
        }
    }

    // Removes trailing zeros, so that the polynomials don't end up like 0x^7+0x^6... ...+15
    fn remove_trail(self) -> Self {
        let mut values: Vec<T> = self
            .values
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
    pub fn leading_coeff(self) -> T {
        if self.values.len() == 0 {
            return T::zero();
        }
        self.values[0].clone()
    }

    pub fn normalize_from_value(self, value: T) -> Poly<T> {
        let mut nself = self;
        for i in 0..nself.values.len() {
            nself.values[i] = nself.values[i].clone() / value.clone();
        }
        return nself;
    }

    pub fn modulus(self, poly2: Poly<T>) -> Poly<T> {
        (self / poly2).1
    }

    pub fn inv_mod(self, poly2: Poly<T>) -> Poly<T> {
        let (_rem, s, _t) = euclidean(&self, &poly2);
        assert_eq!(Poly::new(vec![T::one()]), s.clone() * self);
        s
    }
}

pub fn from_integer_slice(coeffs: Vec<i64>) -> Poly<Rational64> {
    Poly::new(
        coeffs
            .into_iter()
            .map(|x| Rational64::from_integer(x))
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

// Implements Add for the Poly
impl<
        T: std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + num::Zero
            + num::One
            + std::cmp::PartialEq
            + Clone
            + std::fmt::Display,
    > std::ops::Add for Poly<T>
{
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

impl<
        T: std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + num::Zero
            + num::One
            + std::cmp::PartialEq
            + Clone
            + std::fmt::Display,
    > num::Zero for Poly<T>
{
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

impl<
        T: std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + num::Zero
            + num::One
            + std::cmp::PartialEq
            + std::fmt::Display
            + Clone,
    > num::One for Poly<T>
{
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

//Implements Sub for the Poly
impl<
        T: std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + num::Zero
            + num::One
            + std::cmp::PartialEq
            + std::fmt::Display
            + Clone,
    > std::ops::Sub for Poly<T>
{
    type Output = Self;

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

impl<
        T: std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + num::Zero
            + num::One
            + std::fmt::Display
            + std::cmp::PartialEq
            + Clone,
    > std::ops::Mul for Poly<T>
{
    type Output = Self;

    fn mul(self, poly2: Poly<T>) -> Self {
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

impl<
        T: std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + num::Zero
            + num::One
            + std::fmt::Display
            + std::cmp::PartialEq
            + Clone,
    > std::ops::Div for Poly<T>
{
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
    use super::*;

    #[test]
    fn check_add_len() {
        assert_eq!(
            (from_integer_slice(vec![3, 2, 1]) + from_integer_slice(vec![4, 3, 2, 1]))
                .values
                .len(),
            4
        );
    }
    #[test]
    fn check_add_exact() {
        assert_eq!(
            (from_integer_slice(vec![3, 2, 1]) + from_integer_slice(vec![4, 3, 2, 1])),
            from_integer_slice(vec![7, 5, 3, 1])
        );
    }

    #[test]
    fn add_zero() {
        assert_eq!(
            (from_integer_slice(vec![3, 2, 1]) + from_integer_slice(vec![0])),
            from_integer_slice(vec![3, 2, 1])
        );
    }

    #[test]
    fn check_sub_len() {
        assert_eq!(
            (from_integer_slice(vec![3, 2, 1]) - from_integer_slice(vec![4, 3, 2, 1]))
                .values
                .len(),
            4
        );
    }
    #[test]
    fn check_sub_exact() {
        assert_eq!(
            (from_integer_slice(vec![4, 2, 1]) - from_integer_slice(vec![4, 3, 2, 1])),
            from_integer_slice(vec![0, -1, -1, -1])
        );
    }

    #[test]
    fn sub_zero() {
        assert_eq!(
            (from_integer_slice(vec![3, 2, 1]) - from_integer_slice(vec![0])),
            from_integer_slice(vec![3, 2, 1])
        );
    }

    #[test]
    fn add_then_sub_is_same() {
        assert_eq!(
            (from_integer_slice(vec![4, 2, 1]) - from_integer_slice(vec![4, 3, 2, 1])
                + from_integer_slice(vec![4, 3, 2, 1])),
            from_integer_slice(vec![4, 2, 1])
        );
    }
    #[test]
    fn check_new_len() {
        let tvec = vec![1, 2, 3, 4, 5];
        assert_eq!(from_integer_slice(tvec).values.len(), 5);
    }

    #[test]
    fn check_new_exact() {
        let tvec = vec![1, 2, 3, 4, 5];
        assert_eq!(
            from_integer_slice(tvec),
            from_integer_slice(vec![1, 2, 3, 4, 5])
        );
    }
    #[test]
    fn check_new_when_empty() {
        let tvec = Vec::new();
        assert_eq!(from_integer_slice(tvec), from_integer_slice(vec![0]));
    }

    #[test]
    fn add_zero_leaves_result_unchanged() {
        // f(x) = 0;
        let p1 = from_integer_slice(vec![0]);
        // f(x) = x;
        let p2 = from_integer_slice(vec![0, 1]);
        let p3 = p1 + p2.clone();
        assert_eq!(p3, p2);
    }

    #[test]
    fn check_mul_len() {
        let tvec = vec![1, 2, 3];
        assert_eq!(
            (from_integer_slice(tvec.clone()) * from_integer_slice(tvec))
                .values
                .len(),
            5
        );
    }

    #[test]
    fn check_mul_exact() {
        let tvec = vec![1, 2, 3];
        assert_eq!(
            (from_integer_slice(tvec.clone()) * from_integer_slice(tvec)),
            from_integer_slice(vec![1, 4, 10, 12, 9])
        );
    }

    #[test]
    fn mul_by_zero() {
        let tvec = vec![1, 2, 3];
        assert_eq!(
            (from_integer_slice(tvec) * from_integer_slice(vec![0])),
            from_integer_slice(vec![0])
        );
    }

    #[test]
    fn add_to_just_zero() {
        assert_eq!(
            (from_integer_slice(vec![-5]) + from_integer_slice(vec![5]))
                .values
                .len(),
            1
        );
    }
    #[test]
    fn check_div_len() {
        assert_eq!(
            (from_integer_slice(vec![0, 1, 2, 3]) / from_integer_slice(vec![0, 1]))
                .0
                .values
                .len(),
            3
        );
    }
    #[test]
    fn check_div_exact() {
        assert_eq!(
            (from_integer_slice(vec![-4, 0, -2, 1]) / from_integer_slice(vec![-3, 1])),
            (
                from_integer_slice(vec![3, 1, 1]),
                from_integer_slice(vec![5])
            )
        );
    }

    #[test]
    fn div_by_longer_poly() {
        assert_eq!(
            (from_integer_slice(vec![4, 0, 0, 1])
                / from_integer_slice(vec![6, 0, 0, 0, 0, 0, 0, 1])),
            (
                from_integer_slice(vec![0]),
                from_integer_slice(vec![4, 0, 0, 1])
            )
        );
    }

    #[test]
    fn div_by_equal_len() {
        // x^3 + 4 = (x^3 + 6) * 1 - 2
        assert_eq!(
            (from_integer_slice(vec![4, 0, 0, 1]) / from_integer_slice(vec![6, 0, 0, 1])),
            (from_integer_slice(vec![1]), from_integer_slice(vec![-2]))
        );
    }

    #[test]
    fn simple_div_by_equal_len() {
        // x^3 + 4 = (x^3 + 6) * 1 - 2
        assert_eq!(
            (from_integer_slice(vec![8, 0, 0, 2]) / from_integer_slice(vec![4, 0, 0, 1])),
            (from_integer_slice(vec![2]), from_integer_slice(vec![0]))
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
        let _ = from_integer_slice(vec![4, 0, 0, 1]) / from_integer_slice(vec![0]);
    }

    #[test]
    #[should_panic(expected = "Division by zero error")]
    fn div_by_zero_panic_wrong_constuctor() {
        let x = Rational64::from_integer(0);
        let _ = from_integer_slice(vec![4, 0, 0, 1])
            / Poly {
                values: vec![x, x, x],
            };
    }

    #[test]
    fn poly_display() {
        assert_eq!(
            format!("{}", from_integer_slice(vec![1, 2, 3, 4])),
            String::from("4x^3 + 3x^2 + 2x + 1")
        );
    }

    #[test]
    fn poly_display_zero() {
        assert_eq!(
            format!("{}", from_integer_slice(vec![0])),
            String::from("0")
        );
    }

    #[test]
    fn poly_display_with_inner_zeros() {
        assert_eq!(
            format!("{}", from_integer_slice(vec![1, 2, 0, 4, 5])),
            String::from("5x^4 + 4x^3 + 2x + 1")
        );
    }

    #[test]
    fn poly_display_with_no_zero_term() {
        assert_eq!(
            format!("{}", from_integer_slice(vec![0, 2, 3, 4])),
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
            format!("{:?}", from_integer_slice(vec![1, 2, 3, 4])),
            String::from("4x^3 + 3x^2 + 2x + 1")
        );
    }

    #[test]
    fn poly_debug_with_zeros_in_middle() {
        assert_eq!(
            format!("{:?}", from_integer_slice(vec![1, 0, 0, 0, 4, 5, 6])),
            String::from("6x^6 + 5x^5 + 4x^4 + 0x^3 + 0x^2 + 0x + 1")
        );
    }

    #[test]
    fn poly_debug_with_zero_at_end() {
        assert_eq!(
            format!("{:?}", from_integer_slice(vec![0, 0, 0, 0, 4, 5, 6])),
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
}
