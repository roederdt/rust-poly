use crate::Poly;
pub fn interpolate<
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + num::Zero
        + num::One
        + std::cmp::PartialEq
        + Clone
        + std::fmt::Display,
>(
    xs: &Vec<T>,
    ys: &Vec<T>,
) -> Result<Poly<T>, std::fmt::Error> {
    if xs.len() != ys.len() {
        return Err(std::fmt::Error);
    }
    let mut bases: Vec<Poly<T>> = Vec::new();
    let mut instance: Poly<T> = Poly::new(&vec![T::one()]);
    for j in 0..xs.len() {
        for i in 0..xs.len() {
            if i == j {
                continue;
            }
            instance = instance
                * (Poly::new(&vec![T::zero() - xs[i].clone(), T::one()])
                    .coeff_div(xs[j].clone() - xs[i].clone()));
        }
        bases.push(instance.clone());
        instance = Poly::new(&vec![T::one()]);
    }
    let mut lagrange = Poly::new(&vec![T::zero()]);
    for i in 0..bases.len() {
        lagrange = lagrange + bases[i].clone().coeff_mul(ys[i].clone());
    }

    Ok(lagrange)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::rational::Rational64;

    #[test]
    fn lagrange_test_simple() -> Result<(), Box<dyn std::error::Error>> {
        let t = interpolate(
            &vec![
                Rational64::from_integer(0),
                Rational64::from_integer(5),
                Rational64::from_integer(10),
                Rational64::from_integer(15),
            ],
            &vec![
                Rational64::from_integer(1),
                Rational64::from_integer(5),
                Rational64::from_integer(2),
                Rational64::from_integer(10),
            ],
        )?;
        assert_eq!(
            t,
            Poly::new(&vec![
                Rational64::from_integer(1),
                Rational64::new(675, 250),
                Rational64::new(-125, 250),
                Rational64::new(6, 250)
            ])
        );
        Ok(())
    }
}
