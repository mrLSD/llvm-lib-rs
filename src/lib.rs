#![deny(clippy::nursery, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::new_without_default)]
pub mod basic_block;
pub mod builder;
pub mod core;

use libc::{c_char, c_int, c_uint, size_t};
use std::ops::{Deref, DerefMut};

/// Get raw references trait
pub trait GetRef {
    /// Raw LLVM reference type
    type RawRef;
    /// Get LLVM raw reference
    fn get_ref(&self) -> Self::RawRef;
}

/// `c_uint` wrapper (from C-type)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUint(c_uint);

impl From<u32> for CUint {
    fn from(value: u32) -> Self {
        // Force to unwrap c_uint
        Self(c_uint::try_from(value).expect("c_unit casting fail from u32"))
    }
}

impl From<CUint> for u32 {
    fn from(value: CUint) -> Self {
        value.0
    }
}

impl From<usize> for CUint {
    fn from(value: usize) -> Self {
        // Force to unwrap c_uint
        Self(c_uint::try_from(value).expect("c_uint casting fail from usize"))
    }
}

impl Deref for CUint {
    type Target = c_uint;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CUint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// `c_int` wrapper (from C-type)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CInt(c_int);

impl From<i32> for CInt {
    fn from(value: i32) -> Self {
        // Force to unwrap c_int
        Self(c_int::try_from(value).expect("c_int casting fail from i32"))
    }
}

impl From<bool> for CInt {
    fn from(value: bool) -> Self {
        Self(c_int::from(value))
    }
}

impl Deref for CInt {
    type Target = c_int;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// `size_t` wrapper (from C-type)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SizeT(size_t);

impl From<usize> for SizeT {
    fn from(value: usize) -> Self {
        // Force to unwrap size_t
        Self(size_t::try_from(value).expect("size_t casting fail from usize"))
    }
}

impl Deref for SizeT {
    type Target = size_t;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SizeT {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// `CString` wrapper
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CString(std::ffi::CString);

impl From<&str> for CString {
    fn from(value: &str) -> Self {
        // Force to unwrap `CString`
        Self(std::ffi::CString::new(value).expect("CString casting fail from str"))
    }
}

impl Deref for CString {
    type Target = std::ffi::CString;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// `CStr` wrapper
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CStr<'a>(&'a std::ffi::CStr);

impl<'a> CStr<'a> {
    /// Initialize wrapped `CStr`
    /// ## Safety
    /// NOTE: Safety considerations same as for `std::ffi::CStr::from_ptr`.
    #[must_use]
    pub unsafe fn new(value: *const c_char) -> Self {
        unsafe { Self(std::ffi::CStr::from_ptr(value)) }
    }
}

impl<'a> Deref for CStr<'a> {
    type Target = std::ffi::CStr;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[allow(clippy::to_string_trait_impl)]
impl<'a> ToString for CStr<'a> {
    fn to_string(&self) -> String {
        self.0
            .to_str()
            .map(ToString::to_string)
            .expect("Failed to convert CStr to String")
    }
}

/// Wrapping for `*mut c_void`
#[derive(Debug, Copy, Clone)]
pub struct UnsafeMutVoidPtr(*mut std::ffi::c_void);

impl Deref for UnsafeMutVoidPtr {
    type Target = *mut std::ffi::c_void;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
