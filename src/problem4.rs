//! Largest palindrome product
//!
//! > A palindromic number reads the same both ways. The largest palindrome made from the product
//! > of two 2-digit numbers is 9009 = 91 × 99.
//! >
//! > Find the largest palindrome made from the product of two 3-digit numbers.

/// Returns the sequence of digit values (not character representations) representing `n` in base
/// `b`, in reverse order.
fn to_digits_reverse(mut n: u32, b: u32) -> Vec<u32> {
    let mut r = vec![];
    while n > 0 {
        r.push(n % b);
        n /= b;
    }
    r
}

fn is_palindrome_base10(n: u32) -> bool {
    let ds = to_digits_reverse(n, 10);
    for i in 0..(ds.len() / 2) {
        if ds[i] != ds[ds.len() - i - 1] {
            return false;
        }
    }
    true
}

/// Returns the largest palindrome that is the product of two 3-digit numbers.
pub fn solution() -> u32 {
    let mut r = None;
    for i in 100..1000 {
        for j in i..1000 {
            let k = i * j;
            if is_palindrome_base10(k) {
                r = match r {
                    None => Some(k),
                    Some(m) => Some(m.max(k)),
                }
            }
        }
    }
    r.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_digits_reverse() {
        assert_eq!(to_digits_reverse(8675309, 10), vec![9, 0, 3, 5, 7, 6, 8]);
    }

    #[test]
    fn test_solution() {
        assert_eq!(solution(), 906609);
    }
}
