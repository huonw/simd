use super::*;
use super::bitcast;

#[allow(dead_code)]
extern "platform-intrinsic" {
    fn x86_mm_abs_epi16(x: i16x8) -> i16x8;
    fn x86_mm_abs_epi32(x: i32x4) -> i32x4;
    fn x86_mm_abs_epi8(x: i8x16) -> i8x16;
    fn x86_mm_hadd_epi16(x: i16x8, y: i16x8) -> i16x8;
    fn x86_mm_hadd_epi32(x: i32x4, y: i32x4) -> i32x4;
    fn x86_mm_hadds_epi16(x: i16x8, y: i16x8) -> i16x8;
    fn x86_mm_hsub_epi16(x: i16x8, y: i16x8) -> i16x8;
    fn x86_mm_hsub_epi32(x: i32x4, y: i32x4) -> i32x4;
    fn x86_mm_hsubs_epi16(x: i16x8, y: i16x8) -> i16x8;
    fn x86_mm_maddubs_epi16(x: i8x16, y: i8x16) -> i16x8;
    fn x86_mm_mulhrs_epi16(x: i16x8, y: i16x8) -> i16x8;
    fn x86_mm_shuffle_epi8(x: u8x16, y: u8x16) -> u8x16;
    fn x86_mm_sign_epi16(x: i16x8, y: i16x8) -> i16x8;
    fn x86_mm_sign_epi32(x: i32x4, y: i32x4) -> i32x4;
    fn x86_mm_sign_epi8(x: i8x16, y: i8x16) -> i8x16;
}

pub trait SSSE3Bytes {
    fn shuf(self, indices: Self) -> Self;
}

impl SSSE3Bytes for u8x16 {
    #[inline]
    fn shuf(self, indices: Self) -> Self {
        unsafe {x86_mm_shuffle_epi8(self, indices)}
    }
}

impl SSSE3Bytes for i8x16 {
    #[inline]
    fn shuf(self, indices: Self) -> Self {
        unsafe {
            bitcast(x86_mm_shuffle_epi8(bitcast(self),
                                        bitcast(indices)))
        }
    }
}
