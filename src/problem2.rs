//! Even Fibonacci numbers

struct FibonacciIter(u32, u32);

impl Iterator for FibonacciIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.0;
        self.0 = self.1;
        self.1 += r;
        Some(r)
    }
}

/// Returns the sum of all even Fibonacci numbers not exceeding 4,000,000.
///
///     assert_eq!(rust_euler::problem2::solution(), 4_613_732);
///
pub fn solution() -> u32 {
    FibonacciIter(1, 2)
        .take_while(|&i| i <= 4_000_000)
        .filter(|&i| i % 2 == 0)
        .sum()
}
