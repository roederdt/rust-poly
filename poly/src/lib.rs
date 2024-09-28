// Struct that represents a polynomial
// with its highest power, all coefficients(in order from highest power to lowest power),
// and any remainders it might have.
pub struct Poly {
    power: usize,
    values: Vec<i64>,
    rem: Option<(Box<Poly>, Box<Poly>)>,
}

impl Poly {
    // Creates new Poly from i64 slice
    fn new(coeff: &[i64]) -> Self {
        if coeff.len() == 0 {
            Poly {
                power: coeff.len(),
                values: vec![0],
                rem: None,
            }
            .remove_trail()
        } else {
            Poly {
                power: coeff.len(),
                values: coeff.to_vec(),
                rem: None,
            }
            .remove_trail()
        }
    }
    // Removes trailing zeros, so that the polynomials don't end up like 0x^7+0x^6... ...+15
    fn remove_trail(&self) -> Self {
        let mut higher = self.values.clone();
        let mut val = 1;
        // Loops backwards through the indices of the vec, and if the previous one was a zero, pop it
        // If the current one isn't a zero, end the checking
        for x in (0..higher.len()).rev() {
            if val == 0 {
                higher.pop();
            }
            val = higher[x];
            if val != 0 {
                break;
            }
        }
        Poly {
            power: higher.len(),
            values: higher,
            rem: None,
        }
    }
}

// Implements Add for the Poly
impl std::ops::Add for Poly {
    type Output = Self;

    fn add(self, poly2: Poly) -> Self {
        let higher;
        let lower;
        if self.power > poly2.power {
            higher = self.values;
            lower = poly2.values;
        } else {
            higher = poly2.values;
            lower = self.values;
        }

        // Shadows higher with lower added to it
        let higher = higher
            .iter()
            .enumerate()
            .map(|p| {
                // If the values isn't in the shorter vec
                if p.0 >= lower.len() {
                    // Just use the longer vec
                    higher[p.0]
                } else {
                    higher[p.0] + lower[p.0]
                }
            })
            .collect::<Vec<_>>();
        Poly::new(&higher.as_slice())
    }
}

//Implements Sub for the Poly
impl std::ops::Sub for Poly {
    type Output = Self;

    fn sub(self, poly2: Poly) -> Self {
        let h_first;
        let higher;
        let lower;
        if self.power > poly2.power {
            higher = self.values;
            lower = poly2.values;
            h_first = true;
        } else {
            higher = poly2.values;
            lower = self.values;
            h_first = false;
        }

        // Shadows higher with second subtracted from first
        let higher = higher
            .iter()
            .enumerate()
            .map(|p| {
                // If the values isn't in the shorter vec
                if p.0 >= lower.len() {
                    // Just use the longer vec(and subtract it if it's second)
                    if h_first {
                        higher[p.0]
                    } else {
                        higher[p.0] * -1
                    }
                } else {
                    if h_first {
                        higher[p.0] - lower[p.0]
                    } else {
                        lower[p.0] - higher[p.0]
                    }
                }
            })
            .collect::<Vec<_>>();

        Poly::new(&higher.as_slice())
    }
}

impl std::ops::Mul for Poly {
    type Output = Self;

    fn mul(self, poly2: Poly) -> Self {
        let new_power = self.power + poly2.power - 1;
        // Allocate a new vec of the required length
        let mut sum = vec![0; new_power];
        // Loops through both vecs and mults them(added to the stuff already in there)
        for x in 0..self.values.len() {
            for y in 0..poly2.values.len() {
                sum[x + y] += self.values[x] * poly2.values[y];
            }
        }
        Poly::new(&sum.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(
            (Poly::new(&vec![3, 2, 1]) + Poly::new(&vec![4, 3, 2, 1])).power,
            4
        );
    }
    #[test]
    fn add_works2() {
        assert_eq!(
            (Poly::new(&vec![3, 2, 1]) + Poly::new(&vec![4, 3, 2, 1])).values,
            vec![7, 5, 3, 1]
        );
    }
    #[test]
    fn sub_works() {
        assert_eq!(
            (Poly::new(&vec![3, 2, 1]) - Poly::new(&vec![4, 3, 2, 1])).power,
            4
        );
    }
    #[test]
    fn sub_works2() {
        assert_eq!(
            (Poly::new(&vec![4, 2, 1]) - Poly::new(&vec![4, 3, 2, 1])).values,
            vec![0, -1, -1, -1]
        );
    }

    #[test]
    fn a_b_works() {
        assert_eq!(
            (Poly::new(&vec![4, 2, 1]) - Poly::new(&vec![4, 3, 2, 1])
                + Poly::new(&vec![4, 3, 2, 1]))
            .values,
            vec![4, 2, 1]
        );
    }
    #[test]
    fn new_works() {
        let tvec = vec![1, 2, 3, 4, 5];
        assert_eq!(Poly::new(&tvec).power, 5);
    }

    #[test]
    fn new_works2() {
        let tvec = vec![1, 2, 3, 4, 5];
        assert_eq!(Poly::new(&tvec).values, vec![1, 2, 3, 4, 5]);
    }
    #[test]
    fn new_works3() {
        let tvec = Vec::new();
        assert_eq!(Poly::new(&tvec).values, vec![0]);
    }

    #[test]
    fn mul_works() {
        let tvec = vec![1, 2, 3];
        assert_eq!((Poly::new(&tvec) * Poly::new(&tvec)).power, 5);
    }

    #[test]
    fn mul_works2() {
        let tvec = vec![1, 2, 3];
        assert_eq!(
            (Poly::new(&tvec) * Poly::new(&tvec)).values,
            vec![1, 4, 10, 12, 9]
        );
    }
}
