use num::rational::Rational64;
// Struct that represents a polynomial
// with its highest power, all coefficients(in order from highest power to lowest power),
// and any remainders it might have.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Poly {
    values: Vec<Rational64>,
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
        if self.values.len() == 1 {
            return self;
        }
        let mut values: Vec<Rational64> = self
            .values
            .into_iter()
            .rev()
            .skip_while(|&x| x == Rational64::from_integer(0))
            .collect();
        values.reverse();
        Poly { values }
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
        if (divisor.len() == 0) || (dividend[dividend.len() - 1] == Rational64::from_integer(0)) {
            panic!("Division by zero error");
        }
        let mut temp: Vec<Rational64> =
            vec![Rational64::from_integer(0); dividend.len() - div_len + 1];
        let mut t;

        for x in ((div_len - 1)..dividend.len()).rev() {
            dbg!(x);
            t = dbg!(dividend[x]) / divisor[div_len - 1];
            temp[x + 1 - div_len] = dbg!(t);
            for y in 0..div_len {
                dividend[x - y] -= dbg!(t * dbg!(divisor[div_len - y - 1]));
            }
        }

        (Poly::new(temp), Poly::new(dividend))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(
            (Poly::from_integer_slice(vec![3, 2, 1]) + Poly::from_integer_slice(vec![4, 3, 2, 1]))
                .values
                .len(),
            4
        );
    }
    #[test]
    fn add_works2() {
        assert_eq!(
            (Poly::from_integer_slice(vec![3, 2, 1]) + Poly::from_integer_slice(vec![4, 3, 2, 1])),
            Poly::from_integer_slice(vec![7, 5, 3, 1])
        );
    }
    #[test]
    fn sub_works() {
        assert_eq!(
            (Poly::from_integer_slice(vec![3, 2, 1]) - Poly::from_integer_slice(vec![4, 3, 2, 1]))
                .values
                .len(),
            4
        );
    }
    #[test]
    fn sub_works2() {
        assert_eq!(
            (Poly::from_integer_slice(vec![4, 2, 1]) - Poly::from_integer_slice(vec![4, 3, 2, 1])),
            Poly::from_integer_slice(vec![0, -1, -1, -1])
        );
    }

    #[test]
    fn a_b_works() {
        assert_eq!(
            (Poly::from_integer_slice(vec![4, 2, 1]) - Poly::from_integer_slice(vec![4, 3, 2, 1])
                + Poly::from_integer_slice(vec![4, 3, 2, 1])),
            Poly::from_integer_slice(vec![4, 2, 1])
        );
    }
    #[test]
    fn new_works() {
        let tvec = vec![1, 2, 3, 4, 5];
        assert_eq!(Poly::from_integer_slice(tvec).values.len(), 5);
    }

    #[test]
    fn new_works2() {
        let tvec = vec![1, 2, 3, 4, 5];
        assert_eq!(
            Poly::from_integer_slice(tvec),
            Poly::from_integer_slice(vec![1, 2, 3, 4, 5])
        );
    }
    #[test]
    fn new_works3() {
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
    fn mul_works() {
        let tvec = vec![1, 2, 3];
        assert_eq!(
            (Poly::from_integer_slice(tvec.clone()) * Poly::from_integer_slice(tvec))
                .values
                .len(),
            5
        );
    }

    #[test]
    fn mul_works2() {
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
    fn div_works() {
        assert_eq!(
            (Poly::from_integer_slice(vec![0, 1, 2, 3]) / Poly::from_integer_slice(vec![0, 1]))
                .0
                .values
                .len(),
            3
        );
    }
    #[test]
    fn div_works2() {
        assert_eq!(
            (Poly::from_integer_slice(vec![-4, 0, -2, 1]) / Poly::from_integer_slice(vec![-3, 1])),
            (
                Poly::from_integer_slice(vec![3, 1, 1]),
                Poly::from_integer_slice(vec![5])
            )
        );
    }
}
