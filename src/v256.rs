use std::ops;
use std::mem;
#[allow(unused_imports)]
use super::{
	Simd,
    f32x2,
    simd_eq, simd_ne, simd_lt, simd_le, simd_gt, simd_ge,
    simd_shuffle2, simd_shuffle4, simd_shuffle8, simd_shuffle16,
    simd_insert, simd_extract,
    simd_cast,
    simd_add, simd_sub, simd_mul, simd_div, simd_shl, simd_shr, simd_and, simd_or, simd_xor,
    bool8i, bool16i, bool32i, bool32f,
    i16x8, u16x8,
    Unalign, bitcast,
};
use super::sixty_four::*;
#[cfg(all(target_feature = "avx"))]
use super::x86::avx::common;

#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct u64x4(u64, u64, u64, u64);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct i64x4(i64, i64, i64, i64);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct f64x4(f64, f64, f64, f64);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct bool64ix4(i64, i64, i64, i64);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct bool64fx4(i64, i64, i64, i64);

#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct u32x8(u32, u32, u32, u32,
                 u32, u32, u32, u32);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct i32x8(i32, i32, i32, i32,
                 i32, i32, i32, i32);
#[repr(simd)]
#[cfg_attr(feature = "serde", derfve(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct f32x8(f32, f32, f32, f32,
                 f32, f32, f32, f32);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct bool32ix8(i32, i32, i32, i32,
                     i32, i32, i32, i32);#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct bool32fx8(i32, i32, i32, i32,
                     i32, i32, i32, i32);

#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct u16x16(u16, u16, u16, u16, u16, u16, u16, u16,
                  u16, u16, u16, u16, u16, u16, u16, u16);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct i16x16(i16, i16, i16, i16, i16, i16, i16, i16,
                  i16, i16, i16, i16, i16, i16, i16, i16);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct bool16ix16(i16, i16, i16, i16, i16, i16, i16, i16,
                      i16, i16, i16, i16, i16, i16, i16, i16);

#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct u8x32(u8, u8, u8, u8, u8, u8, u8, u8,
                 u8, u8, u8, u8, u8, u8, u8, u8,
                 u8, u8, u8, u8, u8, u8, u8, u8,
                 u8, u8, u8, u8, u8, u8, u8, u8);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct i8x32(i8, i8, i8, i8, i8, i8, i8, i8,
                 i8, i8, i8, i8, i8, i8, i8, i8,
                 i8, i8, i8, i8, i8, i8, i8, i8,
                 i8, i8, i8, i8, i8, i8, i8, i8);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct bool8ix32(i8, i8, i8, i8, i8, i8, i8, i8,
                     i8, i8, i8, i8, i8, i8, i8, i8,
                     i8, i8, i8, i8, i8, i8, i8, i8,
                     i8, i8, i8, i8, i8, i8, i8, i8);

simd! {
    bool8ix32: i8x32 = i8, u8x32 = u8, bool8ix32 = bool8i;
    bool16ix16: i16x16 = i16, u16x16 = u16, bool16ix16 = bool16i;
    bool32ix8: i32x8 = i32, u32x8 = u32, bool32ix8 = bool32i;
    bool64ix4: i64x4 = i64, u64x4 = u64, bool64ix4 = bool64i;

    bool32fx8: f32x8 = f32, bool32fx8 = bool32f;
    bool64fx4: f64x4 = f64, bool64fx4 = bool64f;
}

basic_impls! {
    u64x4: u64, bool64ix4, simd_shuffle4, 4, x0, x1 | x2, x3;
    i64x4: i64, bool64ix4, simd_shuffle4, 4, x0, x1 | x2, x3;
    f64x4: f64, bool64fx4, simd_shuffle4, 4, x0, x1 | x2, x3;

    u32x8: u32, bool32ix8, simd_shuffle8, 8, x0, x1, x2, x3 | x4, x5, x6, x7;
    i32x8: i32, bool32ix8, simd_shuffle8, 8, x0, x1, x2, x3 | x4, x5, x6, x7;
    f32x8: f32, bool32fx8, simd_shuffle8, 8, x0, x1, x2, x3 | x4, x5, x6, x7;

    u16x16: u16, bool16ix16, simd_shuffle16, 16, x0, x1, x2, x3, x4, x5, x6, x7 | x8, x9, x10, x11, x12, x13, x14, x15;
    i16x16: i16, bool16ix16, simd_shuffle16, 16, x0, x1, x2, x3, x4, x5, x6, x7 | x8, x9, x10, x11, x12, x13, x14, x15;

    // u8x16: u8, bool8ix16, simd_shuffle16, 16, x0, x1, x2, x3, x4, x5, x6, x7 | x8, x9, x10, x11, x12, x13, x14, x15;
    // i8x16: i8, bool8ix16, simd_shuffle16, 16, x0, x1, x2, x3, x4, x5, x6, x7 | x8, x9, x10, x11, x12, x13, x14, x15;
}

#[cfg(all(not(target_feature = "avx")))]
#[doc(hidden)]
mod common {
    use super::*;
    // naive implementations for now;
    // this could be improved by treating lower and upper halfs as SSE vectors
    #[inline]
    pub fn bool64ix4_all(x: bool64ix4) -> bool {
        x.0 != 0 && x.1 != 0 && x.2 != 0 && x.3 != 0
    }
    #[inline]
    pub fn bool64ix4_any(x: bool64ix4) -> bool {
        x.0 != 0 || x.1 != 0 || x.2 != 0 || x.3 != 0
    }
    #[inline]
    pub fn bool64fx4_all(x: bool64fx4) -> bool {
        x.0 != 0 && x.1 != 0 && x.2 != 0 && x.3 != 0
    }
    #[inline]
    pub fn bool64fx4_any(x: bool64fx4) -> bool {
        x.0 != 0 || x.1 != 0 || x.2 != 0 || x.3 != 0
    }
    #[inline]
    pub fn bool32ix8_all(x: bool32ix8) -> bool {
        x.0 != 0 && x.1 != 0 && x.2 != 0 && x.3 != 0 &&
        x.4 != 0 && x.5 != 0 && x.6 != 0 && x.7 != 0
    }
    #[inline]
    pub fn bool32ix8_any(x: bool32ix8) -> bool {
        x.0 != 0 || x.1 != 0 || x.2 != 0 || x.3 != 0 ||
        x.4 != 0 || x.5 != 0 || x.6 != 0 || x.7 != 0
    }
    #[inline]
    pub fn bool32fx8_all(x: bool32fx8) -> bool {
        x.0 != 0 && x.1 != 0 && x.2 != 0 && x.3 != 0 &&
        x.4 != 0 && x.5 != 0 && x.6 != 0 && x.7 != 0
    }
    #[inline]
    pub fn bool32fx8_any(x: bool32fx8) -> bool {
        x.0 != 0 || x.1 != 0 || x.2 != 0 || x.3 != 0 ||
        x.4 != 0 || x.5 != 0 || x.6 != 0 || x.7 != 0
    }
}

bool_impls! {
    bool64ix4: bool64i, i64x4, i64, 4, bool64ix4_all, bool64ix4_any, x0, x1 | x2, x3
        [/// Convert `self` to a boolean vector for interacting with floating point vectors.
         to_f -> bool64fx4];

    bool64fx4: bool64f, i64x4, i64, 4, bool64fx4_all, bool64fx4_any, x0, x1 | x2, x3
        [/// Convert `self` to a boolean vector for interacting with integer vectors.
         to_i -> bool64ix4];
    bool32ix8: bool32i, i32x8, i32, 8, bool32ix8_all, bool32ix8_any, x0, x1, x2, x3 | x4, x5, x6, x7
        [/// Convert `self` to a boolean vector for interacting with floating point vectors.
         to_f -> bool32fx8];

    bool32fx8: bool32f, i32x8, i32, 8, bool32fx8_all, bool32fx8_any, x0, x1, x2, x3 | x4, x5, x6, x7
        [/// Convert `self` to a boolean vector for interacting with integer vectors.
         to_i -> bool32ix8];
}

impl u32x8 {
    /// Convert each lane to a signed integer.
    #[inline]
    pub fn to_i32(self) -> i32x8 {
        unsafe {simd_cast(self)}
    }
    /// Convert each lane to a 32-bit float.
    #[inline]
    pub fn to_f32(self) -> f32x8 {
        unsafe {simd_cast(self)}
    }
}
impl i32x8 {
    /// Convert each lane to an unsigned integer.
    #[inline]
    pub fn to_u32(self) -> u32x8 {
        unsafe {simd_cast(self)}
    }
    /// Convert each lane to a 32-bit float.
    #[inline]
    pub fn to_f32(self) -> f32x8 {
        unsafe {simd_cast(self)}
    }
}

operators! {
    Add (simd_add, add):
        i8x32, u8x32, i16x16, u16x16, i32x8, u32x8, i64x4, u64x4,
        f64x4, f32x8;
    Sub (simd_sub, sub):
        i8x32, u8x32, i16x16, u16x16, i32x8, u32x8, i64x4, u64x4,
        f64x4, f32x8;
    Mul (simd_mul, mul):
        i8x32, u8x32, i16x16, u16x16, i32x8, u32x8, i64x4, u64x4,
        f64x4, f32x8;
    Div (simd_div, div): f64x4, f32x8;

    BitAnd (simd_and, bitand):
        i64x4, u64x4, i32x8, u32x8,
        bool64ix4, bool32ix8, bool16ix16,
        bool64fx4, bool32fx8;
    BitOr (simd_or, bitor):
        i64x4, u64x4, i32x8, u32x8,
        bool64ix4, bool32ix8, bool16ix16,
        bool64fx4, bool32fx8;
    BitXor (simd_xor, bitxor):
        i64x4, u64x4, i32x8, u32x8,
        bool64ix4, bool32ix8, bool16ix16,
        bool64fx4, bool32fx8;
}

