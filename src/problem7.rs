//! 10001st prime
//!
//! > By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th
//! > prime is 13.
//! >
//! > What is the 10 001st prime number?

use prime;

/// Returns the 10001st prime number.
///
///     assert_eq!(rust_euler::problem7::solution(), 104743);
///
pub fn solution() -> u64 {
    prime::nth(10_000)
}
