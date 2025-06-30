use crate::algarism::Algarism;
use std::ops::Sub;

impl<const RADIX: usize> Sub for &Algarism<RADIX> {
    type Output = Result<(Algarism, usize), String>;

    fn sub(self, rhs: Self) -> Self::Output {
        let lhs = self;

        if lhs.power != rhs.power() {
            return Err(format!(
                "Trying to sub algarisms with different powers: {:?} + {:?}",
                lhs, rhs
            ));
        }

        let (diff, borrow) = if lhs.base >= rhs.base {
            (lhs.base - rhs.base, 0)
        } else {
            (lhs.base + RADIX as u8 - rhs.base, 1)
        };

        Ok((Algarism::new(diff, lhs.power), borrow))
    }
}

impl<const RADIX: usize> Sub<usize> for Algarism<RADIX> {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self::new(self.base - rhs as u8, self.power)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub() {
        for (a, b, expected_result) in [
            ((8, 0), (3, 0), (5, 0), 0),
            ((5, 0), (7, 0), (8, 0), 1),
            ((4, 0), (5, 0), (9, 0), 1),
            ((3, 2), (4, 2), (9, 2), 1),
            ((5, 5), (5, 5), (0, 5), 0),
            ((9, 0), (9, 0), (0, 0), 0),
        ]
        .iter()
        .map(|&(a, b, diff, borrow)| {
            (
                Algarism::<10>::from(a),
                Algarism::<10>::from(b),
                Ok((Algarism::<10>::from(diff), borrow)),
            )
        }) {
            let expected_diff = &expected_result.clone().unwrap().0;

            assert_eq!(
                &a - &b,
                expected_result,
                "{} - {} != {}",
                a,
                b,
                expected_diff
            );
            //assert_eq!(a - b, Ok((diff, borrow)));
            //assert_eq!(&mut a - &mut b, Ok((diff, borrow)));
        }
    }
}
