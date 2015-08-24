use super::super::*;
use {bitcast, simd_cast, f32x2};

pub use sixty_four::{f64x2, i64x2, u64x2, bool64ix2, bool64fx2};

//pub use super::{u64x2, i64x2, f64x2, bool64ix2, bool64fx2};

#[allow(dead_code)]
extern "platform-intrinsic" {
    fn x86_mm_sqrt_ps(x: f32x4) -> f32x4;

    fn x86_mm_rsqrt_ps(x: f32x4) -> f32x4;
    fn x86_mm_rcp_ps(x: f32x4) -> f32x4;

    fn x86_mm_max_ps(x: f32x4, y: f32x4) -> f32x4;

    fn x86_mm_min_ps(x: f32x4, y: f32x4) -> f32x4;

    fn x86_mm_movemask_ps(x: f32x4) -> u32;
    fn x86_mm_movemask_pd(x: f64x2) -> u32;
    fn x86_mm_movemask_epi8(x: u8x16) -> u32;

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

#[doc(hidden)]
pub mod common {
    use super::super::super::*;
    use std::mem;

    #[inline]
    pub fn f32x4_sqrt(x: f32x4) -> f32x4 {
        unsafe {super::x86_mm_sqrt_ps(x)}
    }
    #[inline]
    pub fn f32x4_approx_rsqrt(x: f32x4) -> f32x4 {
        unsafe {super::x86_mm_rsqrt_ps(x)}
    }
    #[inline]
    pub fn f32x4_approx_reciprocal(x: f32x4) -> f32x4 {
        unsafe {super::x86_mm_rcp_ps(x)}
    }
    #[inline]
    pub fn f32x4_max(x: f32x4, y: f32x4) -> f32x4 {
        unsafe {super::x86_mm_max_ps(x, y)}
    }
    #[inline]
    pub fn f32x4_min(x: f32x4, y: f32x4) -> f32x4 {
        unsafe {super::x86_mm_min_ps(x, y)}
    }

    macro_rules! bools {
        ($($ty: ty, $all: ident, $any: ident, $movemask: ident, $width: expr;)*) => {
            $(
                #[inline]
                pub fn $all(x: $ty) -> bool {
                    unsafe {
                        super::$movemask(mem::transmute(x)) == (1 << $width) - 1
                    }
                }
                #[inline]
                pub fn $any(x: $ty) -> bool {
                    unsafe {
                        super::$movemask(mem::transmute(x)) != 0
                    }
                }
                )*
        }
    }

    bools! {
        bool32fx4, bool32fx4_all, bool32fx4_any, x86_mm_movemask_ps, 4;
        bool8ix16, bool8ix16_all, bool8ix16_any, x86_mm_movemask_epi8, 16;
        bool16ix8, bool16ix8_all, bool16ix8_any, x86_mm_movemask_epi8, 16;
        bool32ix4, bool32ix4_all, bool32ix4_any, x86_mm_movemask_epi8, 16;
    }
}

pub trait Sse2F32x4 {
    fn to_f64(self) -> f64x2;
    fn move_mask(self) -> u32;
}
impl Sse2F32x4 for f32x4 {
    #[inline]
    fn to_f64(self) -> f64x2 {
        unsafe {
            simd_cast(f32x2(self.0, self.1))
        }
    }
    fn move_mask(self) -> u32 {
        unsafe {x86_mm_movemask_ps(self)}
    }
}

pub trait Sse2F64x2 {
    fn move_mask(self) -> u32;
}
pub trait Sse2Bool32fx4 {
    fn move_mask(self) -> u32;
}
pub trait Sse2Bool64fx2 {
    fn move_mask(self) -> u32;
}
pub trait Sse2U8x16 {
    fn move_mask(self) -> u32;
}
pub trait Sse2I8x16 {
    fn move_mask(self) -> u32;
}
pub trait Sse2Bool8ix16 {
    fn move_mask(self) -> u32;
}
impl Sse2F64x2 for f64x2 {
    #[inline]
    fn move_mask(self) -> u32 {
        unsafe { x86_mm_movemask_pd(bitcast(self))}
    }
}
impl Sse2Bool32fx4 for bool32fx4 {
    #[inline]
    fn move_mask(self) -> u32 {
        unsafe { x86_mm_movemask_ps(bitcast(self))}
    }
}
impl Sse2Bool64fx2 for bool64fx2 {
    #[inline]
    fn move_mask(self) -> u32 {
        unsafe { x86_mm_movemask_pd(bitcast(self))}
    }
}
impl Sse2U8x16 for u8x16 {
    #[inline]
    fn move_mask(self) -> u32 {
        unsafe { x86_mm_movemask_epi8(self) }
    }
}
impl Sse2I8x16 for i8x16 {
    #[inline]
    fn move_mask(self) -> u32 {
        unsafe { x86_mm_movemask_epi8(bitcast(self)) }
    }
}
impl Sse2I8x16 for bool8ix16 {
    #[inline]
    fn move_mask(self) -> u32 {
        unsafe { x86_mm_movemask_epi8(bitcast(self)) }
    }
}
