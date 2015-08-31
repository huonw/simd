use super::super::*;
use x86::sse2::*;

#[allow(dead_code)]
extern "platform-intrinsic" {
    fn x86_mm_dp_ps(x: f32x4, y: f32x4, z: i32) -> f32x4;
    fn x86_mm_dp_pd(x: f64x2, y: f64x2, z: i32) -> f64x2;
    fn x86_mm_max_epi8(x: i8x16, y: i8x16) -> i8x16;
    fn x86_mm_max_epu16(x: u16x8, y: u16x8) -> u16x8;
    fn x86_mm_max_epi32(x: i32x4, y: i32x4) -> i32x4;
    fn x86_mm_max_epu32(x: u32x4, y: u32x4) -> u32x4;
    fn x86_mm_min_epi8(x: i8x16, y: i8x16) -> i8x16;
    fn x86_mm_min_epu16(x: u16x8, y: u16x8) -> u16x8;
    fn x86_mm_min_epi32(x: i32x4, y: i32x4) -> i32x4;
    fn x86_mm_min_epu32(x: u32x4, y: u32x4) -> u32x4;
    fn x86_mm_minpos_epu16(x: u16x8) -> u16x8;
    fn x86_mm_mpsadbw_epu8(x: u8x16, y: u8x16, z: i32) -> u16x8;
    fn x86_mm_mul_epi32(x: i32x4, y: i32x4) -> i64x2;
    fn x86_mm_packus_epi32(x: i32x4, y: i32x4) -> u16x8;
    fn x86_mm_testc_si128(x: u64x2, y: u64x2) -> i32;
    fn x86_mm_testncz_si128(x: u64x2, y: u64x2) -> i32;
    fn x86_mm_testz_si128(x: u64x2, y: u64x2) -> i32;
}
