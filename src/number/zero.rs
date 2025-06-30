use super::Number;
use crate::{algarism::Algarism, sign::Sign};
use num_traits::Zero;

impl Zero for Number {
    fn zero() -> Self {
        Self::new(Sign::Zero, &[Algarism::new(0, 0)])
    }

    fn is_zero(&self) -> bool {
        self.sign == Sign::Zero || self.algarisms.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::number;

    #[test]
    fn test_zero() {
        assert_eq!(Number::zero(), number!(0));
    }
}
