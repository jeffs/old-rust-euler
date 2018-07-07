//! Sum square difference
//!
//! > The sum of the squares of the first ten natural numbers is,
//! >
//! > 1<sup>2</sup> + 2<sup>2</sup> + ... + 10<sup>2</sup> = 385
//! >
//! > The square of the sum of the first ten natural numbers is,
//! >
//! > (1 + 2 + ... + 10)<sup>2</sup> = 55<sup>2</sup> = 3025
//! >
//! > Hence the difference between the sum of the squares of the first ten natural numbers and the
//! > square of the sum is 3025 − 385 = 2640.
//! >
//! > Find the difference between the sum of the squares of the first one hundred natural numbers
//! > and the square of the sum.

/// Returns the square of the sums, minus the sum of the squares, of 1..101.
///
///     assert_eq!(rust_euler::problem6::solution(), 25_164_150);
///
pub fn solution() -> u64 {
    let n = 100;
    let sum_of_squares: u64 = (1..(n + 1)).map(|x| x * x).sum();
    let square_of_sum: u64 = (n * (n + 1) / 2).pow(2);
    square_of_sum - sum_of_squares
}
