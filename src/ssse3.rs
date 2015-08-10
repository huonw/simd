use super::*;
use super::bitcast;

extern "platform-intrinsic" {
    fn x86_mm_shuffle_epi8(x: u8x16, y: u8x16) -> u8x16;
}

pub trait SSSE3Bytes {
    fn shuf(self, indices: Self) -> Self;
}

impl SSSE3Bytes for u8x16 {
    #[inline]
    fn shuf(self, indices: Self) -> Self {
        unsafe {x86_mm_shuffle_epi8(self, indices)}
    }
}

impl SSSE3Bytes for i8x16 {
    #[inline]
    fn shuf(self, indices: Self) -> Self {
        unsafe {
            bitcast(x86_mm_shuffle_epi8(bitcast(self),
                                        bitcast(indices)))
        }
    }
}
