use super::Algarism;
use std::cmp::Ordering;

impl<const RADIX: usize> Ord for Algarism<RADIX> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.power
            .cmp(&other.power)
            .reverse()
            .then(self.base.cmp(&other.base))
    }
}

impl<const RADIX: usize> PartialOrd for Algarism<RADIX> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a(base: u8, power: isize) -> Algarism<10> {
        Algarism::new(base, power)
    }

    #[test]
    fn test_ord() {
        assert!(a(0, 0) == a(0, 0));
        assert!(a(0, 1) < a(0, 0));
        assert!(a(0, 0) < a(0, -1));
        assert!(a(0, 2) < a(0, -1));
        assert!(a(0, 3) < a(0, -2));
    }
}
