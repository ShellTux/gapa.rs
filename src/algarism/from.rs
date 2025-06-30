use super::Algarism;

impl<const RADIX: usize> From<(u8, isize)> for Algarism<RADIX> {
    fn from(value: (u8, isize)) -> Self {
        let (base, power) = value;

        Self::new(base, power)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_tuple() {
        assert_eq!(
            Algarism::from((0u8, 0isize)),
            Algarism::<10> { base: 0, power: 0 }
        );

        assert_eq!(
            Algarism::from((8u8, -2isize)),
            Algarism::<10> { base: 8, power: -2 }
        );
    }
}
