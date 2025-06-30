use std::ops::{Mul, Neg};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sign {
    Positive = 1,
    Negative = -1,
    Zero = 0,
}

impl Neg for Sign {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
            Sign::Zero => Sign::Zero,
        }
    }
}

impl Mul for Sign {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Sign::Positive => rhs,
            Sign::Negative => -rhs,
            Sign::Zero => Sign::Zero,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neg() {
        assert_eq!(-Sign::Negative, Sign::Positive);
        assert_eq!(-Sign::Positive, Sign::Negative);
        assert_eq!(-Sign::Zero, Sign::Zero);
    }
}
