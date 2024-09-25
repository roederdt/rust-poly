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
        // declares a vec of the right length
        let temp_arr = vec![0; self.power];
        // shadows temp_arr with the poly correctly added
        let temp_arr = temp_arr
            .iter()
            .enumerate()
            .map(|p| {
                // if the values isn't in the shorter vec
                if p > poly2.power {
                    // just use the longer vec
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

//Implements Sub for the Poly
impl std::ops::Sub for Poly {
    type Output = Self;

    pub fn sub(&self, poly2: &self) -> Self {
        if self.power < poly2.power {
            return poly2 - self;
        }
        // declares a vec of the right length
        let temp_arr = vec![0; self.power];
        // shadows temp_arr with the poly correctly subtracted
        let temp_arr = temp_arr
            .iter()
            .enumerate()
            .map(|p| {
                // if the values isn't in the shorter vec
                if p > poly2.power {
                    // just use the longer vec
                    self.values[self.power - p]
                } else {
                    self.values[self.power - p] - poly2.values[poly2.power - p]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
