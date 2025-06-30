use crate::{algarism::Algarism, number::Number};
use num_traits::Zero;
use std::{
    cmp::Ordering,
    ops::{Neg, Sub},
};

impl Sub for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl Sub for &Number {
    type Output = Number;

    fn sub(self, rhs: Self) -> Self::Output {
        let lhs = self;

        if rhs.is_zero() {
            return self.clone();
        }

        if lhs.is_zero() {
            return rhs.neg();
        }

        eprintln!("lhs = {}, rhs = {}", lhs, rhs);
        dbg!(lhs.sign.cmp(&rhs.sign), lhs.abs_cmp(rhs));

        match lhs.sign.cmp(&rhs.sign) {
            Ordering::Less => -(&lhs.neg() + rhs),
            Ordering::Equal => match lhs.abs_cmp(rhs) {
                Ordering::Greater => {
                    let mut borrow = 0;

                    let algs: Vec<Algarism> = lhs
                        .align_algarisms(rhs)
                        .into_iter()
                        .rev()
                        .map(|(a, b)| {
                            let (diff, b) = (&a - &b).unwrap();
                            eprintln!("diff = {}, b = {}", diff, b);

                            let diff = diff - borrow;
                            eprintln!("diff = {}", diff);

                            borrow = b;

                            diff
                        })
                        .collect();

                    Number::new(lhs.sign.clone(), &algs)
                }
                Ordering::Less => -(rhs - lhs),
                Ordering::Equal => Number::zero(),
            },
            Ordering::Greater => lhs + &rhs.neg(),
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
    fn test_sub() {
        for (n1, n2, n3) in [
            n!(0, 0, 0),
            n!(10, 0, 10),
            n!(0, -15, 15),
            n!(69, 69, 0),
            n!(14, -15, 29),
            n!(15, 14, 1),
            n!(120, 43, 77),
        ] {
            let res1 = &n1 - &n2;
            let res2 = &n2 - &n1;

            assert_eq!(res1, n3, "{} - {} == {} != {}", n1, n2, res1, n3);
            assert_eq!(res1, -res2);
        }
    }
}
