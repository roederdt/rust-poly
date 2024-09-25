// Struct that represents a polynomial
// with its highest power, all coefficients(in order from highest power to lowest power),
// and any remainders it might have.
pub struct Poly {
    power: i64,
    values: Vec<i64>,
    rem: Option<(Poly, Poly)>,
}

// Implements Add for the Poly
impl std::ops::Add for Poly {
    type Output = Self;

    pub fn add(&self, poly2: &self) -> Self {
        let higher = self if self.power>poly2.power else poly2;
        let lower = self if self.power<=poly2.power else poly2;

        // shadows higher with lower added to it(and accidently flips it in the process)
        let higher = higher
            .iter()
            .enumerate()
            .map(|p| {
                // if the values isn't in the shorter vec
                if p > lower.power {
                    // just use the longer vec
                    higher.values[-(p-higher.power)]
                } else {
                    higher.values[-(p-higher.power)] + lower.values[-(p-lower.power)]
                }
            })
            .collect::<Vec<_>>();
        Self {
            power: higher.power,
            values: higher,
            rem: None,
        }
    }
}

//Implements Sub for the Poly
impl std::ops::Sub for Poly {
    type Output = Self;

    pub fn sub(&self, poly2: &self) -> Self {
        let higher = self if self.power>poly2.power else poly2;
        let lower = self if self.power<=poly2.power else poly2;

        // shadows higher with lower subtracted from it(and accidently flips it in the process)
        let higher = higher
            .iter()
            .enumerate()
            .map(|p| {
                // if the values isn't in the shorter vec
                if p > lower.power {
                    // just use the longer vec
                    higher.values[-(p-higher.power)]
                } else {
                    higher.values[-(p-higher.power)] - lower.values[-(p-lower.power)]
                }
            })
            .collect::<Vec<_>>();

        Self {
            power: higher.power,
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

    }
}
