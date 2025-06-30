use crate::{algarism::Algarism, sign::Sign};
use derive_getters::Getters;
use std::cmp::Ordering;

pub mod default;
pub mod display;
pub mod from;
pub mod ops;
pub mod ord;
pub mod zero;

#[derive(Debug, Clone, Getters, PartialEq, Eq, PartialOrd)]
pub struct Number {
    sign: Sign,
    algarisms: Vec<Algarism>,
}

#[macro_export]
macro_rules! number {
    (+, $(($a:expr, $b:expr)),*) => {{
        Number {
            sign: Sign::Positive,
            algarisms: vec![
                $(Algarism::new($a, $b)),*
            ],
        }
    }};
    (-, $(($a:expr, $b:expr)),*) => {{
        Number {
            sign: Sign::Negative,
            algarisms: vec![
                $(Algarism::new($a, $b)),*
            ],
        }
    }};
    (0) => {
        Number {
            sign: Sign::Zero,
            algarisms: vec![Algarism::new(0, 0)],
        }
    };
}

impl Number {
    pub fn new(sign: Sign, algarisms: &[Algarism]) -> Self {
        if algarisms.is_empty() || algarisms.iter().all(|alg| alg.base() == 0) {
            return Self {
                sign: Sign::Zero,
                algarisms: vec![Algarism::new(0, 0)],
            };
        }

        let mut algarisms: Vec<Algarism> = algarisms
            .iter()
            .filter(|alg| alg.base() != 0)
            .cloned()
            .collect();

        algarisms.sort();

        Self { sign, algarisms }
    }

    fn align_algarisms(&self, b: &Self) -> Vec<(Algarism, Algarism)> {
        let a = self;

        let max_power = a
            .algarisms
            .first()
            .map_or(0, |a| a.power())
            .max(b.algarisms.first().map_or(0, |b| b.power()))
            + 1;
        let min_power = a
            .algarisms
            .last()
            .map_or(0, |a| a.power())
            .min(b.algarisms.last().map_or(0, |b| b.power()));

        (min_power..=max_power)
            .rev()
            .map(|power| {
                (
                    a.algarisms
                        .iter()
                        .find(|a| a.power() == power)
                        .map_or(Algarism::new(0, power), |alg| alg.clone()),
                    b.algarisms
                        .iter()
                        .find(|b| b.power() == power)
                        .map_or(Algarism::new(0, power), |alg| alg.clone()),
                )
            })
            .collect()
    }

    fn abs_cmp(&self, other: &Self) -> Ordering {
        use Ordering::*;

        let mut cmp = Equal;

        for (a, b) in self.align_algarisms(other) {
            if cmp != Equal {
                break;
            }

            cmp = a.cmp(&b);
        }

        cmp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs_cmp() {
        use Ordering::*;

        for (a, b, expected_cmp) in [
            // Zero comparisons
            (number!(0), number!(0), Equal),
            (number!(0), number!(+, (1, 0)), Less),
            (number!(+, (1, 0)), number!(0), Greater),
            // Same length, different algarisms
            (
                number!(+, (3, 2), (2, 1)),
                number!(+, (3, 2), (2, 0)),
                Greater,
            ),
            (
                number!(+, (3, 2), (2, 1)),
                number!(+, (3, 2), (2, 1)),
                Equal,
            ),
            // Prefix cases
            (number!(+, (2, 1)), number!(+, (2, 1), (0, 0)), Equal),
            // Higher powers
            (number!(+, (1, 5)), number!(+, (1, 3)), Greater),
            // Sign differences (should not affect abs_cmp)
            (number!(-, (2, 1)), number!(+, (2, 1)), Equal),
            // Non-overlapping powers
            (number!(+, (1, 3)), number!(+, (1, 1)), Greater),
            // Multiple algarisms
            (
                number!(+, (5, 4), (2, 2)),
                number!(+, (5, 4), (2, 1)),
                Greater,
            ),
            // Zero algarisms in the middle
            (number!(+, (3, 3), (0, 2)), number!(+, (3, 3), (1, 2)), Less),
        ]
        .into_iter()
        {
            let cmp = a.abs_cmp(&b);
            assert_eq!(
                a.abs_cmp(&b),
                expected_cmp,
                "{} {} {}",
                a,
                match cmp {
                    Less => "<",
                    Equal => "==",
                    Greater => ">",
                },
                b
            );
        }
    }
}
