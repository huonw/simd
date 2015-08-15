use super::*;
#[allow(unused_imports)]
use super::{
    simd_eq, simd_ne, simd_lt, simd_le, simd_gt, simd_ge,
    simd_shuffle2, simd_shuffle4, simd_shuffle8, simd_shuffle16,
    simd_insert, simd_extract,
    simd_cast,
    simd_add, simd_sub, simd_mul, simd_div, simd_shl, simd_shr, simd_and, simd_or, simd_xor,

    Unalign, bitcast,
};
use std;

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
    bool32ix4: bool32i, i32x4, i32, 2, x0, x1 | x2, x3 [to_f -> bool32fx4];
    bool32fx4: bool32f, i32x4, i32, 2, x0, x1 | x2, x3 [to_i -> bool32ix4];

    bool16ix8: bool16i, i16x8, i16, 2, x0, x1, x2, x3 | x4, x5, x6, x7 [];

    bool8ix16: bool8i, i8x16, i8, 2, x0, x1, x2, x3, x4, x5, x6, x7 | x8, x9, x10, x11, x12, x13, x14, x15 [];
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
    i32x4,
    i16x8,
    i8x16,
}
neg_impls! {
    0.0,
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
    i32x4,
    i16x8,
    i8x16,
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
        i8x16, u8x16, i16x8, u16x8, i32x4, u32x4,
        f32x4;
    Sub (simd_sub, sub):
        i8x16, u8x16, i16x8, u16x8, i32x4, u32x4,
        f32x4;
    Mul (simd_mul, mul):
        i8x16, u8x16, i16x8, u16x8, i32x4, u32x4,
        f32x4;
    Div (simd_div, div): f32x4;

    BitAnd (simd_and, bitand):
        i8x16, u8x16, i16x8, u16x8, i32x4, u32x4,
        bool8ix16, bool16ix8, bool32ix4,
        bool32fx4;
    BitOr (simd_or, bitor):
        i8x16, u8x16, i16x8, u16x8, i32x4, u32x4,
        bool8ix16, bool16ix8, bool32ix4,
        bool32fx4;
    BitXor (simd_xor, bitxor):
        i8x16, u8x16, i16x8, u16x8, i32x4, u32x4,
        bool8ix16, bool16ix8, bool32ix4,
        bool32fx4;
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
    i8x16, u8x16, i16x8, u16x8, i32x4, u32x4
}
