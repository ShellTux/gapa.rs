use super::Number;
use crate::{algarism::Algarism, sign::Sign};
use lazy_regex::regex;
use std::str::FromStr;

impl FromStr for Number {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number_pattern = regex!(r"^([+-]?)(([0-9]+)(\.([0-9]*))?|([0-9]*)\.([0-9]+))");

        let s = s.trim();

        if !number_pattern.is_match(s) {
            return Err(format!("Invalid number {}", s));
        }

        let captures = number_pattern.captures(s).unwrap();

        let algs: Vec<Algarism> = captures
            .get(3)
            .map_or_else(|| captures.get(6), Some)
            .map_or("", |m| m.as_str())
            .chars()
            .rev()
            .enumerate()
            .map(|(index, c)| (c.to_digit(10).unwrap() as u8, index as isize))
            .chain(
                captures
                    .get(5)
                    .map_or_else(|| captures.get(7), Some)
                    .map_or("", |m| m.as_str())
                    .char_indices()
                    .map(|(index, c)| (c.to_digit(10).unwrap() as u8, -(index as isize) - 1)),
            )
            .map(|(base, power)| Algarism::new(base, power))
            .collect();

        Ok(match captures.get(1).map_or("", |m| m.as_str()) {
            "-" => Self::new(Sign::Negative, &algs),
            _ => Self::new(Sign::Positive, &algs),
        })
    }
}

impl From<isize> for Number {
    fn from(value: isize) -> Self {
        value.to_string().as_str().parse().unwrap_or_default()
    }
}

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        value.to_string().as_str().parse().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use crate::{number, sign::Sign};

    use super::*;

    #[test]
    fn test_from_str() {
        for (numbers_str, expected_number) in [
            (
                ["0", "+0", "-0", "00.000", "+0000.0", "-0.0000", "-000.00"].iter(),
                number!(0),
            ),
            (
                [
                    "123", "+123", "0000123", "+0123", "123", "123.0000", "+123.0", "+123.", "123.",
                ]
                .iter(),
                number!(+, (1, 2), (2, 1), (3, 0)),
            ),
            (
                ["-123", "-0000123", "-0123", "-123.0000", "-123.0", "-123."].iter(),
                number!(-, (1, 2), (2, 1), (3, 0)),
            ),
            (
                ["3.14", "0003.14", "+003.14", "3.1400", "+3.140"].iter(),
                number!(+, (3, 0), (1, -1), (4, -2)),
            ),
            (
                [
                    "-0.001",
                    "-0.00100",
                    "-0000.001",
                    "-000.00100",
                    "-.001",
                    "-.00100",
                ]
                .iter(),
                number!(-, (1, -3)),
            ),
            (
                [
                    ".5", "0.5", "+.5", "+0.5", ".50", "0.5", "0.50", "00.50", "+.50", "+00.5",
                    "+00.500",
                ]
                .iter(),
                number!(+, (5, -1)),
            ),
        ] {
            for &number_str in numbers_str {
                let n1 = Number::from_str(number_str).unwrap();
                let n2: Number = number_str.parse().unwrap();

                assert_eq!(n1, expected_number, "{} != {}", n1, expected_number);
                assert_eq!(n2, expected_number, "{} != {}", n2, expected_number);
                assert_eq!(n1, n2, "{} != {}", n1, n2);
            }
        }
    }

    #[test]
    fn test_from_int() {
        for (n, expected_number) in [
            (123, number!(+, (1, 2), (2, 1), (3, 0))),
            (-123, number!(-, (1, 2), (2, 1), (3, 0))),
        ] {
            let n: Number = n.into();

            assert_eq!(n, expected_number, "{} != {}", n, expected_number);
        }
    }

    #[test]
    fn test_from_float() {
        for (n, expected_number) in [
            (3.14, number!(+, (3, 0), (1, -1), (4, -2))),
            (-0.001, number!(-, (1, -3))),
            (0.5, number!(+, (5, -1))),
        ] {
            let n: Number = n.into();

            assert_eq!(n, expected_number, "{} != {}", n, expected_number);
        }
    }
}
