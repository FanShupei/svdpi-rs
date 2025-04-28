use std::{ffi, marker::PhantomData};

use super::*;

#[repr(transparent)]
pub struct In<T: SvBasicType> {
    inner: T::Underlying,
}

impl<T: SvBasicType> In<T> {
    pub fn get(&self) -> T {
        T::from_underlying(self.inner)
    }
}

#[repr(transparent)]
pub struct Out<'a, T: SvBasicType> {
    ptr: &'a mut T::Underlying,
}

impl<T: SvBasicType> Out<'_, T> {
    pub fn set(&mut self, value: T) {
        *self.ptr = value.to_underlying();
    }
}

#[repr(transparent)]
pub struct Inout<'a, T: SvBasicType> {
    ptr: &'a mut T::Underlying,
}

impl<T: SvBasicType> Inout<'_, T> {
    pub fn get(&self) -> T {
        T::from_underlying(*self.ptr)
    }

    pub fn set(&mut self, value: T) {
        *self.ptr = value.to_underlying();
    }
}

#[repr(transparent)]
pub struct Ret<T: SvBasicType> {
    #[allow(unused)]
    inner: T::Underlying,
}

impl<T: SvBasicType> From<T> for Ret<T> {
    fn from(value: T) -> Self {
        Ret {
            inner: value.to_underlying(),
        }
    }
}

// we use supertrait sealed trait trick
// to prevent downstream implements SvBasicType
trait SvBasicTypePriv {}

#[allow(private_bounds)]
pub trait SvBasicType: Copy + SvBasicTypePriv {
    type Underlying: Copy;

    fn from_underlying(x: Self::Underlying) -> Self;
    fn to_underlying(self) -> Self::Underlying;
}

// $T and $U are actually the same, no need for any conversion
macro_rules! impl_sv_basic_type {
    ($T: ty, $U: ty) => {
        impl SvBasicTypePriv for $T {}
        impl SvBasicType for $T {
            type Underlying = $U;
            fn from_underlying(x: Self::Underlying) -> Self {
                x
            }
            fn to_underlying(self) -> Self::Underlying {
                self
            }
        }
    };
}

// See LRM 2023 Table H.1 (Annex H) - Mapping Data Types
impl_sv_basic_type!(i8, ffi::c_char); // byte
impl_sv_basic_type!(u8, ffi::c_uchar); // byte unsigned
impl_sv_basic_type!(i16, ffi::c_short); // shortint
impl_sv_basic_type!(u16, ffi::c_ushort); // shortint unsigned
impl_sv_basic_type!(i32, ffi::c_int); // int
impl_sv_basic_type!(u32, ffi::c_uint); // int unsigned
impl_sv_basic_type!(i64, ffi::c_longlong); // int
impl_sv_basic_type!(u64, ffi::c_ulonglong); // int unsigned
impl_sv_basic_type!(f64, ffi::c_double); // real
impl_sv_basic_type!(f32, ffi::c_float); // shortreal

fn _assert_type_equality() {
    trait Identity {
        type This;
    }

    impl<T> Identity for T {
        type This = T;
    }

    fn type_eq<T, U: Identity<This = T>>() {}

    type_eq::<i8, ffi::c_char>();
    type_eq::<u8, ffi::c_uchar>();
    type_eq::<i16, ffi::c_short>();
    type_eq::<u16, ffi::c_ushort>();
    type_eq::<i32, ffi::c_int>();
    type_eq::<u32, ffi::c_uint>();
    type_eq::<i64, ffi::c_longlong>();
    type_eq::<u64, ffi::c_ulonglong>();
    type_eq::<f64, ffi::c_double>();
    type_eq::<f32, ffi::c_float>();
}

// encodings are defined in svdpi.h, sv_0 / sv_1
impl SvBasicType for bool {
    type Underlying = ffi::c_uchar;
    fn from_underlying(x: Self::Underlying) -> Self {
        match x {
            0 => false,
            1 => true,
            _ => unreachable!(),
        }
    }
    fn to_underlying(self) -> Self::Underlying {
        match self {
            true => 1,
            false => 0,
        }
    }
}
impl SvBasicTypePriv for bool {}

// encodings are defined in svdpi.h, sv_0 / sv_1 / sv_z / sv_x
impl SvBasicType for Logic {
    type Underlying = ffi::c_uchar;
    fn from_underlying(x: Self::Underlying) -> Self {
        match x {
            0 => Logic::Value0,
            1 => Logic::Value1,
            2 => Logic::Z,
            3 => Logic::X,
            _ => unreachable!(),
        }
    }
    fn to_underlying(self) -> Self::Underlying {
        self.into()
    }
}
impl SvBasicTypePriv for Logic {}

impl<T> SvBasicType for *mut T {
    type Underlying = *mut c_void;

    fn from_underlying(x: Self::Underlying) -> Self {
        x as *mut T
    }

    fn to_underlying(self) -> Self::Underlying {
        self as *mut c_void
    }
}
impl<T> SvBasicTypePriv for *mut T {}

#[repr(transparent)]
pub struct InStr<'a> {
    inner: *const c_char,
    phantom: PhantomData<&'a mut u8>,
}

impl<'a> InStr<'a> {
    pub fn get(&self) -> &'a CStr {
        // Safety : InStr must be constructed from a valid C-style string
        unsafe { CStr::from_ptr(self.inner) }
    }
}

#[repr(transparent)]
pub struct RetStr {
    inner: *const c_char,
}

impl From<&'static CStr> for RetStr {
    fn from(value: &'static CStr) -> Self {
        Self {
            inner: value.as_ptr(),
        }
    }
}

impl RetStr {
    pub unsafe fn from_ptr_unchecked(ptr: *const c_char) -> Self {
        Self { inner: ptr }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct InBV<'a, const BITS: usize> {
    inner: *const u32,
    phantom: PhantomData<&'a [u32]>,
}

impl<'a, const BITS: usize> InBV<'a, BITS> {
    const U32_LEN: usize = (BITS + 31) / 32;

    #[cfg(target_endian = "little")]
    const U8_LEN: usize = (BITS + 7) / 8;

    pub fn as_ptr(&self) -> *const u32 {
        self.inner
    }

    pub fn as_array<const LEN: usize>(&self) -> &'a [u32; LEN] {
        assert_eq!(LEN, Self::U32_LEN);
        unsafe { &*(self.inner as *const [u32; LEN]) }
    }

    #[cfg(target_endian = "little")]
    pub fn as_u8_array<const LEN: usize>(&self) -> &'a [u8; LEN] {
        assert_eq!(LEN, Self::U8_LEN);
        unsafe { &*(self.inner as *const [u8; LEN]) }
    }

    pub fn as_slice(&self) -> &'a [u32] {
        unsafe { std::slice::from_raw_parts(self.inner, Self::U32_LEN) }
    }

    #[cfg(target_endian = "little")]
    pub fn as_u8_slice(&self) -> &'a [u8] {
        unsafe { std::slice::from_raw_parts(self.inner as *const u8, Self::U8_LEN) }
    }
}

#[repr(transparent)]
pub struct OutBV<'a, const BITS: usize> {
    inner: *mut u32,
    phantom: PhantomData<&'a mut [u32]>,
}

impl<const BITS: usize> OutBV<'_, BITS> {
    const U32_LEN: usize = (BITS + 31) / 32;

    #[cfg(target_endian = "little")]
    const U8_LEN: usize = (BITS + 7) / 8;

    pub fn as_ptr(&self) -> *mut u32 {
        self.inner
    }

    pub fn as_array<const LEN: usize>(&self) -> &[u32; LEN] {
        assert_eq!(LEN, Self::U32_LEN);
        unsafe { &*(self.inner as *const [u32; LEN]) }
    }

    pub fn as_array_mut<const LEN: usize>(&mut self) -> &mut [u32; LEN] {
        assert_eq!(LEN, Self::U32_LEN);
        unsafe { &mut *(self.inner as *mut [u32; LEN]) }
    }

    #[cfg(target_endian = "little")]
    pub fn as_u8_array<const LEN: usize>(&self) -> &[u8; LEN] {
        assert_eq!(LEN, Self::U8_LEN);
        unsafe { &*(self.inner as *const [u8; LEN]) }
    }

    #[cfg(target_endian = "little")]
    pub fn as_u8_array_mut<const LEN: usize>(&mut self) -> &mut [u8; LEN] {
        assert_eq!(LEN, Self::U8_LEN);
        unsafe { &mut *(self.inner as *mut [u8; LEN]) }
    }

    pub fn as_slice(&self) -> &[u32] {
        unsafe { std::slice::from_raw_parts(self.inner, Self::U32_LEN) }
    }

    pub fn as_slice_mut(&mut self) -> &mut [u32] {
        unsafe { std::slice::from_raw_parts_mut(self.inner, Self::U32_LEN) }
    }

    #[cfg(target_endian = "little")]
    pub fn as_u8_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.inner as *const u8, Self::U8_LEN) }
    }

    #[cfg(target_endian = "little")]
    pub fn as_u8_slice_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.inner as *mut u8, Self::U8_LEN) }
    }
}

pub type InoutBV<'a, const BITS: usize> = OutBV<'a, BITS>;
