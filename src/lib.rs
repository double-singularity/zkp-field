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
    // 3. The Constructor
    // We don't just take the number; we modulo it immediately.
    // This ensures a FieldElement is NEVER invalid.
    // Example: FieldElement::new(20) -> stores 3
    pub fn new(value: u64) -> Self {
        Self {
            value: value % MODULUS,
        }
    }

    // A helper to see the inner value (useful for tests)
    pub fn value(&self) -> u64 {
        self.value
    }
}

// 4. Implement "Display"
// This lets us use println!("{}", f); smoothly.
use std::fmt;

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}