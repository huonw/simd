use super::*;
use std::mem;

#[repr(simd)]
struct u32x2(u32, u32);
#[repr(simd)]
struct u16x4(u16, u16, u16, u16);
#[repr(simd)]
struct u8x8(u8, u8, u8, u8,
            u8, u8, u8, u8);

extern "platform-intrinsic" {
    fn arm_vsqrtq_f32(x: f32x4) -> f32x4;

    fn arm_vrsqrteq_f32(x: f32x4) -> f32x4;
    fn arm_vrecpeq_f32(x: f32x4) -> f32x4;


    fn arm_vminq_f32(x: f32x4, y: f32x4) -> f32x4;
    fn arm_vmaxq_f32(x: f32x4, y: f32x4) -> f32x4;

    fn arm_vpmin_u32(x: u32x2, y: u32x2) -> u32x2;
    fn arm_vpmax_u32(x: u32x2, y: u32x2) -> u32x2;
    fn arm_vpmin_u16(x: u16x4, y: u16x4) -> u16x4;
    fn arm_vpmax_u16(x: u16x4, y: u16x4) -> u16x4;
    fn arm_vpmin_u8(x: u8x8, y: u8x8) -> u8x8;
    fn arm_vpmax_u8(x: u8x8, y: u8x8) -> u8x8;
}


impl f32x4 {
    #[inline]
    pub fn sqrt(self) -> Self {
        unsafe {arm_vsqrtq_f32(self)}
    }
    #[inline]
    pub fn approx_rsqrt(self) -> Self {
        unsafe {arm_vrsqrteq_f32(self)}
    }
    #[inline]
    pub fn approx_reciprocal(self) -> Self {
        unsafe {arm_vrecpeq_f32(self)}
    }
    #[inline]
    pub fn max(self, other: Self) -> Self {
        unsafe {arm_vmaxq_f32(self, other)}
    }
    #[inline]
    pub fn min(self, other: Self) -> Self {
        unsafe {arm_vminq_f32(self, other)}
    }
}

macro_rules! bool_impls {
    ($($outer: ident, $half: ident, $min: ident, $max: ident;)*) => {
        $(impl $outer {
            pub fn any(self) -> bool {
                unsafe {
                    let (lo, hi): ($half, $half) = mem::transmute(self);
                    let x = $max(lo, hi);
                    let y = $max(x, mem::uninitialized());
                    y.0 != 0
                }
            }
            pub fn all(self) -> bool {
                unsafe {
                    let (lo, hi): ($half, $half) = mem::transmute(self);
                    let x = $min(lo, hi);
                    let y = $min(x, mem::uninitialized());
                    y.0 != 0
                }
            }
        })*
    }
}
bool_impls! {
    bool32fx4, u32x2, arm_vpmin_u32, arm_vpmax_u32;
    bool8ix16, u8x8, arm_vpmin_u8, arm_vpmax_u8;
    bool16ix8, u16x4, arm_vpmin_u16, arm_vpmax_u16;
    bool32ix4, u32x2, arm_vpmin_u32, arm_vpmax_u32;
}
