extern crate test;
use super::Bloom;
use test::Bencher;

// launch with `cargo bench --features=bench`
// only works with nightly, still marked unstable

#[bench]
fn bench_bloom_set_check_10_80(b: &mut Bencher) {
    let mut bloom = Bloom::new(10, 80);
    let mut i: usize = 0;
    b.iter(|| {
        bloom.set(&i);
        _ = bloom.check(&i);
        i += 1;
    })
}

#[bench]
fn bench_bloom_set_check_fp_rate_0_1(b: &mut Bencher) {
    let mut bloom = Bloom::new_for_fp_rate(1_000_000_000, 0.01);
    let mut i: usize = 0;
    b.iter(|| {
        bloom.set(&i);
        _ = bloom.check(&i);
        i += 1;
    })
}
