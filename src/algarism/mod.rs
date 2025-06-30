use derive_getters::Getters;

pub mod default;
pub mod display;
pub mod from;
pub mod ops;
pub mod ord;

#[derive(Debug, Clone, Getters, PartialEq, Eq)]
pub struct Algarism<const RADIX: usize = 10> {
    base: u8,
    power: isize,
}

impl<const RADIX: usize> Algarism<RADIX> {
    pub fn new(base: u8, power: isize) -> Self {
        Self { base, power }
    }

    #[allow(unused_comparisons)]
    #[allow(clippy::absurd_extreme_comparisons)]
    pub fn validate(&self) {
        assert!(
            0 <= self.base && self.base < RADIX as u8,
            "self: {:#?}",
            self
        );
    }
}
