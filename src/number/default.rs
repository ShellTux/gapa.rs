use super::Number;
use num_traits::Zero;

impl Default for Number {
    fn default() -> Self {
        Self::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{algarism::Algarism, number, sign::Sign};

    #[test]
    fn test_default() {
        assert_eq!(Number::default(), number!(0));
    }
}
