#![feature(test)]

extern crate test;
use test::Bencher;

use dasp::Frame;

#[bench]
fn bench_frame_offset_amp(b: &mut Bencher) {
    b.iter(|| {
        let fa = [-0.500, -0.250, 0.000, 0.250, 0.500];
        fa.offset_amp(0.5)
    });
}

#[bench]
fn bench_frame_scale_amp(b: &mut Bencher) {
    b.iter(|| {
        let fa = [-0.500, -0.250, 0.000, 0.250, 0.500];
        fa.scale_amp(0.5)
    });
}

#[bench]
fn bench_frame_add_amp(b: &mut Bencher) {
    b.iter(|| {
        let fa = [-0.500, -0.250, 0.000, 0.250, 0.500];
        let fb = [0.250, 0.125, 0.000, -0.125, -0.250];
        fa.add_amp(fb)
    });
}

#[bench]
fn bench_frame_mul_amp(b: &mut Bencher) {
    b.iter(|| {
        let fa = [-0.500, -0.250, 0.000, 0.250, 0.500];
        let fb = [0.250, 0.125, 0.000, -0.125, -0.250];
        fa.mul_amp(fb)
    });
}
