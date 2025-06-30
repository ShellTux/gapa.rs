pub mod add;
pub mod div;
pub mod mul;
pub mod neg;
pub mod sub;

#[cfg(test)]
mod tests {
    use std::ops::RangeInclusive;

    use crate::number::Number;

    fn get_range() -> RangeInclusive<isize> {
        -50_000..=50_000
    }

    #[test]
    fn test_add() {
        let range = get_range();

        for (a, b) in range.clone().zip(range.clone()) {
            let a_number = Number::from(a);
            let b_number = Number::from(b);
            let expected_sum_number = Number::from(a + b);

            let sum_number = &a_number + &b_number;

            assert_eq!(
                sum_number, expected_sum_number,
                "{} + {} == {} != {}",
                a_number, b_number, sum_number, expected_sum_number
            );
        }
    }

    #[test]
    fn test_sub() {
        let range = get_range();

        for (a, b) in range.clone().zip(range) {
            let a_number = Number::from(a);
            let b_number = Number::from(b);
            let expected_diff_number = Number::from(a - b);

            let diff_number = &a_number - &b_number;

            assert_eq!(
                diff_number, expected_diff_number,
                "{} - {} == {} != {}",
                a_number, b_number, diff_number, expected_diff_number
            );
        }
    }
}
