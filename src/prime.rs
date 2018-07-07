//! Utilities for finding prime numbers

use std::collections::BTreeMap;

/// Maps prime numbers to their exponents as factors of some number.  For example,
/// `[(2, 1), (3, 2), (5, 3)]` represents the number 2250, because 2250 == 2<sup>1</sup> *
/// 3<sup>2</sup> * 5<sup>3</sup>.
pub type PowerMap = BTreeMap<u64, u32>;

/// Returns the prime factors of `n`, paired with their exponents.
pub fn factor(mut n: u64) -> PowerMap {
    let mut r = PowerMap::new();
    let mut d = 2;
    while d <= n {
        if n % d == 0 {
            let mut e = 0;
            while n % d == 0 {
                e += 1;
                n /= d;
            }
            r.insert(d, e);
        }
        d += 1;
    }
    r
}

pub fn from_factors(m: &PowerMap) -> u64 {
    let mut r = 1;
    for (&k, &v) in m.iter() {
        r *= k.pow(v)
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_map(kvs: &[(u64, u32)]) -> PowerMap {
        kvs.into_iter().cloned().collect()
    }

    #[test]
    fn test_prime_factor() {
        assert_eq!(factor(0), PowerMap::new());
        assert_eq!(factor(1), PowerMap::new());
        assert_eq!(factor(2), to_map(&[(2, 1)]));
        assert_eq!(factor(24), to_map(&[(2, 3), (3, 1)]));
        assert_eq!(factor(42), to_map(&[(2, 1), (3, 1), (7, 1)]));
    }
}
