//! Multiples of 3 and 5

/// Returns the sum of all multiples of factor in the range 1..end.
fn sum_multiples(factor: u32, end: u32) -> u32 {
    let count = (end - 1) / factor;
    let multiplicand = count * (count + 1) / 2; // sum of 1..end
    multiplicand * factor
}

/// Returns the sum of all multiples of 3 or 5 in the range 1..1000.
///
///     assert_eq!(rust_euler::problem1::solution(), 233168);
///
pub fn solution() -> u32 {
    let f = |n| sum_multiples(n, 1000);
    f(3) + f(5) - f(15)
}
