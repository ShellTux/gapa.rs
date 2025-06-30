use super::Number;
use crate::sign::Sign;
use num_traits::Zero;
use std::fmt;

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }

        let min_power = self.algarisms.last().map_or(0, |alg| alg.power()).min(0);
        let max_power = self.algarisms.first().map_or(0, |alg| alg.power()).max(0);

        if self.sign == Sign::Negative {
            write!(f, "-")?;
        }

        for power in (min_power..=max_power).rev() {
            let base = self
                .algarisms
                .iter()
                .find(|alg| alg.power() == power)
                .map_or(0, |alg| alg.base());

            if power == -1 {
                write!(f, ".{}", base)
            } else {
                write!(f, "{}", base)
            }?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{algarism::Algarism, number};

    #[test]
    fn test_to_string() {
        for (n, s) in [
            (number!(0), "0"),
            (
                number!(+, (1, 1), (2, 0), (3, -1), (4, -2), (5, -3)),
                "12.345",
            ),
            (
                number!(-, (1, 1), (2, 0), (3, -1), (4, -2), (5, -3)),
                "-12.345",
            ),
            (number!(+, (5, -1)), "0.5"),
            (number!(+, (2, 10)), "20000000000"),
        ] {
            assert_eq!(n.to_string(), s);
        }
    }
}
