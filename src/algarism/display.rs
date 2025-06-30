use super::Algarism;
use std::fmt;

impl fmt::Display for Algarism {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.base, "0".repeat(self.power as usize))
    }
}
