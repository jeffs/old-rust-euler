//! Even Fibonacci numbers

/// Returns the sum of all even Fibonacci numbers not exceeding 4,000,000.
///
///     assert_eq!(rust_euler::problem2::solution(), 4_613_732);
///
pub fn solution() -> i32 {
    let mut r = 0;
    let mut a = 1;
    let mut b = 2;
    while b <= 4_000_000 {
        if b % 2 == 0 {
            r += b;
        }
        let c = a + b;
        a = b;
        b = c;
    }
    r
}
