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
