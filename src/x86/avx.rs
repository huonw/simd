use super::super::*;
use sixty_four::*;

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
#[derive(Debug, Copy, Clone)]
pub struct f64x4(f64, f64, f64, f64);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct bool64ix4(i64, i64, i64, i64);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
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
#[derive(Debug, Copy, Clone)]
pub struct f32x8(f32, f32, f32, f32,
                 f32, f32, f32, f32);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct bool32ix8(i32, i32, i32, i32,
                     i32, i32, i32, i32);#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct bool32fix8(i32, i32, i32, i32,
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


#[allow(dead_code)]
extern "platform-intrinsic" {
    fn x86_mm_permutevar_pd(x: f64x2, y: i64x2) -> f64x2;
    fn x86_mm_permutevar_ps(x: f32x4, y: i32x4) -> f32x4;
    fn x86_mm_testc_pd(x: f64x2, y: f64x2) -> i32;
    fn x86_mm_testc_ps(x: f32x4, y: f32x4) -> i32;
    fn x86_mm_testnzc_pd(x: f64x2, y: f64x2) -> i32;
    fn x86_mm_testnzc_ps(x: f32x4, y: f32x4) -> i32;
    fn x86_mm_testz_pd(x: f64x2, y: f64x2) -> i32;
    fn x86_mm_testz_ps(x: f32x4, y: f32x4) -> i32;

    fn x86_mm256_addsub_pd(x: f64x4, y: f64x4) -> f64x4;
    fn x86_mm256_addsub_ps(x: f32x8, y: f32x8) -> f32x8;
    fn x86_mm256_hadd_pd(x: f64x4, y: f64x4) -> f64x4;
    fn x86_mm256_hadd_ps(x: f32x8, y: f32x8) -> f32x8;
    fn x86_mm256_hsub_pd(x: f64x4, y: f64x4) -> f64x4;
    fn x86_mm256_hsub_ps(x: f32x8, y: f32x8) -> f32x8;
    fn x86_mm256_max_pd(x: f64x4, y: f64x4) -> f64x4;
    fn x86_mm256_max_ps(x: f32x8, y: f32x8) -> f32x8;
    fn x86_mm256_min_pd(x: f64x4, y: f64x4) -> f64x4;
    fn x86_mm256_min_ps(x: f32x8, y: f32x8) -> f32x8;
    fn x86_mm256_permutevar_pd(x: f64x4, y: i64x4) -> f64x4;
    fn x86_mm256_permutevar_ps(x: f32x8, y: i32x8) -> f32x8;
    fn x86_mm256_rcp_ps(x: f32x8) -> f32x8;
    fn x86_mm256_rsqrt_ps(x: f32x8) -> f32x8;
    fn x86_mm256_sqrt_pd(x: f64x4) -> f64x4;
    fn x86_mm256_sqrt_ps(x: f32x8) -> f32x8;
    fn x86_mm256_testc_pd(x: f64x4, y: f64x4) -> i32;
    fn x86_mm256_testc_ps(x: f32x8, y: f32x8) -> i32;
    fn x86_mm256_testnzc_pd(x: f64x4, y: f64x4) -> i32;
    fn x86_mm256_testnzc_ps(x: f32x8, y: f32x8) -> i32;
    fn x86_mm256_testz_pd(x: f64x4, y: f64x4) -> i32;
    fn x86_mm256_testz_ps(x: f32x8, y: f32x8) -> i32;
}
