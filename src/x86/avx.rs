use super::super::*;
use sixty_four::*;

pub use v256::{
    f64x4, bool64fx4, u64x4, i64x4, bool64ix4,
    f32x8, bool32fx8, u32x8, i32x8, bool32ix8,
    u16x16, i16x16, bool16ix16,
    u8x32, i8x32, bool8ix32,
};

#[allow(dead_code)]
extern "platform-intrinsic" {
    fn x86_mm256_addsub_ps(x: f32x8, y: f32x8) -> f32x8;
    fn x86_mm256_addsub_pd(x: f64x4, y: f64x4) -> f64x4;
    fn x86_mm256_dp_ps(x: f32x8, y: f32x8, z: i32) -> f32x8;
    fn x86_mm256_hadd_ps(x: f32x8, y: f32x8) -> f32x8;
    fn x86_mm256_hadd_pd(x: f64x4, y: f64x4) -> f64x4;
    fn x86_mm256_hsub_ps(x: f32x8, y: f32x8) -> f32x8;
    fn x86_mm256_hsub_pd(x: f64x4, y: f64x4) -> f64x4;
    fn x86_mm256_max_ps(x: f32x8, y: f32x8) -> f32x8;
    fn x86_mm256_max_pd(x: f64x4, y: f64x4) -> f64x4;
    fn x86_mm256_min_ps(x: f32x8, y: f32x8) -> f32x8;
    fn x86_mm256_min_pd(x: f64x4, y: f64x4) -> f64x4;
    fn x86_mm256_movemask_ps(x: f32x8) -> i32;
    fn x86_mm256_movemask_pd(x: f64x4) -> i32;
    fn x86_mm_permutevar_ps(x: f32x4, y: i32x4) -> f32x4;
    fn x86_mm_permutevar_pd(x: f64x2, y: i64x2) -> f64x2;
    fn x86_mm256_permutevar_ps(x: f32x8, y: i32x8) -> f32x8;
    fn x86_mm256_permutevar_pd(x: f64x4, y: i64x4) -> f64x4;
    fn x86_mm256_rcp_ps(x: f32x8) -> f32x8;
    fn x86_mm256_rsqrt_ps(x: f32x8) -> f32x8;
    fn x86_mm256_sqrt_ps(x: f32x8) -> f32x8;
    fn x86_mm256_sqrt_pd(x: f64x4) -> f64x4;
    fn x86_mm_testc_ps(x: f32x4, y: f32x4) -> i32;
    fn x86_mm256_testc_ps(x: f32x8, y: f32x8) -> i32;
    fn x86_mm_testc_pd(x: f64x2, y: f64x2) -> i32;
    fn x86_mm256_testc_pd(x: f64x4, y: f64x4) -> i32;
    fn x86_mm256_testc_si256(x: u64x4, y: u64x4) -> i32;
    fn x86_mm_testnzc_ps(x: f32x4, y: f32x4) -> i32;
    fn x86_mm256_testnzc_ps(x: f32x8, y: f32x8) -> i32;
    fn x86_mm_testnzc_pd(x: f64x2, y: f64x2) -> i32;
    fn x86_mm256_testnzc_pd(x: f64x4, y: f64x4) -> i32;
    fn x86_mm256_testnzc_si256(x: u64x4, y: u64x4) -> i32;
    fn x86_mm_testz_ps(x: f32x4, y: f32x4) -> i32;
    fn x86_mm256_testz_ps(x: f32x8, y: f32x8) -> i32;
    fn x86_mm_testz_pd(x: f64x2, y: f64x2) -> i32;
    fn x86_mm256_testz_pd(x: f64x4, y: f64x4) -> i32;
    fn x86_mm256_testz_si256(x: u64x4, y: u64x4) -> i32;
}
