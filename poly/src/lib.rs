use num::rational::Rational64;
// Struct that represents a polynomial
// with its highest power, all coefficients(in order from highest power to lowest power),
// and any remainders it might have.
#[derive(Clone, PartialEq, Eq)]
pub struct Poly {
    values: Vec<Rational64>,
}
fn display_rat64(rat: &Rational64) -> String {
    if rat.is_integer() {
        return format!("{}", rat.to_integer());
    }
    format!("{}/{}", rat.numer(), rat.denom())
}
impl Poly {
    // Creates new Poly from Rational64 vector
    pub fn new(coeffs: Vec<Rational64>) -> Self {
        if coeffs.len() == 0 {
            Poly {
                values: vec![Rational64::from_integer(0)],
            }
        } else {
            Poly { values: coeffs }.remove_trail()
        }
    }

    pub fn from_integer_slice(coeffs: Vec<i64>) -> Self {
        Poly::new(
            coeffs
                .into_iter()
                .map(|x| Rational64::from_integer(x))
                .collect::<Vec<Rational64>>(),
        )
    }

    // Removes trailing zeros, so that the polynomials don't end up like 0x^7+0x^6... ...+15
    fn remove_trail(self) -> Self {
        let mut values: Vec<Rational64> = self
            .values
            .into_iter()
            .rev()
            .skip_while(|&x| x == Rational64::from_integer(0))
            .collect();
        values.reverse();

        if values.len() == 0 {
            values.push(Rational64::from_integer(0));
        }

        Poly { values }
    }
}

impl std::fmt::Debug for Poly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for i in (1..self.values.len()).rev() {
            write!(f, "{}x^{} + ", display_rat64(&self.values[i]), i)?;
        }
        write!(f, "{}x^{}", display_rat64(&self.values[0]), 0)?;
        Ok(())
    }
}

impl std::fmt::Display for Poly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut t: Vec<(Rational64, usize)> = Vec::new();
        for i in 0..self.values.len() {
            if self.values[i] != Rational64::from_integer(0) {
                t.push((self.values[i], i));
            }
        }
        if t.len() == 0 {
            t.push((Rational64::from_integer(0), 0));
        }
        for i in (1..t.len()).rev() {
            //checks if any of the remaining terms(before the final term) is the x^1 term
            if t[i].1 == 1 {
                write!(f, "{}x + ", display_rat64(&t[i].0))?;
            } else {
                write!(f, "{}x^{} + ", display_rat64(&t[i].0), t[i].1)?;
            }
        }
        match t[0].1 {
            0 => write!(f, "{}", display_rat64(&t[0].0))?,
            1 => write!(f, "{}x", display_rat64(&t[0].0))?,
            _ => write!(f, "{}x^{}", display_rat64(&t[0].0), t[0].1)?,
        }
        Ok(())
    }
}

// Implements Add for the Poly
impl std::ops::Add for Poly {
    type Output = Poly;

    fn add(self, poly2: Poly) -> Self {
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
            higher.values[x] += lower.values[x];
        }

        higher.remove_trail()
    }
}

//Implements Sub for the Poly
impl std::ops::Sub for Poly {
    type Output = Self;

    fn sub(self, poly2: Poly) -> Self {
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
                higher.values[x] - lower.values.get(x).unwrap_or(&Rational64::from_integer(0))
            } else {
                lower.values.get(x).unwrap_or(&Rational64::from_integer(0)) - higher.values[x]
            };
        }

        higher.remove_trail()
    }
}

impl std::ops::Mul for Poly {
    type Output = Self;

    fn mul(self, poly2: Poly) -> Self {
        let new_power = self.values.len() + poly2.values.len() - 1;
        // Allocate a new vec of the required length
        let mut accum = vec![Rational64::from_integer(0); new_power];
        // Loops through both vecs and mults them(added to the stuff already in there)
        for x in 0..self.values.len() {
            for y in 0..poly2.values.len() {
                accum[x + y] += self.values[x] * poly2.values[y];
            }
        }
        Poly::new(accum)
    }
}

impl std::ops::Div for Poly {
    type Output = (Self, Self);

    fn div(self, poly2: Poly) -> Self::Output {
        let mut dividend = self.values;
        let divisor = poly2.values;
        let div_len = divisor.len();
        if (div_len == 0) || (divisor[div_len - 1] == Rational64::from_integer(0)) {
            panic!("Division by zero error");
        }
        if dividend.len() < divisor.len() {
            return (Poly::from_integer_slice(vec![0]), Poly::new(dividend));
        }
        let mut temp: Vec<Rational64> =
            vec![Rational64::from_integer(0); dividend.len() + 1 - div_len];
        let mut t;

        for x in ((div_len - 1)..dividend.len()).rev() {
            t = dividend[x] / divisor[div_len - 1];
            temp[x + 1 - div_len] = t;
            for y in 0..div_len {
                dividend[x - y] -= t * divisor[div_len - y - 1];
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
            (Poly::from_integer_slice(vec![3, 2, 1]) + Poly::from_integer_slice(vec![4, 3, 2, 1]))
                .values
                .len(),
            4
        );
    }
    #[test]
    fn check_add_exact() {
        assert_eq!(
            (Poly::from_integer_slice(vec![3, 2, 1]) + Poly::from_integer_slice(vec![4, 3, 2, 1])),
            Poly::from_integer_slice(vec![7, 5, 3, 1])
        );
    }
    #[test]
    fn check_sub_len() {
        assert_eq!(
            (Poly::from_integer_slice(vec![3, 2, 1]) - Poly::from_integer_slice(vec![4, 3, 2, 1]))
                .values
                .len(),
            4
        );
    }
    #[test]
    fn check_sub_exact() {
        assert_eq!(
            (Poly::from_integer_slice(vec![4, 2, 1]) - Poly::from_integer_slice(vec![4, 3, 2, 1])),
            Poly::from_integer_slice(vec![0, -1, -1, -1])
        );
    }

    #[test]
    fn add_then_sub_is_same() {
        assert_eq!(
            (Poly::from_integer_slice(vec![4, 2, 1]) - Poly::from_integer_slice(vec![4, 3, 2, 1])
                + Poly::from_integer_slice(vec![4, 3, 2, 1])),
            Poly::from_integer_slice(vec![4, 2, 1])
        );
    }
    #[test]
    fn check_new_len() {
        let tvec = vec![1, 2, 3, 4, 5];
        assert_eq!(Poly::from_integer_slice(tvec).values.len(), 5);
    }

    #[test]
    fn check_new_exact() {
        let tvec = vec![1, 2, 3, 4, 5];
        assert_eq!(
            Poly::from_integer_slice(tvec),
            Poly::from_integer_slice(vec![1, 2, 3, 4, 5])
        );
    }
    #[test]
    fn check_new_when_empty() {
        let tvec = Vec::new();
        assert_eq!(
            Poly::from_integer_slice(tvec),
            Poly::from_integer_slice(vec![0])
        );
    }

    #[test]
    fn add_zero_leaves_result_unchanged() {
        // f(x) = 0;
        let p1 = Poly::from_integer_slice(vec![0]);
        // f(x) = x;
        let p2 = Poly::from_integer_slice(vec![0, 1]);
        let p3 = p1 + p2.clone();
        assert_eq!(p3, p2);
    }

    #[test]
    fn check_mul_len() {
        let tvec = vec![1, 2, 3];
        assert_eq!(
            (Poly::from_integer_slice(tvec.clone()) * Poly::from_integer_slice(tvec))
                .values
                .len(),
            5
        );
    }

    #[test]
    fn check_mul_exact() {
        let tvec = vec![1, 2, 3];
        assert_eq!(
            (Poly::from_integer_slice(tvec.clone()) * Poly::from_integer_slice(tvec)),
            Poly::from_integer_slice(vec![1, 4, 10, 12, 9])
        );
    }

    #[test]
    fn add_to_just_zero() {
        assert_eq!(
            (Poly::from_integer_slice(vec![-5]) + Poly::from_integer_slice(vec![5]))
                .values
                .len(),
            1
        );
    }
    #[test]
    fn check_div_len() {
        assert_eq!(
            (Poly::from_integer_slice(vec![0, 1, 2, 3]) / Poly::from_integer_slice(vec![0, 1]))
                .0
                .values
                .len(),
            3
        );
    }
    #[test]
    fn check_div_exact() {
        assert_eq!(
            (Poly::from_integer_slice(vec![-4, 0, -2, 1]) / Poly::from_integer_slice(vec![-3, 1])),
            (
                Poly::from_integer_slice(vec![3, 1, 1]),
                Poly::from_integer_slice(vec![5])
            )
        );
    }

    #[test]
    fn div_by_longer_poly() {
        assert_eq!(
            (Poly::from_integer_slice(vec![4, 0, 0, 1])
                / Poly::from_integer_slice(vec![6, 0, 0, 0, 0, 0, 0, 1])),
            (
                Poly::from_integer_slice(vec![0]),
                Poly::from_integer_slice(vec![4, 0, 0, 1])
            )
        );
    }

    #[test]
    fn div_by_equal_len() {
        // x^3 + 4 = (x^3 + 6) * 1 - 2
        assert_eq!(
            (Poly::from_integer_slice(vec![4, 0, 0, 1])
                / Poly::from_integer_slice(vec![6, 0, 0, 1])),
            (
                Poly::from_integer_slice(vec![1]),
                Poly::from_integer_slice(vec![-2])
            )
        );
    }

    #[test]
    fn simple_div_by_equal_len() {
        // x^3 + 4 = (x^3 + 6) * 1 - 2
        assert_eq!(
            (Poly::from_integer_slice(vec![8, 0, 0, 2])
                / Poly::from_integer_slice(vec![4, 0, 0, 1])),
            (
                Poly::from_integer_slice(vec![2]),
                Poly::from_integer_slice(vec![0])
            )
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
        let _ = Poly::from_integer_slice(vec![4, 0, 0, 1]) / Poly::from_integer_slice(vec![0]);
    }

    #[test]
    #[should_panic(expected = "Division by zero error")]
    fn div_by_zero_panic_wrong_constuctor() {
        let x = Rational64::from_integer(0);
        let _ = Poly::from_integer_slice(vec![4, 0, 0, 1])
            / Poly {
                values: vec![x, x, x],
            };
    }

    #[test]
    fn poly_display() {
        assert_eq!(
            format!("{}", Poly::from_integer_slice(vec![1, 2, 3, 4])),
            String::from("4x^3 + 3x^2 + 2x + 1")
        );
    }

    #[test]
    fn poly_display_zero() {
        assert_eq!(
            format!("{}", Poly::from_integer_slice(vec![0])),
            String::from("0")
        );
    }

    #[test]
    fn poly_display_with_inner_zeros() {
        assert_eq!(
            format!("{}", Poly::from_integer_slice(vec![1, 2, 0, 4, 5])),
            String::from("5x^4 + 4x^3 + 2x + 1")
        );
    }

    #[test]
    fn poly_display_with_no_zero_term() {
        assert_eq!(
            format!("{}", Poly::from_integer_slice(vec![0, 2, 3, 4])),
            String::from("4x^3 + 3x^2 + 2x")
        );
    }

    #[test]
    fn poly_debug() {
        assert_eq!(
            format!("{:?}", Poly::from_integer_slice(vec![1, 2, 3, 4])),
            String::from("4x^3 + 3x^2 + 2x^1 + 1x^0")
        );
    }

    #[test]
    fn poly_debug_with_zeros_in_middle() {
        assert_eq!(
            format!("{:?}", Poly::from_integer_slice(vec![1, 0, 0, 0, 4, 5, 6])),
            String::from("6x^6 + 5x^5 + 4x^4 + 0x^3 + 0x^2 + 0x^1 + 1x^0")
        );
    }
}
