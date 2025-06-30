use crate::algarism::Algarism;
use std::ops::Add;

impl<const RADIX: usize> Add<(&Algarism, bool)> for &Algarism<RADIX> {
    type Output = (Algarism, bool);

    fn add(self, rhs: (&Algarism, bool)) -> Self::Output {
        let lhs = self;
        let (rhs, carry) = rhs;
        let carry = if carry { 1 } else { 0 };

        if lhs.power != rhs.power() {
            panic!(
                "Trying to add algarisms with different powers: {:?} + {:?}",
                lhs, rhs
            );
        }

        lhs.validate();
        rhs.validate();

        let sum = (lhs.base + rhs.base + carry) as usize;

        let (base, carry) = (sum % RADIX, sum / RADIX);

        assert!(
            carry == 0 || carry == 1,
            "lhs = {:#?}\nrhs = {:#?}",
            lhs,
            rhs
        );

        let carry = carry >= 1;

        (Algarism::new(base as u8, lhs.power), carry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        for (a, b, expected_sum, expected_carry) in [
            ((4, 0), (5, 0), (9, 0), false),
            ((3, 2), (4, 2), (7, 2), false),
            ((5, 5), (5, 5), (0, 5), true),
            ((9, 0), (9, 0), (8, 0), true),
        ]
        .iter()
        .map(|&(a, b, sum, carry)| {
            (
                Algarism::<10>::from(a),
                Algarism::<10>::from(b),
                Algarism::<10>::from(sum),
                carry,
            )
        }) {
            let (sum, carry) = &a + (&b, false);
            assert_eq!(sum, expected_sum);
            assert_eq!(carry, expected_carry);
            //assert_eq!(a + b, Ok((sum, carry)));
            //assert_eq!(&mut a + &mut b, Ok((sum, carry)));
        }
    }

    #[test]
    #[should_panic]
    fn test_add_panic() {
        let a1 = Algarism::<10>::new(0, 0);
        let a2 = Algarism::<10>::new(0, 1);

        let a3 = &a1 + (&a2, false);
        drop(a3);
    }
}
