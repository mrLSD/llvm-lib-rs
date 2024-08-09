//! Functions in this section work on all `ValueRef` instances,
//! regardless of their sub-type. They correspond to functions available
//! on `C llvm::Value` .

use super::ValueKind;
use crate::core::types::TypeRef;
use crate::{CStr, CString, GetRef, SizeT};
use llvm_sys::core;
use llvm_sys::prelude::LLVMValueRef;
use std::ops::Deref;

/// LLVM Value wrapper
#[derive(Debug)]
pub struct ValueRef(LLVMValueRef);

impl Deref for ValueRef {
    type Target = LLVMValueRef;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl GetRef for ValueRef {
    type RawRef = LLVMValueRef;
    fn get_ref(&self) -> Self::RawRef {
        self.0
    }
}

impl From<LLVMValueRef> for ValueRef {
    fn from(value_ref: LLVMValueRef) -> Self {
        Self(value_ref)
    }
}

impl ValueRef {
    /// Obtain the type of a value.
    #[must_use]
    pub fn type_of(&self) -> TypeRef {
        unsafe { TypeRef::from(core::LLVMTypeOf(self.0)) }
    }

    /// Returns the kind of the given LLVM value (Obtain the enumerated type of a Value instance.).
    #[must_use]
    pub fn get_value_kind(&self) -> ValueKind {
        unsafe { ValueKind::from(core::LLVMGetValueKind(self.0)) }
    }

    /// Obtain the string name of a value.
    #[must_use]
    pub fn get_value_name(&self) -> Option<String> {
        unsafe {
            let mut length = SizeT::from(0);
            let c_str = core::LLVMGetValueName2(self.0, &mut *length);
            if c_str.is_null() {
                return None;
            }
            Some(CStr::new(c_str).to_string())
        }
    }

    /// Set the string name of a value.
    pub fn set_value_name(&self, name: &str) {
        let c_string = CString::from(name);
        unsafe {
            core::LLVMSetValueName2(self.0, c_string.as_ptr(), *SizeT::from(name.len()));
        }
    }

    /// Dump a representation of a value to stderr.
    pub fn dump_value(&self) {
        unsafe { core::LLVMDumpValue(self.0) }
    }

    /// Return a string representation of the value. Use
    /// `dispose_message` to free the string.
    #[must_use]
    pub fn print_value_to_string(&self) -> Option<String> {
        unsafe {
            let c_str = core::LLVMPrintValueToString(self.0);
            if c_str.is_null() {
                return None;
            }
            let result = CStr::new(c_str).to_string();
            crate::core::dispose_message(c_str);
            Some(result)
        }
    }

    /// Replace all uses of a value with another one.
    pub fn replace_all_uses_with(&self, new_val: &Self) {
        unsafe { core::LLVMReplaceAllUsesWith(self.0, new_val.0) }
    }

    /// Determines whether the specified value instance is constant.
    #[must_use]
    pub fn is_constant(&self) -> bool {
        unsafe { core::LLVMIsConstant(self.0) != 0 }
    }

    /// Determine whether a value instance is undefined.
    #[must_use]
    pub fn is_undef(&self) -> bool {
        unsafe { core::LLVMIsUndef(self.0) != 0 }
    }

    /// Determine whether a value instance is poisonous.
    #[must_use]
    pub fn is_poison(&self) -> bool {
        unsafe { core::LLVMIsPoison(self.0) != 0 }
    }

    /// Determines whether the specified value instance is an `AMD` node.
    #[must_use]
    pub fn is_amd_node(&self) -> Self {
        unsafe { Self(core::LLVMIsAMDNode(self.0)) }
    }

    /// Determines whether the specified value instance is a value as metadata.
    #[must_use]
    pub fn is_value_as_metadata(&self) -> Self {
        unsafe { Self(core::LLVMIsAValueAsMetadata(self.0)) }
    }

    /// Determines whether the specified value instance is an `AMD` string.
    #[must_use]
    pub fn is_amd_string(&self) -> Self {
        unsafe { Self(core::LLVMIsAMDString(self.0)) }
    }
}
