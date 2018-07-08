#![feature(test)]

extern crate test;

pub mod problem1;
pub mod problem2;
pub mod problem3;
pub mod problem4;
pub mod problem5;
pub mod problem6;
pub mod problem7;

pub mod bit_vec;
pub mod prime;

#[cfg(test)]
mod tests {
    use test::Bencher;

    use prime;

    #[bench]
    fn bench_prime_generate10(b: &mut Bencher) {
        b.iter(|| { prime::generate(10); });
    }

    #[bench]
    fn bench_prime_generate100(b: &mut Bencher) {
        b.iter(|| { prime::generate(100); });
    }

    #[bench]
    fn bench_prime_generate1000(b: &mut Bencher) {
        b.iter(|| { prime::generate(1000); });
    }

    #[bench]
    fn bench_prime_generate10000(b: &mut Bencher) {
        b.iter(|| { prime::generate(10000); });
    }
}
