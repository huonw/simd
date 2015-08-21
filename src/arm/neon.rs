use super::super::*;
use sixty_four::{i64x2, u64x2};

#[repr(simd)]
#[derive(Copy, Clone)]
pub struct u32x2(u32, u32);
#[repr(simd)]
#[derive(Copy, Clone)]
pub struct i32x2(i32, i32);
#[repr(simd)]
#[derive(Copy, Clone)]
pub struct f32x2(f32, f32);

#[repr(simd)]
#[derive(Copy, Clone)]
pub struct u16x4(u16, u16, u16, u16);
#[repr(simd)]
#[derive(Copy, Clone)]
pub struct i16x4(i16, i16, i16, i16);
#[repr(simd)]
#[derive(Copy, Clone)]
pub struct u8x8(u8, u8, u8, u8,
            u8, u8, u8, u8);
#[repr(simd)]
#[derive(Copy, Clone)]
pub struct i8x8(i8, i8, i8, i8,
                i8, i8, i8, i8);

#[repr(simd)]
#[derive(Copy, Clone)]
pub struct i64x1(i64);
#[repr(simd)]
#[derive(Copy, Clone)]
pub struct u64x1(u64);

#[allow(dead_code)]
extern "platform-intrinsic" {
    fn arm_vsqrtq_f32(x: f32x4) -> f32x4;

    fn arm_vhadd_s8(x: i8x8, y: i8x8) -> i8x8;
    fn arm_vhaddq_s8(x: i8x16, y: i8x16) -> i8x16;
    fn arm_vhadd_s16(x: i16x4, y: i16x4) -> i16x4;
    fn arm_vhaddq_s16(x: i16x8, y: i16x8) -> i16x8;
    fn arm_vhadd_s32(x: i32x2, y: i32x2) -> i32x2;
    fn arm_vhaddq_s32(x: i32x4, y: i32x4) -> i32x4;
    fn arm_vhadd_u8(x: u8x8, y: u8x8) -> u8x8;
    fn arm_vhaddq_u8(x: u8x16, y: u8x16) -> u8x16;
    fn arm_vhadd_u16(x: u16x4, y: u16x4) -> u16x4;
    fn arm_vhaddq_u16(x: u16x8, y: u16x8) -> u16x8;
    fn arm_vhadd_u32(x: u32x2, y: u32x2) -> u32x2;
    fn arm_vhaddq_u32(x: u32x4, y: u32x4) -> u32x4;
    fn arm_vrhadd_s8(x: i8x8, y: i8x8) -> i8x8;
    fn arm_vrhaddq_s8(x: i8x16, y: i8x16) -> i8x16;
    fn arm_vrhadd_s16(x: i16x4, y: i16x4) -> i16x4;
    fn arm_vrhaddq_s16(x: i16x8, y: i16x8) -> i16x8;
    fn arm_vrhadd_s32(x: i32x2, y: i32x2) -> i32x2;
    fn arm_vrhaddq_s32(x: i32x4, y: i32x4) -> i32x4;
    fn arm_vrhadd_u8(x: u8x8, y: u8x8) -> u8x8;
    fn arm_vrhaddq_u8(x: u8x16, y: u8x16) -> u8x16;
    fn arm_vrhadd_u16(x: u16x4, y: u16x4) -> u16x4;
    fn arm_vrhaddq_u16(x: u16x8, y: u16x8) -> u16x8;
    fn arm_vrhadd_u32(x: u32x2, y: u32x2) -> u32x2;
    fn arm_vrhaddq_u32(x: u32x4, y: u32x4) -> u32x4;
    fn arm_vqadd_s8(x: i8x8, y: i8x8) -> i8x8;
    fn arm_vqaddq_s8(x: i8x16, y: i8x16) -> i8x16;
    fn arm_vqadd_s16(x: i16x4, y: i16x4) -> i16x4;
    fn arm_vqaddq_s16(x: i16x8, y: i16x8) -> i16x8;
    fn arm_vqadd_s32(x: i32x2, y: i32x2) -> i32x2;
    fn arm_vqaddq_s32(x: i32x4, y: i32x4) -> i32x4;
    fn arm_vqadd_s64(x: i64x1, y: i64x1) -> i64x1;
    fn arm_vqaddq_s64(x: i64x2, y: i64x2) -> i64x2;
    fn arm_vqadd_u8(x: u8x8, y: u8x8) -> u8x8;
    fn arm_vqaddq_u8(x: u8x16, y: u8x16) -> u8x16;
    fn arm_vqadd_u16(x: u16x4, y: u16x4) -> u16x4;
    fn arm_vqaddq_u16(x: u16x8, y: u16x8) -> u16x8;
    fn arm_vqadd_u32(x: u32x2, y: u32x2) -> u32x2;
    fn arm_vqaddq_u32(x: u32x4, y: u32x4) -> u32x4;
    fn arm_vqadd_u64(x: u64x1, y: u64x1) -> u64x1;
    fn arm_vqaddq_u64(x: u64x2, y: u64x2) -> u64x2;
    fn arm_vraddhn_s16(x: i16x8, y: i16x8) -> i8x8;
    fn arm_vraddhn_s32(x: i32x4, y: i32x4) -> i16x4;
    fn arm_vraddhn_s64(x: i64x2, y: i64x2) -> i32x2;
    fn arm_vfma_f32(x: f32x2, y: f32x2, z: f32x2) -> f32x2;
    fn arm_vfmaq_f32(x: f32x4, y: f32x4, z: f32x4) -> f32x4;
    fn arm_vqdmulh_s16(x: i16x4, y: i16x4) -> i16x4;
    fn arm_vqdmulhq_s16(x: i16x8, y: i16x8) -> i16x8;
    fn arm_vqdmulh_s32(x: i32x2, y: i32x2) -> i32x4;
    fn arm_vqdmulhq_s32(x: i32x4, y: i32x4) -> i32x4;
    fn arm_vqrdmulh_s16(x: i16x4, y: i16x4) -> i16x4;
    fn arm_vqrdmulhqr_s16(x: i16x8, y: i16x8) -> i16x8;
    fn arm_vqrdmulh_s32(x: i32x2, y: i32x2) -> i32x4;
    fn arm_vqrdmulhqr_s32(x: i32x4, y: i32x4) -> i32x4;
    fn arm_vmull_s8(x: i8x8, y: i8x8) -> i16x8;
    fn arm_vmull_s16(x: i16x4, y: i16x4) -> i32x4;
    fn arm_vmull_s32(x: i32x2, y: i32x2) -> i64x2;
    fn arm_vmull_u8(x: u8x8, y: u8x8) -> u16x8;
    fn arm_vmull_u16(x: u16x4, y: u16x4) -> u32x4;
    fn arm_vmull_u32(x: u32x2, y: u32x2) -> u64x2;
    fn arm_vqdmull_s16(x: i16x4, y: i16x4) -> i32x4;
    fn arm_vqdmull_s32(x: i32x2, y: i32x2) -> i64x2;
    fn arm_vhsub_s8(x: i8x8, y: i8x8) -> i8x8;
    fn arm_vhsubq_s8(x: i8x16, y: i8x16) -> i8x16;
    fn arm_vhsub_s16(x: i16x4, y: i16x4) -> i16x4;
    fn arm_vhsubq_s16(x: i16x8, y: i16x8) -> i16x8;
    fn arm_vhsub_s32(x: i32x2, y: i32x2) -> i32x2;
    fn arm_vhsubq_s32(x: i32x4, y: i32x4) -> i32x4;
    fn arm_vhsub_u8(x: u8x8, y: u8x8) -> u8x8;
    fn arm_vhsubq_u8(x: u8x16, y: u8x16) -> u8x16;
    fn arm_vhsub_u16(x: u16x4, y: u16x4) -> u16x4;
    fn arm_vhsubq_u16(x: u16x8, y: u16x8) -> u16x8;
    fn arm_vhsub_u32(x: u32x2, y: u32x2) -> u32x2;
    fn arm_vhsubq_u32(x: u32x4, y: u32x4) -> u32x4;
    fn arm_vqsub_s8(x: i8x8, y: i8x8) -> i8x8;
    fn arm_vqsubq_s8(x: i8x16, y: i8x16) -> i8x16;
    fn arm_vqsub_s16(x: i16x4, y: i16x4) -> i16x4;
    fn arm_vqsubq_s16(x: i16x8, y: i16x8) -> i16x8;
    fn arm_vqsub_s32(x: i32x2, y: i32x2) -> i32x2;
    fn arm_vqsubq_s32(x: i32x4, y: i32x4) -> i32x4;
    fn arm_vqsub_s64(x: i64x1, y: i64x1) -> i64x1;
    fn arm_vqsubq_s64(x: i64x2, y: i64x2) -> i64x2;
    fn arm_vqsub_u8(x: u8x8, y: u8x8) -> u8x8;
    fn arm_vqsubq_u8(x: u8x16, y: u8x16) -> u8x16;
    fn arm_vqsub_u16(x: u16x4, y: u16x4) -> u16x4;
    fn arm_vqsubq_u16(x: u16x8, y: u16x8) -> u16x8;
    fn arm_vqsub_u32(x: u32x2, y: u32x2) -> u32x2;
    fn arm_vqsubq_u32(x: u32x4, y: u32x4) -> u32x4;
    fn arm_vqsub_u64(x: u64x1, y: u64x1) -> u64x1;
    fn arm_vqsubq_u64(x: u64x2, y: u64x2) -> u64x2;
    fn arm_vabd_s8(x: i8x8, y: i8x8) -> i8x8;
    fn arm_vabdq_s8(x: i8x16, y: i8x16) -> i8x16;
    fn arm_vabd_s16(x: i16x4, y: i16x4) -> i16x4;
    fn arm_vabdq_s16(x: i16x8, y: i16x8) -> i16x8;
    fn arm_vabd_s32(x: i32x2, y: i32x2) -> i32x2;
    fn arm_vabdq_s32(x: i32x4, y: i32x4) -> i32x4;
    fn arm_vabd_u8(x: u8x8, y: u8x8) -> u8x8;
    fn arm_vabdq_u8(x: u8x16, y: u8x16) -> u8x16;
    fn arm_vabd_u16(x: u16x4, y: u16x4) -> u16x4;
    fn arm_vabdq_u16(x: u16x8, y: u16x8) -> u16x8;
    fn arm_vabd_u32(x: u32x2, y: u32x2) -> u32x2;
    fn arm_vabdq_u32(x: u32x4, y: u32x4) -> u32x4;
    fn arm_vabd_f32(x: f32x2, y: f32x2) -> f32x2;
    fn arm_vabdq_f32(x: f32x4, y: f32x4) -> f32x4;
    fn arm_vmax_s8(x: i8x8, y: i8x8) -> i8x8;
    fn arm_vmaxq_s8(x: i8x16, y: i8x16) -> i8x16;
    fn arm_vmax_s16(x: i16x4, y: i16x4) -> i16x4;
    fn arm_vmaxq_s16(x: i16x8, y: i16x8) -> i16x8;
    fn arm_vmax_s32(x: i32x2, y: i32x2) -> i32x2;
    fn arm_vmaxq_s32(x: i32x4, y: i32x4) -> i32x4;
    fn arm_vmax_u8(x: u8x8, y: u8x8) -> u8x8;
    fn arm_vmaxq_u8(x: u8x16, y: u8x16) -> u8x16;
    fn arm_vmax_u16(x: u16x4, y: u16x4) -> u16x4;
    fn arm_vmaxq_u16(x: u16x8, y: u16x8) -> u16x8;
    fn arm_vmax_u32(x: u32x2, y: u32x2) -> u32x2;
    fn arm_vmaxq_u32(x: u32x4, y: u32x4) -> u32x4;
    fn arm_vmax_f32(x: f32x2, y: f32x2) -> f32x2;
    fn arm_vmaxq_f32(x: f32x4, y: f32x4) -> f32x4;
    fn arm_vmin_s8(x: i8x8, y: i8x8) -> i8x8;
    fn arm_vminq_s8(x: i8x16, y: i8x16) -> i8x16;
    fn arm_vmin_s16(x: i16x4, y: i16x4) -> i16x4;
    fn arm_vminq_s16(x: i16x8, y: i16x8) -> i16x8;
    fn arm_vmin_s32(x: i32x2, y: i32x2) -> i32x2;
    fn arm_vminq_s32(x: i32x4, y: i32x4) -> i32x4;
    fn arm_vmin_u8(x: u8x8, y: u8x8) -> u8x8;
    fn arm_vminq_u8(x: u8x16, y: u8x16) -> u8x16;
    fn arm_vmin_u16(x: u16x4, y: u16x4) -> u16x4;
    fn arm_vminq_u16(x: u16x8, y: u16x8) -> u16x8;
    fn arm_vmin_u32(x: u32x2, y: u32x2) -> u32x2;
    fn arm_vminq_u32(x: u32x4, y: u32x4) -> u32x4;
    fn arm_vmin_f32(x: f32x2, y: f32x2) -> f32x2;
    fn arm_vminq_f32(x: f32x4, y: f32x4) -> f32x4;
    fn arm_vshl_s8(x: i8x8, y: i8x8) -> i8x8;
    fn arm_vshlq_s8(x: i8x16, y: i8x16) -> i8x16;
    fn arm_vshl_s16(x: i16x4, y: i16x4) -> i16x4;
    fn arm_vshlq_s16(x: i16x8, y: i16x8) -> i16x8;
    fn arm_vshl_s32(x: i32x2, y: i32x2) -> i32x2;
    fn arm_vshlq_s32(x: i32x4, y: i32x4) -> i32x4;
    fn arm_vshl_s64(x: i64x1, y: i64x1) -> i64x1;
    fn arm_vshlq_s64(x: i64x2, y: i64x2) -> i64x2;
    fn arm_vshl_u8(x: u8x8, y: u8x8) -> u8x8;
    fn arm_vshlq_u8(x: u8x16, y: u8x16) -> u8x16;
    fn arm_vshl_u16(x: u16x4, y: u16x4) -> u16x4;
    fn arm_vshlq_u16(x: u16x8, y: u16x8) -> u16x8;
    fn arm_vshl_u32(x: u32x2, y: u32x2) -> u32x2;
    fn arm_vshlq_u32(x: u32x4, y: u32x4) -> u32x4;
    fn arm_vshl_u64(x: u64x1, y: u64x1) -> u64x1;
    fn arm_vshlq_u64(x: u64x2, y: u64x2) -> u64x2;
    fn arm_vqshl_s8(x: i8x8, y: i8x8) -> i8x8;
    fn arm_vqshlq_s8(x: i8x16, y: i8x16) -> i8x16;
    fn arm_vqshl_s16(x: i16x4, y: i16x4) -> i16x4;
    fn arm_vqshlq_s16(x: i16x8, y: i16x8) -> i16x8;
    fn arm_vqshl_s32(x: i32x2, y: i32x2) -> i32x2;
    fn arm_vqshlq_s32(x: i32x4, y: i32x4) -> i32x4;
    fn arm_vqshl_s64(x: i64x1, y: i64x1) -> i64x1;
    fn arm_vqshlq_s64(x: i64x2, y: i64x2) -> i64x2;
    fn arm_vqshl_u8(x: u8x8, y: u8x8) -> u8x8;
    fn arm_vqshlq_u8(x: u8x16, y: u8x16) -> u8x16;
    fn arm_vqshl_u16(x: u16x4, y: u16x4) -> u16x4;
    fn arm_vqshlq_u16(x: u16x8, y: u16x8) -> u16x8;
    fn arm_vqshl_u32(x: u32x2, y: u32x2) -> u32x2;
    fn arm_vqshlq_u32(x: u32x4, y: u32x4) -> u32x4;
    fn arm_vqshl_u64(x: u64x1, y: u64x1) -> u64x1;
    fn arm_vqshlq_u64(x: u64x2, y: u64x2) -> u64x2;
    fn arm_vrshl_s8(x: i8x8, y: i8x8) -> i8x8;
    fn arm_vrshlr_s8(x: i8x16, y: i8x16) -> i8x16;
    fn arm_vrshl_s16(x: i16x4, y: i16x4) -> i16x4;
    fn arm_vrshlr_s16(x: i16x8, y: i16x8) -> i16x8;
    fn arm_vrshl_s32(x: i32x2, y: i32x2) -> i32x2;
    fn arm_vrshlr_s32(x: i32x4, y: i32x4) -> i32x4;
    fn arm_vrshl_s64(x: i64x1, y: i64x1) -> i64x1;
    fn arm_vrshlr_s64(x: i64x2, y: i64x2) -> i64x2;
    fn arm_vrshl_u8(x: u8x8, y: u8x8) -> u8x8;
    fn arm_vrshlr_u8(x: u8x16, y: u8x16) -> u8x16;
    fn arm_vrshl_u16(x: u16x4, y: u16x4) -> u16x4;
    fn arm_vrshlr_u16(x: u16x8, y: u16x8) -> u16x8;
    fn arm_vrshl_u32(x: u32x2, y: u32x2) -> u32x2;
    fn arm_vrshlr_u32(x: u32x4, y: u32x4) -> u32x4;
    fn arm_vrshl_u64(x: u64x1, y: u64x1) -> u64x1;
    fn arm_vrshlr_u64(x: u64x2, y: u64x2) -> u64x2;
    fn arm_vqrshl_s8(x: i8x8, y: i8x8) -> i8x8;
    fn arm_vqrshlqr_s8(x: i8x16, y: i8x16) -> i8x16;
    fn arm_vqrshl_s16(x: i16x4, y: i16x4) -> i16x4;
    fn arm_vqrshlqr_s16(x: i16x8, y: i16x8) -> i16x8;
    fn arm_vqrshl_s32(x: i32x2, y: i32x2) -> i32x2;
    fn arm_vqrshlqr_s32(x: i32x4, y: i32x4) -> i32x4;
    fn arm_vqrshl_s64(x: i64x1, y: i64x1) -> i64x1;
    fn arm_vqrshlqr_s64(x: i64x2, y: i64x2) -> i64x2;
    fn arm_vqrshl_u8(x: u8x8, y: u8x8) -> u8x8;
    fn arm_vqrshlqr_u8(x: u8x16, y: u8x16) -> u8x16;
    fn arm_vqrshl_u16(x: u16x4, y: u16x4) -> u16x4;
    fn arm_vqrshlqr_u16(x: u16x8, y: u16x8) -> u16x8;
    fn arm_vqrshl_u32(x: u32x2, y: u32x2) -> u32x2;
    fn arm_vqrshlqr_u32(x: u32x4, y: u32x4) -> u32x4;
    fn arm_vqrshl_u64(x: u64x1, y: u64x1) -> u64x1;
    fn arm_vqrshlqr_u64(x: u64x2, y: u64x2) -> u64x2;
    fn arm_vqmovn_s16(x: i16x8) -> i8x8;
    fn arm_vqmovn_s32(x: i32x4) -> i16x4;
    fn arm_vqmovn_s64(x: i64x2) -> i32x2;
    fn arm_vqmovn_u16(x: u16x8) -> u8x8;
    fn arm_vqmovn_u32(x: u32x4) -> u16x4;
    fn arm_vqmovn_u64(x: u64x2) -> u32x2;
    fn arm_vqmovun_s16(x: i16x8) -> i8x8;
    fn arm_vqmovun_s32(x: i32x4) -> i16x4;
    fn arm_vqmovun_s64(x: i64x2) -> i32x2;
    fn arm_vabs_s8(x: i8x8, y: i8x8) -> i8x8;
    fn arm_vabsq_s8(x: i8x16, y: i8x16) -> i8x16;
    fn arm_vabs_s16(x: i16x4, y: i16x4) -> i16x4;
    fn arm_vabsq_s16(x: i16x8, y: i16x8) -> i16x8;
    fn arm_vabs_s32(x: i32x2, y: i32x2) -> i32x2;
    fn arm_vabsq_s32(x: i32x4, y: i32x4) -> i32x4;
    fn arm_vabs_f32(x: f32x2, y: f32x2) -> f32x2;
    fn arm_vabsq_f32(x: f32x4, y: f32x4) -> f32x4;
    fn arm_vqabs_s8(x: i8x8, y: i8x8) -> i8x8;
    fn arm_vqabsq_s8(x: i8x16, y: i8x16) -> i8x16;
    fn arm_vqabs_s16(x: i16x4, y: i16x4) -> i16x4;
    fn arm_vqabsq_s16(x: i16x8, y: i16x8) -> i16x8;
    fn arm_vqabs_s32(x: i32x2, y: i32x2) -> i32x2;
    fn arm_vqabsq_s32(x: i32x4, y: i32x4) -> i32x4;
    fn arm_vneg_s8(x: i8x8) -> i8x8;
    fn arm_vnegq_s8(x: i8x16) -> i8x16;
    fn arm_vneg_s16(x: i16x4) -> i16x4;
    fn arm_vnegq_s16(x: i16x8) -> i16x8;
    fn arm_vneg_s32(x: i32x2) -> i32x2;
    fn arm_vnegq_s32(x: i32x4) -> i32x4;
    fn arm_vneg_f32(x: f32x2) -> f32x2;
    fn arm_vnegq_f32(x: f32x4) -> f32x4;
    fn arm_vqneg_s8(x: i8x8) -> i8x8;
    fn arm_vqnegq_s8(x: i8x16) -> i8x16;
    fn arm_vqneg_s16(x: i16x4) -> i16x4;
    fn arm_vqnegq_s16(x: i16x8) -> i16x8;
    fn arm_vqneg_s32(x: i32x2) -> i32x2;
    fn arm_vqnegq_s32(x: i32x4) -> i32x4;
    fn arm_vcls_s8(x: i8x8) -> i8x8;
    fn arm_vclsq_s8(x: i8x16) -> i8x16;
    fn arm_vcls_s16(x: i16x4) -> i16x4;
    fn arm_vclsq_s16(x: i16x8) -> i16x8;
    fn arm_vcls_s32(x: i32x2) -> i32x2;
    fn arm_vclsq_s32(x: i32x4) -> i32x4;
    fn arm_vclz_s8(x: i8x8) -> i8x8;
    fn arm_vclzq_s8(x: i8x16) -> i8x16;
    fn arm_vclz_s16(x: i16x4) -> i16x4;
    fn arm_vclzq_s16(x: i16x8) -> i16x8;
    fn arm_vclz_s32(x: i32x2) -> i32x2;
    fn arm_vclzq_s32(x: i32x4) -> i32x4;
    fn arm_vcnt_s8(x: i8x8) -> i8x8;
    fn arm_vcntq_s8(x: i8x16) -> i8x16;
    fn arm_vrecpe_u32(x: u32x2) -> u32x2;
    fn arm_vrecpeq_u32(x: u32x4) -> u32x4;
    fn arm_vrecpe_f32(x: f32x2) -> f32x2;
    fn arm_vrecpeq_f32(x: f32x4) -> f32x4;
    fn arm_vrecps_f32(x: f32x2, y: f32x2) -> f32x2;
    fn arm_vrecpsq_f32(x: f32x4, y: f32x4) -> f32x4;
    fn arm_vrsqrte_u32(x: u32x2) -> u32x2;
    fn arm_vrsqrteq_u32(x: u32x4) -> u32x4;
    fn arm_vrsqrte_f32(x: f32x2) -> f32x2;
    fn arm_vrsqrteq_f32(x: f32x4) -> f32x4;
    fn arm_vrsqrts_f32(x: f32x2, y: f32x2) -> f32x2;
    fn arm_vrsqrtsq_f32(x: f32x4, y: f32x4) -> f32x4;
    fn arm_vbsl_s8(x: i8x8, y: i8x8, z: i8x8) -> i8x8;
    fn arm_vbslq_s8(x: i8x16, y: i8x16, z: i8x16) -> i8x16;
    fn arm_vpadd_s8(x: i8x8, y: i8x8) -> i8x8;
    fn arm_vpadd_s16(x: i16x4, y: i16x4) -> i16x4;
    fn arm_vpadd_s32(x: i32x2, y: i32x2) -> i32x2;
    fn arm_vpadd_u8(x: u8x8, y: u8x8) -> u8x8;
    fn arm_vpadd_u16(x: u16x4, y: u16x4) -> u16x4;
    fn arm_vpadd_u32(x: u32x2, y: u32x2) -> u32x2;
    fn arm_vpadd_f32(x: f32x2, y: f32x2) -> f32x2;
    fn arm_vpaddl_s8(x: i8x8) -> i16x4;
    fn arm_vpaddlq_s8(x: i8x16) -> i16x8;
    fn arm_vpaddl_s16(x: i16x4) -> i32x2;
    fn arm_vpaddlq_s16(x: i16x8) -> i32x4;
    fn arm_vpaddl_s32(x: i32x2) -> i64x1;
    fn arm_vpaddlq_s32(x: i32x4) -> i64x2;
    fn arm_vpaddl_u8(x: u8x8) -> u16x4;
    fn arm_vpaddlq_u8(x: u8x16) -> u16x8;
    fn arm_vpaddl_u16(x: u16x4) -> u32x2;
    fn arm_vpaddlq_u16(x: u16x8) -> u32x4;
    fn arm_vpaddl_u32(x: u32x2) -> u64x1;
    fn arm_vpaddlq_u32(x: u32x4) -> u64x2;
    fn arm_vpadal_s8(x: i16x4, y: i8x8) -> i16x4;
    fn arm_vpadalq_s8(x: i16x8, y: i8x16) -> i16x8;
    fn arm_vpadal_s16(x: i32x2, y: i16x4) -> i32x2;
    fn arm_vpadalq_s16(x: i32x4, y: i16x8) -> i32x4;
    fn arm_vpadal_s32(x: i64x1, y: i32x2) -> i64x1;
    fn arm_vpadalq_s32(x: i64x2, y: i32x4) -> i64x2;
    fn arm_vpadal_u8(x: u16x4, y: u8x8) -> u16x4;
    fn arm_vpadalq_u8(x: u16x8, y: u8x16) -> u16x8;
    fn arm_vpadal_u16(x: u32x2, y: u16x4) -> u32x2;
    fn arm_vpadalq_u16(x: u32x4, y: u16x8) -> u32x4;
    fn arm_vpadal_u32(x: u64x1, y: u32x2) -> u64x1;
    fn arm_vpadalq_u32(x: u64x2, y: u32x4) -> u64x2;
    fn arm_vpmax_s16(x: i16x4, y: i16x4) -> i16x4;
    fn arm_vpmax_s32(x: i32x2, y: i32x2) -> i32x2;
    fn arm_vpmax_s8(x: i8x8, y: i8x8) -> i8x8;
    fn arm_vpmax_u16(x: u16x4, y: u16x4) -> u16x4;
    fn arm_vpmax_u32(x: u32x2, y: u32x2) -> u32x2;
    fn arm_vpmax_u8(x: u8x8, y: u8x8) -> u8x8;
    fn arm_vpmin_s16(x: i16x4, y: i16x4) -> i16x4;
    fn arm_vpmin_s32(x: i32x2, y: i32x2) -> i32x2;
    fn arm_vpmin_s8(x: i8x8, y: i8x8) -> i8x8;
    fn arm_vpmin_u16(x: u16x4, y: u16x4) -> u16x4;
    fn arm_vpmin_u32(x: u32x2, y: u32x2) -> u32x2;
    fn arm_vpmin_u8(x: u8x8, y: u8x8) -> u8x8;
    fn arm_vtbl1_s8(t0: i8x8, idx: i8x8) -> i8x8;
    fn arm_vtbl1_u8(t0: u8x8, idx: u8x8) -> u8x8;
    // these aren't exactly the C intrinsics (x: they take one argument)
    fn arm_vtbl2_s8(t0: i8x8, t1: i8x8, idx: i8x8) -> i8x8;
    fn arm_vtbl2_u8(t0: u8x8, t1: u8x8, idx: u8x8) -> u8x8;
    fn arm_vtbl3_s8(t0: i8x8, t1: i8x8, t2: i8x8, idx: i8x8) -> i8x8;
    fn arm_vtbl3_u8(t0: u8x8, t1: u8x8, t2: u8x8, idx: u8x8) -> u8x8;
    fn arm_vtbl4_s8(t0: i8x8, t1: i8x8, t2: i8x8, t3: i8x8, idx: i8x8) -> i8x8;
    fn arm_vtbl4_u8(t0: u8x8, t1: u8x8, t2: u8x8, t3: u8x8, idx: u8x8) -> u8x8;
    fn arm_vtbx1_s8(x: i8x8, t0: i8x8, idx: i8x8) -> i8x8;
    fn arm_vtbx1_u8(x: u8x8, t0: u8x8, idx: u8x8) -> u8x8;
    fn arm_vtbx2_s8(x: i8x8, t0: i8x8, t1: i8x8, idx: i8x8) -> i8x8;
    fn arm_vtbx2_u8(x: u8x8, t0: u8x8, t1: u8x8, idx: u8x8) -> u8x8;
    fn arm_vtbx3_s8(x: i8x8, t0: i8x8, t1: i8x8, t2: i8x8, idx: i8x8) -> i8x8;
    fn arm_vtbx3_u8(x: u8x8, t0: u8x8, t1: u8x8, t2: u8x8, idx: u8x8) -> u8x8;
    fn arm_vtbx4_s8(x: i8x8, t0: i8x8, t1: i8x8, t2: i8x8, t3: i8x8, idx: i8x8) -> i8x8;
    fn arm_vtbx4_u8(x: u8x8, t0: u8x8, t1: u8x8, t2: u8x8, t3: u8x8, idx: u8x8) -> u8x8;
}

impl u8x8 {
    #[inline]
    pub fn table_lookup_1(self, t0: u8x8) -> u8x8 {
        unsafe {arm_vtbl1_u8(t0, self)}
    }
    #[inline]
    pub fn table_lookup_2(self, t0: u8x8, t1: u8x8) -> u8x8 {
        unsafe {arm_vtbl2_u8(t0, t1, self)}
    }
    #[inline]
    pub fn table_lookup_3(self, t0: u8x8, t1: u8x8, t2: u8x8) -> u8x8 {
        unsafe {arm_vtbl3_u8(t0, t1, t2, self)}
    }
    #[inline]
    pub fn table_lookup_4(self, t0: u8x8, t1: u8x8, t2: u8x8, t3: u8x8) -> u8x8 {
        unsafe {arm_vtbl4_u8(t0, t1, t2, t3, self)}
    }
}

#[doc(hidden)]
pub mod common {
    use super::super::super::*;
    use super::*;
    use std::mem;

    #[inline]
    pub fn f32x4_sqrt(x: f32x4) -> f32x4 {
        unsafe {super::arm_vsqrtq_f32(x)}
    }
    #[inline]
    pub fn f32x4_approx_rsqrt(x: f32x4) -> f32x4 {
        unsafe {super::arm_vrsqrteq_f32(x)}
    }
    #[inline]
    pub fn f32x4_approx_reciprocal(x: f32x4) -> f32x4 {
        unsafe {super::arm_vrecpeq_f32(x)}
    }
    #[inline]
    pub fn f32x4_max(x: f32x4, y: f32x4) -> f32x4 {
        unsafe {super::arm_vmaxq_f32(x, y)}
    }
    #[inline]
    pub fn f32x4_min(x: f32x4, y: f32x4) -> f32x4 {
        unsafe {super::arm_vminq_f32(x, y)}
    }

    macro_rules! bools {
        ($($ty: ty, $half: ty, $all: ident ($min: ident), $any: ident ($max: ident);)*) => {
            $(
                #[inline]
                pub fn $all(x: $ty) -> bool {
                    unsafe {
                        let (lo, hi): ($half, $half) = mem::transmute(x);
                        let x = super::$min(lo, hi);
                        let y = super::$min(x, mem::uninitialized());
                        y.0 != 0
                    }
                }
                #[inline]
                pub fn $any(x: $ty) -> bool {
                    unsafe {
                        let (lo, hi): ($half, $half) = mem::transmute(x);
                        let x = super::$max(lo, hi);
                        let y = super::$max(x, mem::uninitialized());
                        y.0 != 0
                    }
                }
                )*
        }
    }

    bools! {
        bool32fx4, u32x2, bool32fx4_all(arm_vpmin_u32), bool32fx4_any(arm_vpmax_u32);
        bool8ix16, u8x8, bool8ix16_all(arm_vpmin_u8), bool8ix16_any(arm_vpmax_u8);
        bool16ix8, u16x4, bool16ix8_all(arm_vpmin_u16), bool16ix8_any(arm_vpmax_u16);
        bool32ix4, u32x2, bool32ix4_all(arm_vpmin_u32), bool32ix4_any(arm_vpmax_u32);
    }
}
