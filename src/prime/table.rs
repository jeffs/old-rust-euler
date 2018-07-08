fn to_index(i: u64) -> usize {
    ((i - 3) / 2) as usize
}

/// Maps numbers to bools indicating whether those numbers are prime.
pub struct Table {
    bits: Vec<bool>,
}

impl Table {

    pub fn new(size: usize) -> Table {
        let mut bits = vec![true; size];
        if size > 3 {
            // `m` and `n` are candidate primes.
            // `i` and `j` are their indexes in the bit-vector.
            let mut m = 3;
            while m * m <= size as u64 {
                let i = to_index(m);
                if bits[i] {
                    let mut n = m + m + m;
                    let mut j = to_index(n);
                    while j < bits.len() {
                        bits[j] = false;
                        n += m + m;
                        j = to_index(n);
                    }
                }
                m += 2;
            }
        }
        Table { bits }
    }

    /// Returns true if `n` is prime, and false otherwise.  `n` must be strictly less than the
    /// size supplied on construction.
    ///
    ///     let prime_table = rust_euler::prime::Table::new(100);
    ///
    ///     assert!(!prime_table.is_prime(0));
    ///     assert!(!prime_table.is_prime(1));
    ///     assert!(prime_table.is_prime(2));
    ///     assert!(prime_table.is_prime(3));
    ///     assert!(!prime_table.is_prime(4));
    ///     assert!(prime_table.is_prime(5));
    ///     assert!(prime_table.is_prime(79));
    ///
    pub fn is_prime(&self, n: u64) -> bool {
        match n {
            1 => false,
            2 => true,
            _ => n % 2 != 0 && self.bits[to_index(n)],
        }
    }
}
