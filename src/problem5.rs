//! Smallest multiple
//!
//! > 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without
//! > any remainder.
//! >
//! > What is the smallest positive number that is evenly divisible by all of the numbers from 1 to
//! > 20?

use prime::{factor, from_factors, PowerMap};

/// Returns the smallest positive integer divisible by all integers in the range 1..21.
pub fn solution() -> u64 {
    // prime-factor each number in 1..21 into a prime => exponent map
    // for each key in the union of the keys of all those maps
    //  map it to the max of all the corresponding values
    // convert the final (reduced) power-map to an integer

    let mut r = PowerMap::new();
    for m in (1..21u64).map(|i| factor(i)) {
        for (&k, &v) in m.iter() {
            r.entry(k).and_modify(|w| *w = v.max(*w)).or_insert(v);
        }
    }
    from_factors(&r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(), 232_792_560);
    }
}
