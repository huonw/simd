#![feature(test)]

extern crate simd;
extern crate test;

use test::black_box as bb;
use test::Bencher as B;
use simd::{f32x4, u32x4};

fn naive(c_x: f32, c_y: f32, max_iter: u32) -> u32 {
    let mut x = c_x;
    let mut y = c_y;
    let mut count = 0;
    while count < max_iter {
        let xy = x * y;
        let xx = x * x;
        let yy = y * y;
        let sum = xx + yy;
        if sum > 4.0 {
            break
        }
        count += 1;
        x = xx - yy + c_x;
        y = xy * 2.0 + c_y;
    }
    count
}

fn simd4(c_x: f32x4, c_y: f32x4, max_iter: u32) -> u32x4 {
    let mut x = c_x;
    let mut y = c_y;

    let mut count = u32x4::splat(0);
    for _ in 0..max_iter as usize {
        let xy = x * y;
        let xx = x * x;
        let yy = y * y;
        let sum = xx + yy;
        let mask = sum.lt(f32x4::splat(4.0));

        if !mask.any() { break }
        count = count + mask.to_i().select(u32x4::splat(1), u32x4::splat(0));

        x = xx - yy + c_x;
        y = xy + xy + c_y;
    }
    count
}

const SCALE: f32 = 3.0 / 100.0;
const N: u32 = 100;
#[bench]
fn mandel_naive(b: &mut B) {
    b.iter(|| {
        for j in 0..100 {
            let y = -1.5 + (j as f32) * SCALE;
            for i in 0..100 {
                let x = -2.2 + (i as f32) * SCALE;
                bb(naive(x, y, N));
            }
        }
    })
}
#[bench]
fn mandel_simd4(b: &mut B) {
    let tweak = u32x4::new(0, 1, 2, 3);
    b.iter(|| {
        for j in 0..100 {
            let y = f32x4::splat(-1.5) + f32x4::splat(SCALE) * u32x4::splat(j).to_f32();
            for i in 0..25 {
                let i = u32x4::splat(i * 4) + tweak;
                let x = f32x4::splat(-2.2) + f32x4::splat(SCALE) * i.to_f32();
                bb(simd4(x, y, N));
            }
        }
    })
}
