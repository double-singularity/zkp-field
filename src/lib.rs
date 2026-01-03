// src/lib.rs

// 1. Define our Prime Modulus.
// In a real ZK app (like Starknet), this number is massive (250+ bits).
// For this toy project, we use a small prime: 17.
// This means all numbers in our universe are 0..16.
const MODULUS: u64 = 17;

// 2. Define the Struct.
// We derive:
// - Debug: So we can print it with {:?}
// - Clone, Copy: Because numbers should be cheap to copy (stack memory), not moved.
// - PartialEq: So we can check if a == b.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FieldElement {
    value: u64,
}

impl FieldElement {

    pub fn new(value: u64) -> Self {
        Self {
            value: value % MODULUS,
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

use std::ops::Add;

impl Add for FieldElement {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        FieldElement::new(self.value() + other.value())
    }
}

use std::fmt;

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod test {
    use super::*;



    #[test]
    fn test_creation_and_modulo() {
        let a = FieldElement::new(5);
        assert_eq!(a.value(), 5);

        let b = FieldElement::new(20);
        assert_eq!(b.value(), 3);

        let c = FieldElement::new(17);
        assert_eq!(c.value(), 0);
    }

    #[test]
    fn test_equality() {
        let a = FieldElement::new(20);
        let b = FieldElement::new(3);

        assert_eq!(a.value(), b.value());
    }

    #[test]
    fn test_add_func() {
        let a = FieldElement::new(17);
        let b = FieldElement::new(1);
        let c = FieldElement::new(18);
        let d = FieldElement::new(3);

        assert_eq!((a + b).value(), 1);
        assert_eq!((c + d).value(), 4);
        assert_eq!((b + c).value(), 2);
    }
}