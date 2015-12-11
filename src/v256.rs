use std::ops;
#[allow(unused_imports)]
use super::{
	Simd,
    f32x2,
    simd_eq, simd_ne, simd_lt, simd_le, simd_gt, simd_ge,
    simd_shuffle2, simd_shuffle4, simd_shuffle8, simd_shuffle16,
    simd_insert, simd_extract,
    simd_cast,
    simd_add, simd_sub, simd_mul, simd_div, simd_shl, simd_shr, simd_and, simd_or, simd_xor,
    bool32f,
    Unalign, bitcast,
};
use super::sixty_four::*;
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct u64x4(u64, u64, u64, u64);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct i64x4(i64, i64, i64, i64);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct f64x4(f64, f64, f64, f64);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct bool64ix4(i64, i64, i64, i64);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct bool64fx4(i64, i64, i64, i64);

#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct u32x8(u32, u32, u32, u32,
                 u32, u32, u32, u32);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct i32x8(i32, i32, i32, i32,
                 i32, i32, i32, i32);
#[repr(simd)]
#[cfg_attr(feature = "serde", derfve(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct f32x8(f32, f32, f32, f32,
                 f32, f32, f32, f32);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct bool32ix8(i32, i32, i32, i32,
                     i32, i32, i32, i32);#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct bool32fx8(i32, i32, i32, i32,
                     i32, i32, i32, i32);

#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct u16x16(u16, u16, u16, u16, u16, u16, u16, u16,
                  u16, u16, u16, u16, u16, u16, u16, u16);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct i16x16(i16, i16, i16, i16, i16, i16, i16, i16,
                  i16, i16, i16, i16, i16, i16, i16, i16);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct bool16ix16(i16, i16, i16, i16, i16, i16, i16, i16,
                      i16, i16, i16, i16, i16, i16, i16, i16);

#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct u8x32(u8, u8, u8, u8, u8, u8, u8, u8,
                 u8, u8, u8, u8, u8, u8, u8, u8,
                 u8, u8, u8, u8, u8, u8, u8, u8,
                 u8, u8, u8, u8, u8, u8, u8, u8);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct i8x32(i8, i8, i8, i8, i8, i8, i8, i8,
                 i8, i8, i8, i8, i8, i8, i8, i8,
                 i8, i8, i8, i8, i8, i8, i8, i8,
                 i8, i8, i8, i8, i8, i8, i8, i8);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct bool8ix32(i8, i8, i8, i8, i8, i8, i8, i8,
                     i8, i8, i8, i8, i8, i8, i8, i8,
                     i8, i8, i8, i8, i8, i8, i8, i8,
                     i8, i8, i8, i8, i8, i8, i8, i8);
					 
operators! {
    Add (simd_add, add):
        f64x4, f32x8;
    Sub (simd_sub, sub):
        f64x4, f32x8;
    Mul (simd_mul, mul):
        f64x4, f32x8;
    Div (simd_div, div): f64x4, f32x8;
}

simd! {
    bool64fx4: f64x4 = f64, bool64fx4 = bool64f;
    bool32fx8: f32x8 = f32, bool32fx8 = bool32f;
}
basic_impls! {
    f64x4: f64, bool64fx4, simd_shuffle4, 4, x0, x1 | x2, x3;
    f32x8: f32, bool32fx8, simd_shuffle8, 8, x0, x1, x2, x3 | x4, x5, x6, x7;
}