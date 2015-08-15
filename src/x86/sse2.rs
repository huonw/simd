use super::super::*;
use {bitcast, Simd2, simd_cast};

pub use sixty_four::{f64x2, i64x2, u64x2, bool64ix2, bool64fx2};

//pub use super::{u64x2, i64x2, f64x2, bool64ix2, bool64fx2};

#[allow(dead_code)]
extern "platform-intrinsic" {
    fn x86_mm_sqrt_ps(x: f32x4) -> f32x4;

    fn x86_mm_rsqrt_ps(x: f32x4) -> f32x4;
    fn x86_mm_rcp_ps(x: f32x4) -> f32x4;

    fn x86_mm_max_ps(x: f32x4, y: f32x4) -> f32x4;

    fn x86_mm_min_ps(x: f32x4, y: f32x4) -> f32x4;

    fn x86_mm_movemask_ps(x: f32x4) -> i32;
    fn x86_mm_movemask_epi8(x: u8x16) -> i32;

    fn x86_mm_adds_epi16(x: i16x8, y: i16x8) -> i16x8;
    fn x86_mm_adds_epi8(x: i8x16, y: i8x16) -> i8x16;
    fn x86_mm_adds_epu16(x: i16x8, y: i16x8) -> i16x8;
    fn x86_mm_adds_epu8(x: i8x16, y: i8x16) -> i8x16;
    fn x86_mm_avg_epu16(x: i16x8, y: i16x8) -> i16x8;
    fn x86_mm_avg_epu8(x: i8x16, y: i8x16) -> i8x16;
    fn x86_mm_madd_epi16(x: i16x8, y: i16x8) -> i32x4;
    fn x86_mm_max_epi16(x: i16x8, y: i16x8) -> i16x8;
    fn x86_mm_max_epu8(x: i8x16, y: i8x16) -> i8x16;
    fn x86_mm_min_epi16(x: i16x8, y: i16x8) -> i16x8;
    fn x86_mm_min_epu8(x: i8x16, y: i8x16) -> i8x16;
    fn x86_mm_mul_epu32(x: i32x4, y: i32x4) -> i64x2;
    fn x86_mm_mulhi_epi16(x: i8x16, y: i8x16) -> i8x16;
    fn x86_mm_mulhi_epu16(x: i8x16, y: i8x16) -> i8x16;
    fn x86_mm_packs_epi16(x: i16x8, y: i16x8) -> i8x16;
    fn x86_mm_packs_epi32(x: i32x4, y: i32x4) -> i16x8;
    fn x86_mm_packus_epi16(x: i16x8, y: i16x8) -> i8x16;
    fn x86_mm_sad_epu8(x: i8x16, y: i8x16) -> i64x2;
    fn x86_mm_subs_epi16(x: i16x8, y: i16x8) -> i16x8;
    fn x86_mm_subs_epi8(x: i8x16, y: i8x16) -> i8x16;
    fn x86_mm_subs_epu16(x: i16x8, y: i16x8) -> i16x8;
    fn x86_mm_subs_epu8(x: i8x16, y: i8x16) -> i8x16;
}

impl f32x4 {
    #[inline]
    pub fn sqrt(self) -> f32x4 {
        unsafe {x86_mm_sqrt_ps(self)}
    }
    #[inline]
    pub fn approx_rsqrt(self) -> f32x4 {
        unsafe {x86_mm_rsqrt_ps(self)}
    }
    #[inline]
    pub fn approx_reciprocal(self) -> f32x4 {
        unsafe {x86_mm_rcp_ps(self)}
    }
    #[inline]
    pub fn max(self, other: f32x4) -> f32x4 {
        unsafe {x86_mm_max_ps(self, other)}
    }
    #[inline]
    pub fn min(self, other: f32x4) -> f32x4 {
        unsafe {x86_mm_min_ps(self, other)}
    }
}

macro_rules! bool_impls {
    ($($ty: ty, $movemask: ident, $width: expr;)*) => {
        $(impl $ty {
            #[inline]
            pub fn any(self) -> bool {
                unsafe {$movemask(bitcast(self)) != 0}
            }
            #[inline]
            pub fn all(self) -> bool {
                unsafe {$movemask(bitcast(self)) == (1 << $width) - 1}
            }
        })*
    }
}
bool_impls! {
    bool32fx4, x86_mm_movemask_ps, 4;

    bool8ix16, x86_mm_movemask_epi8, 16;
    bool16ix8, x86_mm_movemask_epi8, 16;
    bool32ix4, x86_mm_movemask_epi8, 16;
}

pub trait F32x4 {
    fn to_f64(self) -> f64x2;
}
impl F32x4 for f32x4 {
    #[inline]
    fn to_f64(self) -> f64x2 {
        unsafe {
            simd_cast(Simd2(self.0, self.1))
        }
    }
}
