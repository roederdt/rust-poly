#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Z2z {
    Zero,
    One,
}

impl num::Zero for Z2z {
    fn zero() -> Self {
        Z2z::Zero
    }
    fn is_zero(&self) -> bool {
        matches!(self, Z2z::Zero)
    }
}

impl num::One for Z2z {
    fn one() -> Self {
        Z2z::One
    }
    fn is_one(&self) -> bool {
        matches!(self, Z2z::One)
    }
}

impl std::ops::Add for Z2z {
    type Output = Self;

    fn add(self, z2z2: Z2z) -> Self::Output {
        match (self, z2z2) {
            (Z2z::Zero, Z2z::Zero) => Z2z::Zero,
            (Z2z::Zero, Z2z::One) => Z2z::One,
            (Z2z::One, Z2z::Zero) => Z2z::One,
            (Z2z::One, Z2z::One) => Z2z::Zero,
        }
    }
}

impl std::ops::Sub for Z2z {
    type Output = Self;

    fn sub(self, z2z2: Z2z) -> Self::Output {
        match (self, z2z2) {
            (Z2z::Zero, Z2z::Zero) => Z2z::Zero,
            (Z2z::Zero, Z2z::One) => Z2z::One,
            (Z2z::One, Z2z::Zero) => Z2z::One,
            (Z2z::One, Z2z::One) => Z2z::Zero,
        }
    }
}

impl std::ops::Mul for Z2z {
    type Output = Self;

    fn mul(self, z2z2: Z2z) -> Self::Output {
        match (self, z2z2) {
            (Z2z::Zero, Z2z::Zero) => Z2z::Zero,
            (Z2z::Zero, Z2z::One) => Z2z::Zero,
            (Z2z::One, Z2z::Zero) => Z2z::Zero,
            (Z2z::One, Z2z::One) => Z2z::One,
        }
    }
}

impl std::ops::Div for Z2z {
    type Output = Self;

    fn div(self, z2z2: Z2z) -> Self::Output {
        match (self, z2z2) {
            (Z2z::Zero, Z2z::Zero) => panic!("division by Z2z::Zero"),
            (Z2z::Zero, Z2z::One) => Z2z::Zero,
            (Z2z::One, Z2z::Zero) => panic!("division by Z2z::Zero"),
            (Z2z::One, Z2z::One) => Z2z::One,
        }
    }
}

impl std::fmt::Display for Z2z {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Z2z::Zero => {
                write!(f, "1")?;
            }
            Z2z::One => {
                write!(f, "1")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_and_one() {
        assert_eq!(Z2z::One + Z2z::One, Z2z::Zero);
    }

    #[test]
    fn add_one_and_zero() {
        assert_eq!(Z2z::One + Z2z::Zero, Z2z::One);
    }

    #[test]
    fn sub_one_and_zero() {
        assert_eq!(Z2z::One - Z2z::Zero, Z2z::One);
    }

    #[test]
    fn sub_zero_and_one() {
        assert_eq!(Z2z::Zero - Z2z::One, Z2z::One);
    }

    #[test]
    fn mul_zero_and_zero() {
        assert_eq!(Z2z::Zero * Z2z::Zero, Z2z::Zero);
    }

    #[test]
    fn mul_one_and_one() {
        assert_eq!(Z2z::One * Z2z::One, Z2z::One);
    }

    #[test]
    #[should_panic]
    fn div_one_and_zero() {
        assert_eq!(Z2z::One / Z2z::Zero, Z2z::Zero);
    }

    #[test]
    #[should_panic]
    fn div_zero_and_zero() {
        assert_eq!(Z2z::Zero / Z2z::Zero, Z2z::Zero);
    }

    #[test]
    fn div_one_and_one() {
        assert_eq!(Z2z::One / Z2z::One, Z2z::One);
    }

    #[test]
    fn div_zero_and_one() {
        assert_eq!(Z2z::Zero / Z2z::One, Z2z::Zero);
    }
}
