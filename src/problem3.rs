//! Largest prime factor
//!
//! > The prime factors of 13195 are 5, 7, 13 and 29.
//! >
//! > What is the largest prime factor of the number 600851475143 ?

/// Returns pairs of prime factors of `n` and their exponents.
fn prime_factor(mut n: u64) -> Vec<(u64, usize)> {
    let mut r = Vec::new();
    let mut d = 2;
    while d <= n {
        if n % d == 0 {
            let mut e = 0;
            while n % d == 0 {
                e += 1;
                n /= d;
            }
            r.push((d, e));
        }
        d += 1;
    }
    r
}

/// Returns the largest prime factor of 600_851_475_143.
///
///     assert_eq!(rust_euler::problem3::solution(), 6857);
///
pub fn solution() -> u64 {
    prime_factor(600_851_475_143).last().unwrap().0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_factor() {
        assert_eq!(prime_factor(0), vec![]);
        assert_eq!(prime_factor(1), vec![]);
        assert_eq!(prime_factor(2), vec![(2, 1)]);
        assert_eq!(prime_factor(24), vec![(2, 3), (3, 1)]);
        assert_eq!(prime_factor(42), vec![(2, 1), (3, 1), (7, 1)]);
    }
}
