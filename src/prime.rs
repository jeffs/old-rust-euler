//! Utilities for finding prime numbers

/// Returns the prime factors of `n`, paired with their exponents.
pub fn factor(mut n: u64) -> Vec<(u64, u64)> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_factor() {
        assert_eq!(factor(0), vec![]);
        assert_eq!(factor(1), vec![]);
        assert_eq!(factor(2), vec![(2, 1)]);
        assert_eq!(factor(24), vec![(2, 3), (3, 1)]);
        assert_eq!(factor(42), vec![(2, 1), (3, 1), (7, 1)]);
    }
}
