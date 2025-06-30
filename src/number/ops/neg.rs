use crate::number::Number;
use std::ops::Neg;

impl Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut result = self;
        result.sign = -result.sign;
        result
    }
}

impl Neg for &Number {
    type Output = Number;

    fn neg(self) -> Self::Output {
        self.clone().neg()
    }
}

#[cfg(test)]
mod tests {
    use crate::number;
    use crate::{algarism::Algarism, number::Number, sign::Sign};

    #[test]
    fn test_neg() {
        // Create some test numbers
        let pos_number = number!(+, (3, 2), (1, 1));
        let neg_number = number!(-, (3, 2), (1, 1));
        let zero_number = number!(0);

        // Negate positive number
        let negated = -pos_number.clone();
        assert_eq!(negated.sign, Sign::Negative);
        assert_eq!(negated.algarisms, pos_number.algarisms);
        // Original should remain unchanged
        assert_eq!(pos_number.sign, Sign::Positive);

        // Negate negative number
        let negated_neg = -neg_number.clone();
        assert_eq!(negated_neg.sign, Sign::Positive);
        assert_eq!(negated_neg.algarisms, neg_number.algarisms);

        // Negate zero
        let neg_zero = -zero_number.clone();
        assert_eq!(neg_zero.sign, Sign::Zero);
        assert_eq!(neg_zero.algarisms, zero_number.algarisms);

        // Negate via reference
        let ref_number = &pos_number;
        let neg_ref = -ref_number;
        assert_eq!(neg_ref.sign, Sign::Negative);
        assert_eq!(neg_ref.algarisms, ref_number.algarisms);
        // Original should stay unchanged
        assert_eq!(ref_number.sign, Sign::Positive);
    }
}
