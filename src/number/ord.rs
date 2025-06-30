use crate::number::Number;
use std::cmp::Ordering;

impl Ord for &Number {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sign.cmp(&other.sign)
    }
}
