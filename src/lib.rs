#![cfg_attr(feature = "serde", feature(plugin, custom_derive))]
#![cfg_attr(feature = "serde", plugin(serde_macros))]

#![feature(cfg_target_feature, repr_simd, platform_intrinsics)]
#![allow(non_camel_case_types)]

#[cfg(feature = "serde")]
extern crate serde;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct bool8i(i8);
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct bool16i(i16);
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct bool32i(i32);
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct bool64i(i64);
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct bool32f(i32);
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct bool64f(i64);

pub unsafe trait Simd {
    type Bool: Simd;
    type Elem;
}

#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct u64x2(u64, u64);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct i64x2(i64, i64);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct f64x2(f64, f64);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct bool64ix2(i64, i64);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct bool64fx2(i64, i64);

#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct u32x4(u32, u32, u32, u32);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct i32x4(i32, i32, i32, i32);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct f32x4(f32, f32, f32, f32);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct bool32ix4(i32, i32, i32, i32);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct bool32fx4(i32, i32, i32, i32);

#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct u16x8(u16, u16, u16, u16,
                 u16, u16, u16, u16);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct i16x8(i16, i16, i16, i16,
                 i16, i16, i16, i16);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct bool16ix8(i16, i16, i16, i16,
                     i16, i16, i16, i16);

#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct u8x16(u8, u8, u8, u8, u8, u8, u8, u8,
                 u8, u8, u8, u8, u8, u8, u8, u8);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct i8x16(i8, i8, i8, i8, i8, i8, i8, i8,
                 i8, i8, i8, i8, i8, i8, i8, i8);
#[repr(simd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy)]
pub struct bool8ix16(i8, i8, i8, i8, i8, i8, i8, i8,
                     i8, i8, i8, i8, i8, i8, i8, i8);


macro_rules! simd {
    ($($bool: ty: $($ty: ty = $elem: ty),*;)*) => {
        $($(unsafe impl Simd for $ty {
            type Bool = $bool;
            type Elem = $elem;
        }
            impl Clone for $ty { fn clone(&self) -> Self { *self } }
            )*)*}
}
simd! {
    bool8ix16: i8x16 = i8, u8x16 = u8, bool8ix16 = bool8i;
    bool16ix8: i16x8 = i16, u16x8 = u16, bool16ix8 = bool16i;
    bool32ix4: i32x4 = i32, u32x4 = u32, bool32ix4 = bool32i;
    bool32fx4: f32x4 = f32, bool32fx4 = bool32f;
    bool64ix2: i64x2 = i64, u64x2 = u64, bool64ix2 = bool64i;
    bool64fx2: f64x2 = f64, bool64fx2 = bool64f;
}

#[allow(dead_code)]
#[repr(simd)]
struct Simd2<T>(T, T);
#[allow(dead_code)]
#[repr(simd)]
pub struct Simd4<T>(T, T, T, T);
#[allow(dead_code)]
#[repr(simd)]
pub struct Simd8<T>(T, T, T, T, T, T, T, T);
#[allow(dead_code)]
#[repr(simd)]
struct Simd16<T>(T, T, T, T, T, T, T, T,
                 T, T, T, T, T, T, T, T);

impl Copy for Simd2<f32> {}
impl Copy for Simd2<bool32f> {}
simd! {
    Simd2<bool32f>: Simd2<f32> = f32, Simd2<bool32f> = bool32f;
}

#[allow(dead_code)]
extern "platform-intrinsic" {
    fn simd_eq<T: Simd<Bool = U>, U>(x: T, y: T) -> U;
    fn simd_ne<T: Simd<Bool = U>, U>(x: T, y: T) -> U;
    fn simd_lt<T: Simd<Bool = U>, U>(x: T, y: T) -> U;
    fn simd_le<T: Simd<Bool = U>, U>(x: T, y: T) -> U;
    fn simd_gt<T: Simd<Bool = U>, U>(x: T, y: T) -> U;
    fn simd_ge<T: Simd<Bool = U>, U>(x: T, y: T) -> U;

    fn simd_shuffle2<T: Simd<Elem = U>, U>(x: T, y: T, i0: u32, i1: u32) -> Simd2<U>;
    fn simd_shuffle4<T: Simd<Elem = U>, U>(x: T, y: T, i0: u32, i1: u32, i2: u32, i3: u32) -> Simd4<U>;
    fn simd_shuffle8<T: Simd<Elem = U>, U>(x: T, y: T, i0: u32, i1: u32, i2: u32, i3: u32, i4: u32, i5: u32, i6: u32, i7: u32) -> Simd8<U>;
    fn simd_shuffle16<T: Simd<Elem = U>, U>(x: T, y: T,
                                            i0: u32, i1: u32, i2: u32, i3: u32, i4: u32, i5: u32, i6: u32, i7: u32,
                                            i8: u32, i9: u32, i10: u32, i11: u32, i12: u32, i13: u32, i14: u32, i15: u32)
                                            -> Simd16<U>;

    fn simd_insert<T: Simd<Elem = U>, U>(x: T, idx: u32, val: U) -> T;
    fn simd_extract<T: Simd<Elem = U>, U>(x: T, idx: u32) -> U;

    fn simd_cast<T: Simd, U: Simd>(x: T) -> U;

    fn simd_add<T: Simd>(x: T, y: T) -> T;
    fn simd_sub<T: Simd>(x: T, y: T) -> T;
    fn simd_mul<T: Simd>(x: T, y: T) -> T;
    fn simd_div<T: Simd>(x: T, y: T) -> T;
    fn simd_shl<T: Simd>(x: T, y: T) -> T;
    fn simd_shr<T: Simd>(x: T, y: T) -> T;
    fn simd_and<T: Simd>(x: T, y: T) -> T;
    fn simd_or<T: Simd>(x: T, y: T) -> T;
    fn simd_xor<T: Simd>(x: T, y: T) -> T;
}

#[allow(dead_code)]
#[inline]
fn bitcast<T: Simd, U: Simd>(x: T) -> U {
    assert_eq!(std::mem::size_of::<T>(),
               std::mem::size_of::<U>());
    unsafe {std::mem::transmute_copy(&x)}
}

#[repr(packed)]
#[derive(Debug, Copy, Clone)]
struct Unalign<T>(T);

macro_rules! basic_impls {
    ($(
        $name: ident:
        $elem: ident, $bool: ident, $shuffle: ident, $length: expr, $($first: ident),* | $($last: ident),*;
        )*) => {
        $(impl $name {
            #[inline]
            pub fn new($($first: $elem),*, $($last: $elem),*) -> $name {
                $name($($first),*, $($last),*)
            }

            #[allow(unused_variables)]
            #[inline]
            pub fn splat(x: $elem) -> $name {
                $name($({ let $first = ();  x}),*,
                      $({ let $last = ();  x}),*)
            }

            #[inline]
            pub fn eq(self, other: Self) -> $bool {
                unsafe {simd_eq(self, other)}
            }
            #[inline]
            pub fn ne(self, other: Self) -> $bool {
                unsafe {simd_ne(self, other)}
            }
            #[inline]
            pub fn lt(self, other: Self) -> $bool {
                unsafe {simd_lt(self, other)}
            }
            #[inline]
            pub fn le(self, other: Self) -> $bool {
                unsafe {simd_le(self, other)}
            }
            #[inline]
            pub fn gt(self, other: Self) -> $bool {
                unsafe {simd_gt(self, other)}
            }
            #[inline]
            pub fn ge(self, other: Self) -> $bool {
                unsafe {simd_ge(self, other)}
            }

            #[inline]
            pub fn extract(self, idx: u32) -> $elem {
                assert!(idx < $length);
                unsafe {simd_extract(self, idx)}
            }
            #[inline]
            pub fn replace(self, idx: u32, elem: $elem) -> Self {
                assert!(idx < $length);
                unsafe {simd_insert(self, idx, elem)}
            }

            #[inline]
            pub fn load(array: &[$elem], idx: usize) -> Self {
                let data = &array[idx..idx + $length];
                let loaded = unsafe {
                    *(data.as_ptr() as *const Unalign<Self>)
                };
                loaded.0
            }

            #[inline]
            pub fn store(self, array: &mut [$elem], idx: usize) {
                let place = &mut array[idx..idx + $length];
                unsafe {
                    *(place.as_mut_ptr() as *mut Unalign<Self>) = Unalign(self)
                }
            }
        })*
    }
}

basic_impls! {
    u64x2: u64, bool64ix2, simd_shuffle2, 2, x0 | x1;
    i64x2: i64, bool64ix2, simd_shuffle2, 2, x0 | x1;
    f64x2: f64, bool64fx2, simd_shuffle2, 2, x0 | x1;

    u32x4: u32, bool32ix4, simd_shuffle4, 4, x0, x1 | x2, x3;
    i32x4: i32, bool32ix4, simd_shuffle4, 4, x0, x1 | x2, x3;
    f32x4: f32, bool32fx4, simd_shuffle4, 4, x0, x1 | x2, x3;

    u16x8: u16, bool16ix8, simd_shuffle8, 8, x0, x1, x2, x3 | x4, x5, x6, x7;
    i16x8: i16, bool16ix8, simd_shuffle8, 8, x0, x1, x2, x3 | x4, x5, x6, x7;

    u8x16: u8, bool8ix16, simd_shuffle16, 16, x0, x1, x2, x3, x4, x5, x6, x7 | x8, x9, x10, x11, x12, x13, x14, x15;
    i8x16: i8, bool8ix16, simd_shuffle16, 16, x0, x1, x2, x3, x4, x5, x6, x7 | x8, x9, x10, x11, x12, x13, x14, x15;
}

macro_rules! bool_impls {
    ($(
        $name: ident:
        $elem: ident, $repr: ident, $repr_elem: ident, $length: expr, $($first: ident),* | $($last: ident),*
        [$($cvt: ident -> $cvt_to: ident),*];
        )*) => {
        $(impl $name {
            fn to_repr(self) -> $repr {
                unsafe {std::mem::transmute(self)}
            }
            fn from_repr(x: $repr) -> Self {
                unsafe {std::mem::transmute(x)}
            }

            #[inline]
            pub fn new($($first: bool),*, $($last: bool),*) -> $name {
                unsafe {
                    // negate everything together
                    simd_sub($name::splat(false),
                             $name($( ($first as $repr_elem) ),*,
                                   $( ($last as $repr_elem) ),*))
                }
            }

            #[allow(unused_variables)]
            #[inline]
            pub fn splat(x: bool) -> $name {
                let x = if x {!(0 as $repr_elem)} else {0};
                $name($({ let $first = (); x}),*,
                      $({ let $last = (); x}),*)
            }

            #[inline]
            pub fn extract(self, idx: u32) -> bool {
                assert!(idx < $length);
                unsafe {simd_extract(self.to_repr(), idx) != 0}
            }
            #[inline]
            pub fn replace(self, idx: u32, elem: bool) -> Self {
                assert!(idx < $length);
                let x = if elem {!(0 as $repr_elem)} else {0};
                unsafe {Self::from_repr(simd_insert(self.to_repr(), idx, x))}
            }
            #[inline]
            pub fn select<T: Simd<Bool = $name>>(self, then: T, else_: T) -> T {
                let then: $repr = bitcast(then);
                let else_: $repr = bitcast(else_);
                bitcast((then & self.to_repr()) | (else_ & (!self).to_repr()))
            }

            $(
                #[inline]
                pub fn $cvt(self) -> $cvt_to {
                    bitcast(self)
                }
                )*
        }
          impl std::ops::Not for $name {
              type Output = Self;

              #[inline]
              fn not(self) -> Self {
                  Self::from_repr($repr::splat(!(0 as $repr_elem)) ^ self.to_repr())
              }
          }
          )*
    }
}

bool_impls! {
    bool64ix2: bool64i, i64x2, i64, 2, x0 | x1 [to_f -> bool64fx2];
    bool64fx2: bool64f, i64x2, i64, 2, x0 | x1 [to_i -> bool64ix2];

    bool32ix4: bool32i, i32x4, i32, 2, x0, x1 | x2, x3 [to_f -> bool32fx4];
    bool32fx4: bool32f, i32x4, i32, 2, x0, x1 | x2, x3 [to_i -> bool32ix4];

    bool16ix8: bool16i, i16x8, i16, 2, x0, x1, x2, x3 | x4, x5, x6, x7 [];

    bool8ix16: bool8i, i8x16, i8, 2, x0, x1, x2, x3, x4, x5, x6, x7 | x8, x9, x10, x11, x12, x13, x14, x15 [];
}

impl u64x2 {
    #[inline]
    pub fn to_i64(self) -> i64x2 {
        unsafe {simd_cast(self)}
    }
    #[inline]
    pub fn to_f64(self) -> f64x2 {
        unsafe {simd_cast(self)}
    }
}
impl i64x2 {
    #[inline]
    pub fn to_u64(self) -> u64x2 {
        unsafe {simd_cast(self)}
    }
    #[inline]
    pub fn to_f64(self) -> f64x2 {
        unsafe {simd_cast(self)}
    }
}
impl f64x2 {
    #[inline]
    pub fn to_i64(self) -> i64x2 {
        unsafe {simd_cast(self)}
    }
    #[inline]
    pub fn to_u64(self) -> u64x2 {
        unsafe {simd_cast(self)}
    }

    #[inline]
    pub fn to_f32(self) -> f32x4 {
        unsafe {
            let x: Simd2<f32> = simd_cast(self);
            f32x4::new(x.0, x.1, 0.0, 0.0)
        }
    }
}

impl u32x4 {
    #[inline]
    pub fn to_i32(self) -> i32x4 {
        unsafe {simd_cast(self)}
    }
    #[inline]
    pub fn to_f32(self) -> f32x4 {
        unsafe {simd_cast(self)}
    }
}
impl i32x4 {
    #[inline]
    pub fn to_u32(self) -> u32x4 {
        unsafe {simd_cast(self)}
    }
    #[inline]
    pub fn to_f32(self) -> f32x4 {
        unsafe {simd_cast(self)}
    }
}
impl f32x4 {
    #[inline]
    pub fn to_i32(self) -> i32x4 {
        unsafe {simd_cast(self)}
    }
    #[inline]
    pub fn to_u32(self) -> u32x4 {
        unsafe {simd_cast(self)}
    }
    #[inline]
    pub fn to_f64(self) -> f64x2 {
        unsafe {
            simd_cast(Simd2(self.0, self.1))
        }
    }
}

impl i16x8 {
    #[inline]
    pub fn to_u16(self) -> u16x8 {
        unsafe {simd_cast(self)}
    }
}
impl u16x8 {
    #[inline]
    pub fn to_i16(self) -> i16x8 {
        unsafe {simd_cast(self)}
    }
}

impl i8x16 {
    #[inline]
    pub fn to_u8(self) -> u8x16 {
        unsafe {simd_cast(self)}
    }
}
impl u8x16 {
    #[inline]
    pub fn to_i8(self) -> i8x16 {
        unsafe {simd_cast(self)}
    }
}


macro_rules! neg_impls {
    ($zero: expr, $($ty: ident,)*) => {
        $(impl std::ops::Neg for $ty {
            type Output = Self;
            fn neg(self) -> Self {
                $ty::splat($zero) - self
            }
        })*
    }
}
neg_impls!{
    0,
    i64x2,
    i32x4,
    i16x8,
    i8x16,
}
neg_impls! {
    0.0,
    f64x2,
    f32x4,
}
macro_rules! not_impls {
    ($($ty: ident,)*) => {
        $(impl std::ops::Not for $ty {
            type Output = Self;
            fn not(self) -> Self {
                $ty::splat(!0) ^ self
            }
        })*
    }
}
not_impls! {
    i64x2,
    i32x4,
    i16x8,
    i8x16,
    u64x2,
    u32x4,
    u16x8,
    u8x16,
}

macro_rules! operators {
    ($($trayt: ident ($func: ident, $method: ident): $($ty: ty),*;)*) => {
        $(
            $(impl std::ops::$trayt for $ty {
                type Output = Self;
                #[inline]
                fn $method(self, x: Self) -> Self {
                    unsafe {$func(self, x)}
                }
            })*
                )*
    }
}
operators! {
    Add (simd_add, add):
        i8x16, u8x16, i16x8, u16x8, i32x4, u32x4, i64x2, u64x2,
        f32x4, f64x2;
    Sub (simd_sub, sub):
        i8x16, u8x16, i16x8, u16x8, i32x4, u32x4, i64x2, u64x2,
        f32x4, f64x2;
    Mul (simd_mul, mul):
        i8x16, u8x16, i16x8, u16x8, i32x4, u32x4, i64x2, u64x2,
        f32x4, f64x2;
    Div (simd_div, div): f32x4, f64x2;

    BitAnd (simd_and, bitand):
        i8x16, u8x16, i16x8, u16x8, i32x4, u32x4, i64x2, u64x2,
        bool8ix16, bool16ix8, bool32ix4, bool64ix2,
        bool32fx4, bool64fx2;
    BitOr (simd_or, bitor):
        i8x16, u8x16, i16x8, u16x8, i32x4, u32x4, i64x2, u64x2,
        bool8ix16, bool16ix8, bool32ix4, bool64ix2,
        bool32fx4, bool64fx2;
    BitXor (simd_xor, bitxor):
        i8x16, u8x16, i16x8, u16x8, i32x4, u32x4, i64x2, u64x2,
        bool8ix16, bool16ix8, bool32ix4, bool64ix2,
        bool32fx4, bool64fx2;
}

macro_rules! shift_one {
    ($ty: ident, $($by: ident),*) => {
        $(
        impl std::ops::Shl<$by> for $ty {
            type Output = Self;
            #[inline]
            fn shl(self, other: $by) -> Self {
                unsafe { simd_shl(self, $ty::splat(other as <$ty as Simd>::Elem)) }
            }
        }
        impl std::ops::Shr<$by> for $ty {
            type Output = Self;
            fn shr(self, other: $by) -> Self {
                unsafe {simd_shr(self, $ty::splat(other as <$ty as Simd>::Elem))}
            }
        }
            )*
    }
}

macro_rules! shift {
    ($($ty: ident),*) => {
        $(shift_one! {
            $ty,
            u8, u16, u32, u64, usize,
            i8, i16, i32, i64, isize
        })*
    }
}
shift! {
    i8x16, u8x16, i16x8, u16x8, i32x4, u32x4, i64x2, u64x2
}

#[cfg(target_feature = "sse2")]
pub mod sse2;
#[cfg(target_feature = "ssse3")]
pub mod ssse3;
#[cfg(all(target_arch = "arm",
          target_feature = "neon"))]
mod neon;
#[cfg(target_arch = "aarch64")]
mod aarch64;
