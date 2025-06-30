use crate::{algarism::Algarism, number::Number};
use num_traits::Zero;
use std::{
    cmp::Ordering,
    ops::{Add, Neg},
};

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl Add for &Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        let lhs = self;

        if lhs.is_zero() {
            return rhs.clone();
        }

        if rhs.is_zero() {
            return lhs.clone();
        }

        match lhs.sign.cmp(&rhs.sign) {
            Ordering::Less => rhs - &lhs.neg(),
            Ordering::Equal => {
                let mut carry = false;

                let algs: Vec<Algarism> = lhs
                    .align_algarisms(rhs)
                    .iter()
                    .rev()
                    .map(|(a, b)| {
                        let (sum, c) = a + (b, carry);

                        carry = c;

                        sum
                    })
                    .collect();

                Number::new(lhs.sign.clone(), &algs)
            }
            Ordering::Greater => lhs - &rhs.neg(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! n {
        ( $( $x:expr ),* $(,)? ) => {
            (
                $( Number::from($x) ),*
            )
        };
    }

    #[test]
    fn test_add() {
        for (n1, n2, n3) in [
            n!(0, 0, 0),
            n!(0, 1, 1),
            n!(0, 10, 10),
            n!(-10, 0, -10),
            n!(10, 5, 15),
            n!(0., 0.5, 0.5),
            n!(-0.12, 0.12, 0.),
            n!(3.14, 0.7501, 3.8901),
            n!(1.2, 2.3, 3.5),
            n!(0.5, -0.5, 0.),
        ] {
            let sum1 = &n1 + &n2;
            let sum2 = &n2 + &n1;

            assert_eq!(sum1, n3, "{} + {} == {} != {}", n1, n2, sum1, n3);
            assert_eq!(sum2, n3, "{} + {} == {} != {}", n2, n1, sum2, n3);
        }
    }
}
