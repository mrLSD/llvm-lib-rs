//! Functions in this group model `ValueRef` instances that correspond
//! to constants referring to scalar types.

use super::ValueRef;
use crate::core::types::TypeRef;
use crate::{CDouble, CInt, CString, CUint, GetRef};
use llvm_sys::core;

impl ValueRef {
    /// Obtain a constant value for an integer type.
    ///
    /// The returned value corresponds to a `llvm ConstantInt`.
    #[must_use]
    pub fn const_int(ty: &TypeRef, n: u64, sign_extend: bool) -> Self {
        unsafe {
            Self(core::LLVMConstInt(
                ty.get_ref(),
                n,
                *CInt::from(sign_extend),
            ))
        }
    }

    /// Obtain a constant value for an integer of arbitrary precision.
    #[must_use]
    pub fn const_int_of_arbitrary_precision(ty: &TypeRef, words: &[u64]) -> Self {
        unsafe {
            Self(core::LLVMConstIntOfArbitraryPrecision(
                ty.get_ref(),
                *CUint::from(words.len()),
                words.as_ptr(),
            ))
        }
    }

    /// Obtain a constant value for an integer parsed from a string.
    ///
    /// A similar API, `const_int_of_string_and_size` is also available. If the
    /// string's length is available, it is preferred to call that function
    /// instead.
    #[must_use]
    pub fn const_int_of_string(ty: &TypeRef, text: &str, radix: u8) -> Self {
        let c_text = CString::from(text);
        unsafe {
            Self(core::LLVMConstIntOfString(
                ty.get_ref(),
                c_text.as_ptr(),
                radix,
            ))
        }
    }

    /// Obtain a constant value for an integer parsed from a string with
    /// specified length.
    #[must_use]
    pub fn const_int_of_string_and_size(ty: &TypeRef, text: &str, radix: u8) -> Self {
        let c_text = CString::from(text);
        unsafe {
            Self(core::LLVMConstIntOfStringAndSize(
                ty.get_ref(),
                c_text.as_ptr(),
                *CUint::from(text.len()),
                radix,
            ))
        }
    }

    /// Obtain a constant value referring to a double floating point value.
    #[must_use]
    pub fn const_real(ty: &TypeRef, n: f64) -> Self {
        unsafe { Self(core::LLVMConstReal(ty.get_ref(), *CDouble::from(n))) }
    }

    /// Obtain a constant for a floating point value parsed from a string.
    ///
    /// A similar API, `const_real_of_string_and_size` is also available. It
    /// should be used if the input string's length is known.
    #[must_use]
    pub fn const_real_of_string(ty: &TypeRef, text: &str) -> Self {
        let c_text = CString::from(text);
        unsafe { Self(core::LLVMConstRealOfString(ty.get_ref(), c_text.as_ptr())) }
    }

    /// Obtain a constant for a floating point value parsed from a string with specified length.
    #[must_use]
    pub fn const_real_of_string_and_size(ty: &TypeRef, text: &str) -> Self {
        let c_text = CString::from(text);
        unsafe {
            Self(core::LLVMConstRealOfStringAndSize(
                ty.get_ref(),
                c_text.as_ptr(),
                *CUint::from(text.len()),
            ))
        }
    }

    /// Obtain the zero extended value for an integer constant value.
    #[must_use]
    pub fn const_int_get_zext_value(&self) -> u64 {
        unsafe { core::LLVMConstIntGetZExtValue(self.0) }
    }

    /// Obtain the sign extended value for an integer constant value.
    #[must_use]
    pub fn const_int_get_sext_value(&self) -> i64 {
        unsafe { core::LLVMConstIntGetSExtValue(self.0) }
    }

    /// Obtain the double value for a floating point constant value.
    /// `losesInfo` indicates if some precision was lost in the conversion.
    ///
    /// ## Returns
    /// `f64` constant value and `losesInfo` flag
    #[must_use]
    pub fn const_real_get_double(&self) -> (f64, bool) {
        let mut loses_info_c = 0;
        let result = unsafe { core::LLVMConstRealGetDouble(self.0, &mut loses_info_c) };
        (result, loses_info_c != 0)
    }
}
