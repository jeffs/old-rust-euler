//! Utilities for finding prime numbers

use std::collections::{BTreeMap, BTreeSet};

/// Maps prime numbers to their exponents as factors of some number.  For example,
/// `[(2, 1), (3, 2), (5, 3)]` represents the number 2250, because 2250 == 2<sup>1</sup> *
/// 3<sup>2</sup> * 5<sup>3</sup>.
pub type PowerMap = BTreeMap<u64, u32>;

/// Constructs a map from the specified key/value `pairs`.
///
///     use rust_euler::prime::{to_map, PowerMap};
///
///     assert_eq!(to_map(&[]), PowerMap::new());
///
pub fn to_map(pairs: &[(u64, u32)]) -> PowerMap {
    pairs.into_iter().cloned().collect()
}

/// Returns the prime factors of `n`, paired with their exponents.
///
///     use rust_euler::prime::{factor, to_map, PowerMap};
///
///     assert_eq!(factor(0), PowerMap::new());
///     assert_eq!(factor(1), PowerMap::new());
///     assert_eq!(factor(2), to_map(&[(2, 1)]));
///     assert_eq!(factor(24), to_map(&[(2, 3), (3, 1)]));
///     assert_eq!(factor(42), to_map(&[(2, 1), (3, 1), (7, 1)]));
///
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

/// Returns the number that is the product of the specified powers of primes.
///
///     use rust_euler::prime::{from_factors, to_map, PowerMap};
///
///     assert_eq!(from_factors(&PowerMap::new()), 1);
///     assert_eq!(from_factors(&to_map(&[(2, 3), (3, 1)])), 24);
///
pub fn from_factors(m: &PowerMap) -> u64 {
    let mut r = 1;
    for (&k, &v) in m.iter() {
        r *= k.pow(v)
    }
    r
}

/// Returns true if none of the candidate divisors (`ds`) evenly divides `n`.
fn none_divides(ds: &Vec<u64>, n: u64) -> bool {
    ds.iter().find(|&d| n % d == 0).is_none()
}

/// Returns the first `n` prime numbers.
///
///     use rust_euler::prime;
///
///     assert_eq!(prime::generate(0), Vec::new());
///     assert_eq!(prime::generate(5), vec![2, 3, 5, 7, 11]);
///
pub fn generate(n: usize) -> Vec<u64> {
    if n == 0 {
        return Vec::new();
    }
    let mut ks = [2].to_vec(); // known primes
    let mut i = 3;
    while ks.len() < n {
        if none_divides(&ks, i) {
            ks.push(i);
        }
        i += 2;
    }
    ks
}

/// Returns all prime numbers strictly less than `end`.
///
///     use rust_euler::prime;
///
///     assert_eq!(prime::generate_to(2), Vec::new());
///     assert_eq!(prime::generate_to(13), vec![2, 3, 5, 7, 11]);
///
pub fn generate_to(end: u64) -> BTreeSet<u64> {
    fn to_set(xs: &[u64]) -> BTreeSet<u64> {
        xs.iter().cloned().collect()
    }
    match end {
        0...2 => BTreeSet::new(),
        3 => to_set(&[2]),
        4 => to_set(&[2, 3]),
        _ => {
            let mut r = 1..(end / 2)
            r
        }
    }
}

/// Returns the nth prime number, where `n` is a 0-based index.
///
///     use rust_euler::prime;
///
///     assert_eq!(prime::nth(0), 2);
///     assert_eq!(prime::nth(1), 3);
///     assert_eq!(prime::nth(2), 5);
///
pub fn nth(n: usize) -> u64 {
    *generate(n + 1).last().unwrap()
}
