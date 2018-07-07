//! Largest prime factor
//!
//! > The prime factors of 13195 are 5, 7, 13 and 29.
//! >
//! > What is the largest prime factor of the number 600851475143 ?

use super::prime::factor;

/// Returns the largest prime factor of 600_851_475_143.
pub fn solution() -> u64 {
    *factor(600_851_475_143).iter().next_back().unwrap().0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(), 6857);
    }
}
