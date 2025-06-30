use super::Algarism;

impl<const RADIX: usize> Default for Algarism<RADIX> {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(Algarism::default(), Algarism::<10> { base: 0, power: 0 });
    }
}
