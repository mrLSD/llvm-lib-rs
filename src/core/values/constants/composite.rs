//! Functions in this group operate on composite constants.

use super::ValueRef;
use crate::core::context::ContextRef;
use crate::core::types::TypeRef;
use crate::{CInt, CStr, CString, CUint, GetRef};
use llvm_sys::core;

impl ValueRef {
    /// Create a `ConstantDataSequential` and initialize it with a string.
    #[must_use]
    pub fn const_string_in_context(
        context: &ContextRef,
        string: &str,
        dont_null_terminate: bool,
    ) -> Self {
        let c_string = CString::from(string);
        unsafe {
            Self(core::LLVMConstStringInContext(
                context.get_ref(),
                c_string.as_ptr(),
                *CUint::from(string.len()),
                *CInt::from(dont_null_terminate),
            ))
        }
    }

    /// Create a `ConstantDataSequential` with string content in the global context.
    ///
    /// This is the same as `const_string_in_context` except it operates on the
    /// global context.
    #[must_use]
    pub fn const_string(string: &str, dont_null_terminate: bool) -> Self {
        let c_string = CString::from(string);
        unsafe {
            Self(core::LLVMConstString(
                c_string.as_ptr(),
                *CUint::from(string.len()),
                *CInt::from(dont_null_terminate),
            ))
        }
    }

    /// Returns true if the specified constant is an array of `i8`.
    #[must_use]
    pub fn is_constant_string(&self) -> bool {
        unsafe { core::LLVMIsConstantString(self.0) != 0 }
    }

    /// Get the given constant data sequential as a string.
    #[must_use]
    pub fn get_as_string(&self) -> Option<String> {
        unsafe {
            let mut length = 0;
            let c_str = core::LLVMGetAsString(self.0, &mut length);
            if c_str.is_null() {
                None
            } else {
                Some(CStr::new(c_str).to_string())
            }
        }
    }

    /// Create an anonymous `ConstantStruct` with the specified values.
    #[must_use]
    pub fn const_struct_in_context(
        context: &ContextRef,
        constant_vals: &[Self],
        packed: bool,
    ) -> Self {
        let mut constant_vals = constant_vals.iter().map(|v| v.0).collect::<Vec<_>>();
        let constant_vals_ptr = if constant_vals.is_empty() {
            std::ptr::null_mut()
        } else {
            constant_vals.as_mut_ptr()
        };
        unsafe {
            Self(core::LLVMConstStructInContext(
                context.get_ref(),
                constant_vals_ptr,
                *CUint::from(constant_vals.len()),
                *CInt::from(packed),
            ))
        }
    }

    /// Create a `ConstantStruct` in the global `Context`.
    ///
    /// This is the same as `constStruct_in_context` except it operates on the
    /// global context.
    #[must_use]
    pub fn const_struct(constant_vals: &[Self], packed: bool) -> Self {
        let mut constant_vals = constant_vals.iter().map(|v| v.0).collect::<Vec<_>>();
        let constant_vals_ptr = if constant_vals.is_empty() {
            std::ptr::null_mut()
        } else {
            constant_vals.as_mut_ptr()
        };
        unsafe {
            Self(core::LLVMConstStruct(
                constant_vals_ptr,
                *CUint::from(constant_vals.len()),
                *CInt::from(packed),
            ))
        }
    }

    /// Create a `ConstantArray` from values.
    #[must_use]
    pub fn const_array2(element_type: &TypeRef, constant_vals: &[Self]) -> Self {
        let mut constant_vals = constant_vals.iter().map(|v| v.0).collect::<Vec<_>>();
        let constant_vals_ptr = if constant_vals.is_empty() {
            std::ptr::null_mut()
        } else {
            constant_vals.as_mut_ptr()
        };
        unsafe {
            Self(core::LLVMConstArray2(
                element_type.get_ref(),
                constant_vals_ptr,
                u64::try_from(constant_vals.len()).unwrap_or(u64::MAX),
            ))
        }
    }

    /// Create a non-anonymous `ConstantStruct` from values.
    #[must_use]
    pub fn const_named_struct(struct_type: &TypeRef, constant_vals: &[Self]) -> Self {
        let mut constant_vals = constant_vals.iter().map(|v| v.0).collect::<Vec<_>>();
        let constant_vals_ptr = if constant_vals.is_empty() {
            std::ptr::null_mut()
        } else {
            constant_vals.as_mut_ptr()
        };
        unsafe {
            Self(core::LLVMConstNamedStruct(
                struct_type.get_ref(),
                constant_vals_ptr,
                *CUint::from(constant_vals.len()),
            ))
        }
    }

    /// Get element of a constant aggregate `(struct, array or vector)` at the
    /// specified index. Returns `None` if the index is out of range, or it's not
    /// possible to determine the element (e.g., because the constant is a
    /// constant expression.)
    #[must_use]
    pub fn get_aggregate_element(&self, idx: u32) -> Option<Self> {
        let element = unsafe { core::LLVMGetAggregateElement(self.0, *CUint::from(idx)) };
        if element.is_null() {
            None
        } else {
            Some(Self(element))
        }
    }

    /// Create a `ConstantVector` from values.
    #[must_use]
    pub fn const_vector(scalar_constant_vals: &[Self]) -> Self {
        let mut scalar_constant_vals = scalar_constant_vals.iter().map(|v| v.0).collect::<Vec<_>>();
        let scalar_constant_vals_ptr = if scalar_constant_vals.is_empty() {
            std::ptr::null_mut()
        } else {
            scalar_constant_vals.as_mut_ptr()
        };
        unsafe {
            Self(core::LLVMConstVector(
                scalar_constant_vals_ptr,
                *CUint::from(scalar_constant_vals.len()),
            ))
        }
    }
}
