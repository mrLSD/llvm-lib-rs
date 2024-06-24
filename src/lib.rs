#![deny(clippy::nursery, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::new_without_default)]
pub mod basic_block;
pub mod builder;
pub mod context;
pub mod core;
pub mod module;
pub mod types;
pub mod value;

use libc::{c_uint, size_t};
use std::ops::Deref;

/// `c_uint` wrapper (from C-type)
pub struct CUint(c_uint);

impl From<u32> for CUint {
    fn from(value: u32) -> Self {
        // Force to unwrap c_uint to u32 with expect fail message
        Self(c_uint::try_from(value).expect("c_unit casting fail from u32"))
    }
}

impl From<usize> for CUint {
    fn from(value: usize) -> Self {
        // Force to unwrap c_uint
        Self(c_uint::try_from(value).expect("c_unit casting fail from usize"))
    }
}

impl Deref for CUint {
    type Target = c_uint;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// `size_t` wrapper (from C-type)
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

/// `CString` wrapper
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

/// Wrapping for `*mut c_void`
#[derive(Copy, Clone)]
pub struct UnsafeMutVoidPtr(*mut std::ffi::c_void);

impl Deref for UnsafeMutVoidPtr {
    type Target = *mut std::ffi::c_void;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
