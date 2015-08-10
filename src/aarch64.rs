use super::*;
use super::bitcast;

extern "platform-intrinsic" {
    fn aarch64_vsqrtq_f32(x: f32x4) -> f32x4;

    fn aarch64_vrsqrteq_f32(x: f32x4) -> f32x4;


    fn aarch64_vminq_f32(x: f32x4, y: f32x4) -> f32x4;
    fn aarch64_vmaxq_f32(x: f32x4, y: f32x4) -> f32x4;

    fn aarch64_vmaxvq_u8(x: u8x16) -> u8;
    fn aarch64_vmaxvq_u16(x: u16x8) -> u16;
    fn aarch64_vmaxvq_u32(x: u32x4) -> u32;
    fn aarch64_vminvq_u8(x: u8x16) -> u8;
    fn aarch64_vminvq_u16(x: u16x8) -> u16;
    fn aarch64_vminvq_u32(x: u32x4) -> u32;
}

impl f32x4 {
    pub fn sqrt(self) -> Self {
        unsafe {aarch64_vsqrtq_f32(self)}
    }
    pub fn approx_rsqrt(self) -> Self {
        unsafe {aarch64_vrsqrteq_f32(self)}
    }
    pub fn max(self, other: Self) -> Self {
        unsafe {aarch64_vmaxq_f32(self, other)}
    }
    pub fn min(self, other: Self) -> Self {
        unsafe {aarch64_vminq_f32(self, other)}
    }
}

macro_rules! bool_impls {
    ($($t: ty, $min: ident, $max: ident;)*) => {
        $(impl $t {
            pub fn any(self) -> bool {
                unsafe {
                    $max(bitcast(self)) != 0
                }
            }
            pub fn all(self) -> bool {
                unsafe {
                    $min(bitcast(self)) != 0
                }
            }
        })*
    }
}
bool_impls! {
    bool32fx4, aarch64_vminvq_u32, aarch64_vmaxvq_u32;
    bool8ix16, aarch64_vminvq_u8, aarch64_vmaxvq_u8;
    bool16ix8, aarch64_vminvq_u16, aarch64_vmaxvq_u16;
    bool32ix4, aarch64_vminvq_u32, aarch64_vmaxvq_u32;
}
