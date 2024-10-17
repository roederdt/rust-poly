use crate::Poly;
pub fn euclidean(poly1: &Poly, poly2: &Poly) -> (Poly, Poly, Poly) {
    let mut remainder_old = poly1.clone();
    let mut remainder_new = poly2.clone();
    let mut quotient: Poly;
    let (mut s_old, mut s_new) = (
        Poly::from_integer_slice(vec![1]),
        Poly::from_integer_slice(vec![0]),
    );
    let (mut t_old, mut t_new) = (
        Poly::from_integer_slice(vec![0]),
        Poly::from_integer_slice(vec![1]),
    );
    let mut temp;
    while remainder_new != Poly::from_integer_slice(vec![0]) {
        temp = remainder_new.clone();
        (quotient, remainder_new) = remainder_old.clone() / remainder_new.clone();
        remainder_old = temp;

        (s_old, s_new) = (
            s_new.clone(),
            s_old.clone() - (quotient.clone() * s_new.clone()),
        );
        dbg!(s_old.clone());
        (t_old, t_new) = (
            t_new.clone(),
            t_old.clone() - (quotient.clone() * t_new.clone()),
        );
        dbg!(t_old.clone());
    }
    // normalize
    let lc = Poly::leading_coeff(remainder_old.clone());

    dbg!("s_old.clone()");
    s_old = Poly::normalize_from_value(s_old.clone(), lc);
    dbg!(s_old.clone());
    t_old = Poly::normalize_from_value(t_old.clone(), lc);
    dbg!(t_old.clone());

    remainder_old = Poly::normalize(remainder_old);

    (s_old, t_old, remainder_old)
}

#[cfg(test)]
mod tests {
    use num::Rational64;

    use super::*;

    #[test]
    fn bookcookmath_example_polynomial_gcd() {
        assert_eq!(
            euclidean(
                &Poly::from_integer_slice(vec![1, 0, -1, 0, 2, 1]),
                &Poly::from_integer_slice(vec![-1, 0, 0, 0, 1])
            )
            .2,
            Poly::from_integer_slice(vec![1],)
        );
    }

    #[test]
    fn bookcookmath_example_polynomial_bezout() {
        assert_eq!(
            euclidean(
                &Poly::from_integer_slice(vec![1, 0, -1, 0, 2, 1]),
                &Poly::from_integer_slice(vec![-1, 0, 0, 0, 1])
            ),
            (
                Poly::new(vec![
                    Rational64::new(23, 51),
                    Rational64::new(-10, 51),
                    Rational64::new(11, 51),
                    Rational64::new(-7, 51)
                ]),
                Poly::new(vec![
                    Rational64::new(-28, 51),
                    Rational64::new(-10, 51),
                    Rational64::new(-12, 51),
                    Rational64::new(3, 51),
                    Rational64::new(7, 51)
                ]),
                Poly::from_integer_slice(vec![1])
            )
        )
    }

    #[test]
    fn sage_example_complex() {
        assert_eq!(
            euclidean(
                &Poly::from_integer_slice(vec![1, 0, 1]),
                &Poly::from_integer_slice(vec![-1, 12, -20, -52, 3, 1])
            ),
            (
                Poly::new(vec![
                    Rational64::new(4731, 4709),
                    Rational64::new(-329, 4709),
                    Rational64::new(-3511, 4709),
                    Rational64::new(173, 4709),
                    Rational64::new(65, 4709)
                ]),
                Poly::new(vec![Rational64::new(22, 4709), Rational64::new(-65, 4709)]),
                Poly::from_integer_slice(vec![1])
            )
        )
    }

    #[test]
    fn sage_example_simple() {
        assert_eq!(
            euclidean(
                &Poly::from_integer_slice(vec![1, 0, 1]),
                &Poly::from_integer_slice(vec![-1, 1, -1, -1, 1, 1])
            ),
            (
                Poly::new(vec![
                    Rational64::new(11, 10),
                    Rational64::new(-4, 10),
                    Rational64::new(-7, 10),
                    Rational64::new(2, 10),
                    Rational64::new(3, 10)
                ]),
                Poly::new(vec![Rational64::new(1, 10), Rational64::new(-3, 10)]),
                Poly::from_integer_slice(vec![1])
            )
        )
    }
}
