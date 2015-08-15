use super::super::*;
use x86::sse2::*;

#[allow(dead_code)]
extern "platform-intrinsic" {
    fn x86_mm_max_epi32(x: i32x4, y: i32x4) -> i32x4;
    fn x86_mm_max_epi8(x: i8x16, y: i8x16) -> i8x16;
    fn x86_mm_max_epu16(x: i16x8, y: i16x8) -> i16x8;
    fn x86_mm_max_epu32(x: i32x4, y: i32x4) -> i32x4;
    fn x86_mm_min_epi32(x: i32x4, y: i32x4) -> i32x4;
    fn x86_mm_min_epi8(x: i8x16, y: i8x16) -> i8x16;
    fn x86_mm_min_epu16(x: i16x8, y: i16x8) -> i16x8;
    fn x86_mm_min_epu32(x: i32x4, y: i32x4) -> i32x4;
    fn x86_mm_minpos_epu16(x: i16x8) -> i16x8;
    fn x86_mm_mul_epi32(x: i32x4, y: i32x4) -> i64x2;
    fn x86_mm_packus_epi32(x: i32x4, y: i32x4) -> i16x8;
    fn x86_mm_testc_si128(x: i64x2, y: i64x2) -> i32;
    fn x86_mm_testnzc_si128(x: i64x2, y: i64x2) -> i32;
    fn x86_mm_testz_si128(x: i64x2, y: i64x2) -> i32;
}
