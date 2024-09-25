// Struct that represents a polynomial
// with its highest power, all coefficients(in order from highest power to lowest power),
// and any remainders it might have.
pub struct Poly {
    power: i64,
    values: Vec::new(),
    rem: Option<(Poly, Poly)>,
}
// Implements Add for the Poly
impl std::ops::Add for Poly {
    type Output = Self;

    pub fn add(&self, poly2: &self) -> Self {
        if self.power < poly2.power {
            return poly2 + self;
        }
        let temp_arr = vec![0; self.power];
        temp_arr = temp_arr
            .iter()
            .enumerate()
            .map(|p| {
                if p > poly2.power {
                    self.values[self.power - p]
                } else {
                    self.values[self.power - p] + poly2.values[poly2.power - p]
                }
            })
            .collect::<Vec<_>>();
        Self {
            power: self.power,
            values: temp_arr,
            rem: None,
        }
    }
}

impl std::ops::Sub for Poly {
    type Output = Self;

    pub fn sub(&self, poly2: &self) -> Self {
        if self.power < poly2.power {
            return poly2 - self;
        }
        let temp_arr = vec![0; self.power];
        temp_arr = temp_arr
            .iter()
            .enumerate()
            .map(|p| {
                if p > poly2.power {
                    self.values[self.power - p]
                } else {
                    self.values[self.power - p] - poly2.values[poly2.power - p]
                }
            })
            .collect::<Vec<_>>();
        PSelf {
            power: self.power,
            values: temp_arr,
            rem: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
