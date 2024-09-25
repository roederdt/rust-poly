// Struct that represents a polynomial
// with its highest power, all coefficients(in order from highest power to lowest power),
// and any remainders it might have.
pub struct Poly {
    power: usize,
    values: Vec<i64>,
    rem: Option<(Box<Poly>, Box<Poly>)>,
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

        // shadows higher with lower added to it(and accidently flips it in the process)
        let higher = higher
            .iter()
            .enumerate()
            .map(|p| {
                // if the values isn't in the shorter vec
                if p.0 as i64 > lower.len() as i64 - 1 {
                    // just use the longer vec
                    higher[((p.0 as i64 - (higher.len() as i64) + 1) * -1) as usize]
                } else {
                    higher[((p.0 as i64 - (higher.len() as i64) + 1) * -1) as usize]
                        + lower[((p.0 as i64 - (lower.len() as i64) + 1) * -1) as usize]
                }
            })
            .collect::<Vec<_>>();

        Self {
            power: higher.len(),
            values: higher,
            rem: None,
        }
    }
}

//Implements Sub for the Poly
impl std::ops::Sub for Poly {
    type Output = Self;

    fn sub(self, poly2: Poly) -> Self {
        let higher;
        let lower;
        if self.power > poly2.power {
            higher = self.values;
            lower = poly2.values;
        } else {
            higher = poly2.values;
            lower = self.values;
        }

        // shadows higher with lower subtracted from it(and accidently flips it in the process)
        let higher = higher
            .iter()
            .enumerate()
            .map(|p| {
                // if the values isn't in the shorter vec
                if p.0 as i64 > lower.len() as i64 - 1 {
                    // just use the longer vec
                    higher[((p.0 as i64 - (higher.len() as i64) + 1) * -1) as usize]
                } else {
                    higher[((p.0 as i64 - (higher.len() as i64) + 1) * -1) as usize]
                        - lower[((p.0 as i64 - (lower.len() as i64) + 1) * -1) as usize]
                }
            })
            .collect::<Vec<_>>();

        Self {
            power: higher.len(),
            values: higher,
            rem: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            (Poly {
                power: 3,
                values: vec![1, 2, 3],
                rem: None
            } + Poly {
                power: 4,
                values: vec![1, 2, 3, 4],
                rem: None
            })
            .power,
            4
        );
    }
}
