#![allow(dead_code, unused_variables)]

fn sign(n: usize) -> usize {
    if n == 0 {
        0
    } else {
        1
    }
}

/// Returns the minimum number of bytes actually needed to store `bit_count` bits.
fn min_bytes(bit_count: usize) -> usize {
    bit_count / 8 + sign(bit_count % 8)
}

/// Dynamically sized sequence of bits.
pub struct BitVec {
    bytes: Vec<u8>, // storage; bit capacity is `bytes.len() * 8`
    size: usize,    // number of used bits
}

impl BitVec {
    fn assign_trailing_bits(&mut self, value: bool) {
        assert_eq!(self.bytes.len(), min_bytes(self.size));
        if self.size % 8 != 0 {
            let mask = (1 << (self.size % 8)) - 1;
            if let Some(byte) = self.bytes.last_mut() {
                if value {
                    *byte |= !mask;
                } else {
                    *byte &= mask;
                }
            }
        }
    }

    /// Returns the total number of bits this vector can store without reallocating (including any
    /// bits that are already in use).
    fn capacity(&self) -> usize {
        self.bytes.len() * 8
    }

    pub fn new() -> BitVec {
        BitVec {
            bytes: Vec::new(),
            size: 0,
        }
    }

    pub fn with_all(value: bool, size: usize) -> BitVec {
        let byte_count = min_bytes(size);
        let byte_value = if value { !0 } else { 0 };
        BitVec {
            bytes: vec![byte_value; byte_count],
            size,
        }
    }

    pub fn pushn(&mut self, value: bool, count: usize) {
        let byte_value = if value { !0 } else { 0 };
        self.bytes.resize(min_bytes(self.size), byte_value); // shrink
        self.assign_trailing_bits(value);
        self.size += count;
        self.bytes.resize(min_bytes(self.size), byte_value); // grow
    }

    pub fn set(&mut self, index: usize) {
        assert!(index < self.size);
        let byte = &mut self.bytes[index / 8];
        let mask = 1 << (index % 8);
        *byte |= mask;
    }

    pub fn get(&self, index: usize) -> bool {
        assert!(index < self.size);
        let byte = self.bytes[index / 8];
        let mask = 1 << (index % 8);
        byte & mask != 0
    }

    pub fn len(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(BitVec::new().len(), 0);
    }

    #[test]
    fn test_with_all() {
        assert_eq!(BitVec::with_all(true, 0).len(), 0);
        assert_eq!(BitVec::with_all(false, 1).len(), 1);
        assert_eq!(BitVec::with_all(true, 2).len(), 2);
        let fs = BitVec::with_all(false, 3);
        for i in 0..2 {
            assert!(!fs.get(i));
        }
        let ts = BitVec::with_all(true, 4);
        for i in 0..3 {
            assert!(ts.get(i));
        }
    }

    #[test]
    fn test_pushn() {
        let mut v = BitVec::new();
        v.pushn(false, 8);
        v.pushn(false, 0);
        v.pushn(true, 0);
        v.pushn(true, 6);
        v.pushn(false, 7);
        v.pushn(true, 5);
        let v = v;
        assert_eq!(v.len(), 8 + 0 + 0 + 6 + 7 + 5);
        assert!(!v.get(0));
        assert!(!v.get(1));
        assert!(!v.get(2));
        assert!(!v.get(3));
        assert!(!v.get(4));
        assert!(!v.get(5));
        assert!(!v.get(6));
        assert!(!v.get(7));
        assert!(v.get(8));
        assert!(v.get(9));
        assert!(v.get(10));
        assert!(v.get(11));
        assert!(v.get(12));
        assert!(v.get(13));
        assert!(!v.get(14));
        assert!(!v.get(15));
        assert!(!v.get(16));
        assert!(!v.get(17));
        assert!(!v.get(18));
        assert!(!v.get(19));
        assert!(!v.get(20));
        assert!(v.get(21));
        assert!(v.get(22));
        assert!(v.get(23));
        assert!(v.get(24));
        assert!(v.get(25));
    }

    #[test]
    fn test_capacity() {
        let mut v = BitVec::new();
        assert_eq!(v.capacity(), 0);
        v.pushn(false, 1);
        assert_eq!(v.capacity(), 8);
        v.pushn(false, 2);
        assert_eq!(v.capacity(), 8);
        assert_eq!(BitVec::with_all(true, 0).capacity(), 0);
        assert_eq!(BitVec::with_all(true, 1).capacity(), 8);
        assert_eq!(BitVec::with_all(true, 2).capacity(), 8);
        assert_eq!(BitVec::with_all(true, 16).capacity(), 16);
        assert_eq!(BitVec::with_all(true, 17).capacity(), 24);
    }

    #[test]
    fn test_set() {
        let mut v = BitVec::with_all(false, 11);
        v.set(4);
        v.set(9);
        assert!(!v.get(0));
        assert!(!v.get(1));
        assert!(!v.get(2));
        assert!(!v.get(3));
        assert!(v.get(4));
        assert!(!v.get(5));
        assert!(!v.get(6));
        assert!(!v.get(7));
        assert!(!v.get(8));
        assert!(v.get(9));
        assert!(!v.get(10));
    }
}
