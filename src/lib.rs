#![feature(test)]

extern crate test;
use test::Bencher;

use dasp::Frame;

#[bench]
fn bench_frame_offset_amp(b: &mut Bencher) {
    b.iter(|| {
        let fa = [-0.50, -0.25, 0.00, 0.25, 0.5];
        fa.offset_amp(0.5)
    });
}
