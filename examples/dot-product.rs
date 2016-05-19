extern crate simd;
use simd::f32x4;

#[inline(never)]
pub fn dot(x: &[f32], y: &[f32]) -> f32 {
    assert_eq!(x.len(), y.len());

    let len = std::cmp::min(x.len(), y.len());

    let mut sum = f32x4::splat(0.0);
    let mut i = 0;
    while i < len & !3 {
        let x = f32x4::load(x, i);
        let y = f32x4::load(y, i);
        sum = sum + x * y;
        i += 4
    }
    sum.extract(0) + sum.extract(1) + sum.extract(2) + sum.extract(3)
}

fn main() {
    println!("{}", dot(&[1.0, 3.0, 5.0, 7.0], &[2.0, 4.0, 6.0, 8.0]));
    println!("{}", dot(&[1.0, 3.0, 6.0, 7.0, 10.0, 6.0, 3.0, 2.0],
                       &[2.0, 4.0, 6.0, 8.0, 2.0, 4.0, 6.0, 8.0]));
}
