//! This section contains APIs for interacting with `MValueRef` that
//! correspond to `LLVM Constant` instances.

use super::ValueRef;
use crate::core::types::TypeRef;
use crate::GetRef;
use llvm_sys::core;

impl ValueRef {
    /// Obtain a constant value referring to the null instance of a type.
    #[must_use]
    pub fn const_null(ty: &TypeRef) -> Self {
        unsafe { Self(core::LLVMConstNull(ty.get_ref())) }
    }

    /// Obtain a constant value referring to the instance of a type
    /// consisting of all ones.
    ///
    /// This is only valid for integer types.
    #[must_use]
    pub fn const_all_ones(ty: &TypeRef) -> Self {
        unsafe { Self(core::LLVMConstAllOnes(ty.get_ref())) }
    }

    /// Obtain a constant value referring to an undefined value of a type.
    #[must_use]
    pub fn get_undef(ty: &TypeRef) -> Self {
        unsafe { Self(core::LLVMGetUndef(ty.get_ref())) }
    }

    /// Obtain a constant value referring to a poison value of a type.
    #[must_use]
    pub fn get_poison(ty: &TypeRef) -> Self {
        unsafe { Self(core::LLVMGetPoison(ty.get_ref())) }
    }

    /// Determine whether a value instance is null.
    #[must_use]
    pub fn is_null(&self) -> bool {
        unsafe { core::LLVMIsNull(self.0) != 0 }
    }

    /// Obtain a constant that is a constant pointer pointing to `NULL` for a
    /// specified type.
    #[must_use]
    pub fn const_pointer_null(ty: &TypeRef) -> Self {
        unsafe { Self(core::LLVMConstPointerNull(ty.get_ref())) }
    }
}
