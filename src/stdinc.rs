#![allow(non_camel_case_types)]

pub use core::ffi::*;

#[inline]
pub const fn SDL_arraysize<T, const N: usize>(_arr: &[T; N]) -> usize {
  N
}
#[inline]
pub const fn SDL_TABLESIZE<T, const N: usize>(_table: &[T; N]) -> usize {
  N
}
#[inline]
#[allow(clippy::identity_op)]
pub const fn SDL_FOURCC(A: u8, B: u8, C: u8, D: u8) -> u32 {
  ((A as u32) << 0)
    | ((B as u32) << 8)
    | ((C as u32) << 16)
    | ((D as u32) << 24)
}

pub type SDL_bool = bool32::Bool32;
