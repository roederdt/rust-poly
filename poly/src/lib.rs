// Struct that represents a polynomial
// with its highest power, all coefficients(in order from highest power to lowest power),
// and any remainders it might have.
pub struct Poly {
    power: usize,
    values: Vec<i64>,
    rem: Option<(Box<Poly>, Box<Poly>)>,
}

impl Poly {
    fn new(coeff: &[i64]) -> Self {
        Poly {
            power: coeff.len(),
            values: coeff.to_vec(),
            rem: None,
        }
        .remove_trail()
    }

    fn remove_trail(&self) -> Self {
        let mut higher = self.values.clone();
        let mut val = 1;
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

        // shadows higher with lower added to it
        let higher = higher
            .iter()
            .enumerate()
            .map(|p| {
                // if the values isn't in the shorter vec
                if p.0 >= lower.len() {
                    // just use the longer vec
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

        // shadows higher with second subtracted from first
        let higher = higher
            .iter()
            .enumerate()
            .map(|p| {
                // if the values isn't in the shorter vec
                if p.0 >= lower.len() {
                    // just use the longer vec(and subtract it if it's second)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(
            (Poly {
                power: 3,
                values: vec![3, 2, 1],
                rem: None
            } + Poly {
                power: 4,
                values: vec![4, 3, 2, 1],
                rem: None
            })
            .power,
            4
        );
    }
    #[test]
    fn add_works2() {
        assert_eq!(
            (Poly {
                power: 3,
                values: vec![3, 2, 1],
                rem: None
            } + Poly {
                power: 4,
                values: vec![4, 3, 2, 1],
                rem: None
            })
            .values,
            vec![7, 5, 3, 1]
        );
    }
    #[test]
    fn sub_works() {
        assert_eq!(
            (Poly {
                power: 3,
                values: vec![3, 2, 1],
                rem: None
            } - Poly {
                power: 4,
                values: vec![4, 3, 2, 1],
                rem: None
            })
            .power,
            4
        );
    }
    #[test]
    fn sub_works2() {
        assert_eq!(
            (Poly {
                power: 3,
                values: vec![4, 2, 1],
                rem: None
            } - Poly {
                power: 4,
                values: vec![4, 3, 2, 1],
                rem: None
            })
            .values,
            vec![0, -1, -1, -1]
        );
    }

    #[test]
    fn a_b_works() {
        assert_eq!(
            (Poly {
                power: 3,
                values: vec![4, 2, 1],
                rem: None
            } - Poly {
                power: 4,
                values: vec![4, 3, 2, 1],
                rem: None
            } + Poly {
                power: 4,
                values: vec![4, 3, 2, 1],
                rem: None
            })
            .values,
            vec![4, 2, 1]
        );
    }
    #[test]
    fn new_works() {
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        assert_eq!(Poly::new(slice).power, 2);
    }

    #[test]
    fn new_works2() {
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        assert_eq!(Poly::new(slice).values, vec![2, 3]);
    }
}
