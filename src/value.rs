use super::module::ModuleRef;
use super::types::TypeRef;
use crate::{CStr, CString, CUint, GetRef, SizeT};
use llvm_sys::core::{LLVMAddFunction, LLVMGetInlineAsmAsmString, LLVMGetParam, LLVMSetValueName2};
use llvm_sys::prelude::LLVMValueRef;
use std::ops::Deref;
use std::rc::Rc;

/// LLVM Value wrapper
pub struct ValueRef(LLVMValueRef);

impl ValueRef {
    /// Create Value form raw Value reference
    pub const fn create(value_ref: LLVMValueRef) -> Self {
        Self(value_ref)
    }

    /// Get raw value reference
    #[must_use]
    pub const fn get(&self) -> LLVMValueRef {
        self.0
    }

    /// Get function parameter by index
    #[must_use]
    pub fn get_func_param(func_value: &Rc<Self>, index: usize) -> Self {
        unsafe { Self(LLVMGetParam(***func_value, *CUint::from(index))) }
    }

    /// Set the string name of a value. By default, in LLVM values monotonic increased
    pub fn set_value_name2(&self, name: &str) {
        unsafe {
            let c_name = CString::from(name);
            LLVMSetValueName2(
                **self,
                c_name.as_ptr(),
                *SizeT::from(c_name.to_bytes().len()),
            );
        }
    }

    /// Get the template string used for an inline assembly snippet.
    #[must_use]
    pub fn get_inline_asm_asm_string(&self) -> Option<String> {
        unsafe {
            let mut length = SizeT::from(0_usize);
            let c_str = LLVMGetInlineAsmAsmString(self.0, &mut *length);
            if c_str.is_null() {
                None
            } else {
                Some(CStr::new(c_str).to_string())
            }
        }
    }
}

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
