// Struct that represents a polynomial
// with its highest power, all coefficients(in order from highest power to lowest power),
// and any remainders it might have.
pub struct Poly {
    values: Vec<i64>,
}

impl Poly {
    // Creates new Poly from i64 slice
    fn new(coeff: Vec<i64>) -> Self {
        if coeff.len() == 0 {
            Poly { values: vec![0] }
        } else {
            Poly { values: coeff }.remove_trail()
        }
    }
    // Removes trailing zeros, so that the polynomials don't end up like 0x^7+0x^6... ...+15
    fn remove_trail(self) -> Self {
        if self.values.len() == 1 {
            return self;
        }
        let mut values: Vec<i64> = self
            .values
            .into_iter()
            .rev()
            .skip_while(|&x| x == 0)
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
        let mut lower;
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
                higher.values[x] - lower.values.get(x).unwrap_or(&0)
            } else {
                lower.values.get(x).unwrap_or(&0) - higher.values[x]
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
        let mut accum = vec![0; new_power];
        // Loops through both vecs and mults them(added to the stuff already in there)
        for x in 0..self.values.len() {
            for y in 0..poly2.values.len() {
                accum[x + y] += self.values[x] * poly2.values[y];
            }
        }
        Poly::new(accum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(
            (Poly::new(vec![3, 2, 1]) + Poly::new(vec![4, 3, 2, 1]))
                .values
                .len(),
            4
        );
    }
    #[test]
    fn add_works2() {
        assert_eq!(
            (Poly::new(vec![3, 2, 1]) + Poly::new(vec![4, 3, 2, 1])).values,
            vec![7, 5, 3, 1]
        );
    }
    #[test]
    fn sub_works() {
        assert_eq!(
            (Poly::new(vec![3, 2, 1]) - Poly::new(vec![4, 3, 2, 1]))
                .values
                .len(),
            4
        );
    }
    #[test]
    fn sub_works2() {
        assert_eq!(
            (Poly::new(vec![4, 2, 1]) - Poly::new(vec![4, 3, 2, 1])).values,
            vec![0, -1, -1, -1]
        );
    }

    #[test]
    fn a_b_works() {
        assert_eq!(
            (Poly::new(vec![4, 2, 1]) - Poly::new(vec![4, 3, 2, 1]) + Poly::new(vec![4, 3, 2, 1]))
                .values,
            vec![4, 2, 1]
        );
    }
    #[test]
    fn new_works() {
        let tvec = vec![1, 2, 3, 4, 5];
        assert_eq!(Poly::new(tvec).values.len(), 5);
    }

    #[test]
    fn new_works2() {
        let tvec = vec![1, 2, 3, 4, 5];
        assert_eq!(Poly::new(tvec).values, vec![1, 2, 3, 4, 5]);
    }
    #[test]
    fn new_works3() {
        let tvec = Vec::new();
        assert_eq!(Poly::new(tvec).values, vec![0]);
    }

    #[test]
    fn mul_works() {
        let tvec = vec![1, 2, 3];
        assert_eq!((Poly::new(tvec.clone()) * Poly::new(tvec)).values.len(), 5);
    }

    #[test]
    fn mul_works2() {
        let tvec = vec![1, 2, 3];
        assert_eq!(
            (Poly::new(tvec.clone()) * Poly::new(tvec)).values,
            vec![1, 4, 10, 12, 9]
        );
    }
}
